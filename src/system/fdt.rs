use std::fmt::{Display, Formatter};
use std::fmt;
use std::collections::HashMap;
const FDT_MAGIC: u32 = 0xd00dfeed;
const FDT_VERSION: u32 = 17;

const FDT_BEGIN_NODE: u32 = 1;
const FDT_END_NODE: u32 = 2;
const FDT_PROP: u32 = 3;
#[allow(dead_code)]
const FDT_NOP: u32 = 4;
const FDT_END: u32 = 9;

pub fn compile(root: &FdtNode) -> Vec<u8> {
    let mut state = FdtState::new();
    state.compile(root)
}

#[allow(dead_code)]
struct FdtHeader {
    magic: u32,
    total_size: u32,
    off_dt_struct: u32,
    off_dt_strings: u32,
    off_mem_rsvmap: u32,
    version: u32,
    last_comp_version: u32,
    boot_cpuid_phys: u32,
    dt_strings_size: u32,
    dt_struct_size: u32,
}

struct FdtState {
    string_table: HashMap<String, u32>,
    string_buffer: Vec<u8>,
    struct_buffer: Vec<u8>,
}

impl FdtState {
    fn new() -> FdtState {
        FdtState {
            string_table: HashMap::new(),
            string_buffer: vec![],
            struct_buffer: vec![],
        }
    }

    fn get_string_offset(&mut self, v: &str) -> u32 {
        if let Some(off) = self.string_table.get(v) {
            *off
        } else {
            let off = self.string_buffer.len() as u32;
            self.string_table.insert(v.to_string(), off);
            self.string_buffer.append(&mut v.as_bytes().to_vec());
            self.string_buffer.push(0);
            off
        }
    }

    fn compile(&mut self, root: &FdtNode) -> Vec<u8> {
        root.pack(self);
        self.struct_buffer.append(&mut FDT_END.to_be_bytes().to_vec());
        let mut header = FdtHeader {
            magic: FDT_MAGIC.to_be(),
            total_size: 0,
            off_dt_struct: 0,
            off_dt_strings: 0,
            off_mem_rsvmap: 0,
            version: FDT_VERSION.to_be(),
            last_comp_version: (FDT_VERSION - 1).to_be(),
            boot_cpuid_phys: 0,
            dt_strings_size: (self.string_buffer.len() as u32).to_be(),
            dt_struct_size: (self.struct_buffer.len() as u32).to_be(),
        };
        let mut pos = std::mem::size_of::<FdtHeader>() as u32;
        let off_dt_struct = pos;
        header.off_dt_struct = off_dt_struct.to_be();
        pos += self.struct_buffer.len() as u32;
        while pos & 0x7 != 0 {
            pos += 1
        }
        let off_mem_rsvmap = pos;
        header.off_mem_rsvmap = off_mem_rsvmap.to_be();
        let re = FdtRsvEntry {
            address: 0,
            size: 0,
        };
        pos += std::mem::size_of::<FdtRsvEntry>() as u32;
        let off_dt_strings= pos;
        header.off_dt_strings = off_dt_strings.to_be();
        pos += self.string_buffer.len() as u32;
        while pos & 0x7 != 0 {
            pos += 1
        };
        header.total_size = pos.to_be();
        let mut res: Vec<u8> = vec![0; pos as usize];
        res[0..std::mem::size_of::<FdtHeader>()].copy_from_slice(unsafe { std::slice::from_raw_parts((&header as *const FdtHeader) as *const u8, std::mem::size_of::<FdtHeader>()) });
        res[off_dt_struct as usize.. (off_dt_struct as usize) + self.struct_buffer.len()].copy_from_slice(&self.struct_buffer);
        res[off_mem_rsvmap as usize..(off_mem_rsvmap as usize) + std::mem::size_of::<FdtRsvEntry>()].copy_from_slice(unsafe { std::slice::from_raw_parts((&re as *const FdtRsvEntry) as *const u8, std::mem::size_of::<FdtRsvEntry>()) });
        res[off_dt_strings as usize..(off_dt_strings as usize) + self.string_buffer.len()].copy_from_slice(&self.string_buffer);
        res
    }
}

#[allow(dead_code)]
struct FdtRsvEntry {
    address: u64,
    size: u64,
}

enum FdtPropValue {
    Null,
    Str(Vec<String>),
    U32(Vec<u32>),
}

impl FdtPropValue {
    fn pack(&self) -> Vec<u8> {
        match self {
            FdtPropValue::Null => vec![],
            FdtPropValue::Str(value) => {
                let mut res = vec![];
                for s in value {
                    res.append(&mut s.as_bytes().to_vec());
                    res.push(0);
                }
                if res.len() & 0x3 != 0 {
                    let padding_len = 4 - (res.len() & 0x3);
                    for _ in 0..padding_len {
                        res.push(0)
                    }
                }
                res
            }
            FdtPropValue::U32(value) => {
                let mut res: Vec<u8> = vec![];
                for v in value {
                    res.append(&mut v.to_be_bytes().to_vec());
                }
                res
            }
        }
    }
}

impl Display for FdtPropValue {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            FdtPropValue::Null => write!(f, ""),
            FdtPropValue::Str(values) => write!(f, "= {}", values.iter().map(|s| { format!("\"{}\"", s) }).collect::<Vec<String>>().join(",")),
            FdtPropValue::U32(values) => {
                write!(f, "= <")?;
                write!(f, "{}", values.iter().map(|v| { format!("{:#x}", v) }).collect::<Vec<String>>().join(" "))?;
                write!(f, ">")
            }
        }
    }
}

pub struct FdtProp {
    indent: usize,
    name: String,
    value: FdtPropValue,
}

impl FdtProp {
    pub fn null_prop(name: &str) -> FdtProp {
        FdtProp {
            indent: 0,
            name: name.to_string(),
            value: FdtPropValue::Null,
        }
    }
    pub fn str_prop(name: &str, value: Vec<&str>) -> FdtProp {
        FdtProp {
            indent: 0,
            name: name.to_string(),
            value: FdtPropValue::Str(value.iter().map(|s| { s.to_string() }).collect()),
        }
    }

    pub fn u32_prop(name: &str, value: Vec<u32>) -> FdtProp {
        FdtProp {
            indent: 0,
            name: name.to_string(),
            value: FdtPropValue::U32(value),
        }
    }

    pub fn u64_prop(name: &str, value: Vec<u64>) -> FdtProp {
        let mut value_u32 = vec![];
        for v in value {
            value_u32.push((v >> 32) as u32);
            value_u32.push(v as u32);
        }
        FdtProp {
            indent: 0,
            name: name.to_string(),
            value: FdtPropValue::U32(value_u32),
        }
    }

    fn pack(&self, state: &mut FdtState) {
        state.struct_buffer.append(&mut FDT_PROP.to_be_bytes().to_vec());
        let mut data = self.value.pack();
        state.struct_buffer.append(&mut (data.len() as u32).to_be_bytes().to_vec());
        let mut offset = state.get_string_offset(&self.name).to_be_bytes().to_vec();
        state.struct_buffer.append(&mut offset);
        state.struct_buffer.append(&mut data);
    }
}

impl Display for FdtProp {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        writeln!(f, "{:indent$}{} {};", "", self.name, self.value.to_string(), indent = self.indent * 4)
    }
}

pub struct FdtNode {
    indent: usize,
    name: String,
    props: Vec<FdtProp>,
    nodes: Vec<Box<FdtNode>>,
}

impl FdtNode {
    pub fn new(name: &str) -> FdtNode {
        FdtNode {
            indent: 0,
            name: name.to_string(),
            props: vec![],
            nodes: vec![],
        }
    }

    pub fn new_with_num(name: &str, num: u64) -> FdtNode {
        Self::new(&format!("{}@{}", name, num))
    }

    fn upate_indent(&mut self, indent: usize) {
        self.indent = indent + 1;
        for prop in self.props.iter_mut() {
            prop.indent = self.indent + 1;
        }
        for node in self.nodes.iter_mut() {
            node.upate_indent(self.indent)
        }
    }

    pub fn add_prop(&mut self, mut prop: FdtProp) {
        prop.indent = self.indent + 1;
        self.props.push(prop)
    }

    pub fn add_node(&mut self, mut node: FdtNode) {
        node.upate_indent(self.indent);
        self.nodes.push(Box::new(node))
    }

    fn pack_name(&self) -> Vec<u8> {
        let mut res = self.name.as_bytes().to_vec();
        res.push(0);
        if res.len() & 0x3 != 0 {
            let padding_len = 4 - (res.len() & 0x3);
            for _ in 0..padding_len {
                res.push(0);
            }
        }
        res
    }

    fn pack(&self, state: &mut FdtState) {
        state.struct_buffer.append(&mut FDT_BEGIN_NODE.to_be_bytes().to_vec());
        state.struct_buffer.append(&mut self.pack_name());
        for prop in self.props.iter() {
            prop.pack(state)
        }
        for node in self.nodes.iter() {
            node.pack(state)
        }
        state.struct_buffer.append(&mut FDT_END_NODE.to_be_bytes().to_vec());
    }
}

impl Display for FdtNode {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        writeln!(f, "{:indent$}{} {{", "", self.name, indent = self.indent * 4)?;
        for prop in self.props.iter() {
            write!(f, "{}", prop.to_string())?
        }
        for node in self.nodes.iter() {
            write!(f, "{}", node.to_string())?
        }
        writeln!(f, "{:indent$}}};", "", indent = self.indent * 4)?;
        Ok(())
    }
}
