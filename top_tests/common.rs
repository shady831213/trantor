use std::cmp::min;
use std::ops::Deref;
use std::path::Path;
use std::rc::Rc;
use terminus_spaceport::devices::term_exit;
use terminus_spaceport::memory::region::{BytesAccess, Region, U64Access, GHEAP};
use terminus_spaceport::memory::MemInfo;
use terminus_spaceport::space::Space;
use terminus_spaceport::{space, EXIT_CTRL};
use trantor::devices::bus::Bus;
use trantor::devices::htif::HTIF;
pub use trantor::global::XLen;
use trantor::processor::{Processor, ProcessorCfg};
use trantor::system::elf::ElfLoader;

pub struct RsicvTestRunner {
    debug: bool,
    tests_cnt: usize,
    name: Option<String>,
    timer: std::time::Instant,
}

impl RsicvTestRunner {
    pub fn new() -> RsicvTestRunner {
        let mut args = std::env::args();
        let mut debug = false;
        let mut name: Option<String> = None;
        let mut arg: Option<String> = args.next();
        while let Some(a) = &arg {
            if *a == "-d".to_string() {
                debug = true
            }
            if *a == "-r".to_string() {
                name = args.next()
            }
            arg = args.next()
        }
        RsicvTestRunner {
            debug,
            tests_cnt: 0,
            name,
            timer: std::time::Instant::now(),
        }
    }

    pub fn test_mp(&mut self, xlen: XLen, name: &str, num_cores: usize) {
        let valid = {
            if let Some(test_name) = &self.name {
                test_name == name
            } else {
                true
            }
        };
        if valid {
            if !riscv_test(xlen, name, self.debug, num_cores) {
                term_exit();
                assert!(false, format!("{} fail!", name))
            }
            self.tests_cnt += 1;
            println!("{}", format!("{} pass!", name));
        }
    }

    pub fn test(&mut self, xlen: XLen, name: &str) {
        self.test_mp(xlen, name, 1)
    }
}

impl Drop for RsicvTestRunner {
    fn drop(&mut self) {
        term_exit();
        println!(
            "{} tests Pass in {} micro seconds!",
            self.tests_cnt,
            self.timer.elapsed().as_micros()
        )
    }
}

fn riscv_test(xlen: XLen, name: &str, debug: bool, num_cores: usize) -> bool {
    EXIT_CTRL.reset();
    //you may modify ProcessorCfg implementation here
    let configs = vec![ProcessorCfg {}; num_cores];

    //create buss
    let bus: Rc<Bus> = Rc::new(Bus::new());
    //all processors
    let mut processors: Vec<Processor> = vec![];

    //read elf file and add htif device
    let elf = ElfLoader::new(
        Path::new("top_tests/elf")
            .join(Path::new(name))
            .to_str()
            .expect(&format!("{} not existed!", name)),
    )
    .unwrap();
    if let Some((base, tohost, fromhost)) = elf.htif_section().expect("Invalid ELF!") {
        let htif = Region::io(0, 0x1000, Box::new(HTIF::new(tohost, fromhost, false)));
        register_region(&bus, "htif", base, &htif).unwrap();
    }

    //init all cores
    for cfg in configs {
        let p = Processor::new(processors.len(), cfg, &bus, None, None);
        processors.push(p)
    }

    //add main memory
    register_memory(
        &bus,
        "main_memory",
        0x80000000,
        &GHEAP.alloc(0x10000000, 1).expect("main_memory alloc fail!"),
    )
    .unwrap();

    //load program into memory
    load_elf(&bus, &elf).unwrap();

    //reset processors
    let entry_point = elf.entry_point().unwrap();
    for p in &mut processors {
        p.reset(entry_point).unwrap();
    }

    //main loop
    loop {
        if let Ok(msg) = EXIT_CTRL.poll() {
            if debug {
                println!("{}", msg)
            }
            break;
        }
        for p in &mut processors {
            p.step(1);
            if debug {
                println!("{}", p.trace())
            }
        }
    }
    if debug {
        for p in &mut processors {
            println!("{}", p.dump_state())
        }
    }
    let htif = bus.space().get_region("htif").unwrap();
    U64Access::read(htif.deref(), &htif.info.base) == 0x1
}

fn register_region(
    bus: &Rc<Bus>,
    name: &str,
    base: u64,
    region: &Rc<Region>,
) -> std::result::Result<(), space::Error> {
    bus.space_mut()
        .add_region(name, &Region::remap(base, &region))?;
    Ok(())
}

fn register_memory(
    bus: &Rc<Bus>,
    name: &str,
    base: u64,
    mem: &Rc<Region>,
) -> std::result::Result<(), space::Error> {
    match register_region(bus, name, base, &mem) {
        Ok(_) => Ok(()),
        Err(e) => {
            if let space::Error::Overlap(n, msg) = e {
                if n == "htif".to_string() {
                    let htif_region = bus.space().get_region(&n).unwrap();
                    let range0 = if base < htif_region.info.base {
                        Some(MemInfo {
                            base: base,
                            size: htif_region.info.base - base,
                        })
                    } else {
                        None
                    };
                    let range1 =
                        if base + mem.info.size > htif_region.info.base + htif_region.info.size {
                            Some(MemInfo {
                                base: htif_region.info.base + htif_region.info.size,
                                size: base + mem.info.size
                                    - (htif_region.info.base + htif_region.info.size),
                            })
                        } else {
                            None
                        };
                    range0.iter().for_each(|info| {
                        bus.space_mut()
                            .add_region(name, &Region::remap_partial(info.base, mem, 0, info.size))
                            .unwrap();
                    });
                    range1.iter().for_each(|info| {
                        bus.space_mut()
                            .add_region(
                                &format!("{}_1", name),
                                &Region::remap_partial(info.base, mem, info.base - base, info.size),
                            )
                            .unwrap();
                    });
                    Ok(())
                } else {
                    Err(space::Error::Overlap(n, msg))
                }
            } else {
                Err(e)
            }
        }
    }
}

fn load_elf(bus: &Rc<Bus>, elf: &ElfLoader) -> std::result::Result<(), String> {
    elf.load(|addr, data| {
        fn load(space: &Space, addr: u64, data: &[u8]) -> std::result::Result<(), String> {
            if data.is_empty() {
                Ok(())
            } else {
                let region = space.get_region_by_addr(&addr).expect("not enough memory!");
                let len = min(
                    (region.info.base + region.info.size - addr) as usize,
                    data.len(),
                );
                let (head, tails) = data.split_at(len);
                BytesAccess::write(region.deref(), &addr, head).unwrap();
                load(space, region.info.base + region.info.size, tails)
            }
        };
        load(bus.space().deref(), addr, data)
    })?;
    Ok(())
}
