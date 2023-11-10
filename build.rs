use std::{array, fmt::format, io::Write};

use json::JsonValue;

const HEADER: &str = r#"
#![cfg_attr(rustfmt, rustfmt_skip)]
// ===================================================================================================
//  This file is generated by build.rs. DO NOT EDIT
// ===================================================================================================
//  This file contains the opcodes for the 6502 CPU, extracted from the 6502.json file in the res folder.
//  The opcodes are stored in a static array called the optable, which is indexed by the opcode byte.
//  This is **SIGNIFICANTLY** faster than a match statement, and is the reason why the optable is used.
//  The optable is generated by the build script, which is how this file is generated.
//
//  lazy_static! thank you for existing

use crate::{addressing::AddressingMode, cpu::CPU, ops::{opcode::Operation,op::{nop,undoc_nop},load_ops::*,store_ops::*,reg_ops::*,arithmatic_ops::*,branch_ops::*,stack_ops::*,status_ops::*}};

"#;

const OPTABLE_HEADER: &str = r#"
/// This is the optable, which contains all of the opcodes for the 6502 CPU.
/// The order of the ops is **EXTREMELY** important, as the index is the opcode byte.
/// Any opcode marked as UNDOC_NOP is an undocumented opcode, and will be logged when executed, but will not do anything.
/// The optable is like a huge match statement, but is **SIGNIFICANTLY** faster because it is a static array.
"#;
#[derive(Debug, Clone)]
struct Op {
    code: u8,
    cycles: u8,
    page_cross_incr: u8,
    length: u8,
    addressing_mode: String, // enum is in project but cant be used from build script
    addressing_mode_const: String,
}

impl Op {
    fn new(
        code: u8,
        cycles: u8,
        page_cross_incr: u8,
        length: u8,
        addressing_mode: &str,
        addressing_mode_const: &str,
    ) -> Self {
        Self {
            code,
            cycles,
            page_cross_incr,
            length,
            addressing_mode: addressing_mode.to_owned(),
            addressing_mode_const: addressing_mode_const.to_owned(),
        }
    }
}

impl From<&JsonValue> for Op {
    fn from(value: &JsonValue) -> Self {
        // empty return because switching json file structure
        // convert CAP_CASE to PascalCase
        let mut mode = String::new();
        for word in value["addr_mode"].as_str().unwrap().split("_") {
            mode.push_str(&word[..1].to_uppercase());
            mode.push_str(&word[1..].to_lowercase());
        }
        Self {
            code: value["opcode"].as_u8().unwrap(),
            cycles: value["cycles"].as_u8().unwrap(),
            page_cross_incr: value["page_cross_incr"].as_u8().unwrap(),
            length: value["length"].as_u8().unwrap(),
            addressing_mode: mode,
            addressing_mode_const: value["addr_mode"].as_str().unwrap().to_owned(),
        }
    }
}
#[derive(Debug, Clone)]
struct JsonOp {
    name: String,
    doc: String,
    optype: String,
    ops: Vec<Op>,
}

impl JsonOp {
    fn new(name: &str, doc: &str, optype: &str, ops: Vec<Op>) -> Self {
        Self {
            name: name.to_owned(),
            doc: doc.to_owned(),
            optype: optype.to_owned(),
            ops,
        }
    }
    fn to_code(&self) -> String {
        let mut code = String::new();
        code.push_str(&format!("/// {}\n", self.doc));
        // disable snake case linting for this line
        code.push_str(&format!("#[allow(non_snake_case)]\n"));
        code.push_str(&format!("pub mod {} {{\n", self.name.to_uppercase()));
        code.push_str(" use super::*;\n\n");
        code.push_str(" lazy_static! {\n");
        for op in &self.ops {
            code.push_str(&format!("   pub static ref {}: Operation = Operation::new(\"{}\", \"{}\", {:#04X?}, {}, {}, {}, {}, AddressingMode::{});\n", op.addressing_mode_const, self.name, self.optype, op.code, self.name.to_lowercase(), op.cycles, op.page_cross_incr, op.length, op.addressing_mode));
        }
        code.push_str(" }\n}\n\n");
        code = code.replace(r"\n", "\n");
        code
    }
}
#[derive(Debug, Clone)]
struct OptableEntry {
    code: u8,
    op: String,
    mode: String,
}

impl OptableEntry {
    fn new(code: u8, op: &str, mode: &str) -> Self {
        Self {
            code,
            op: op.to_owned(),
            mode: mode.to_owned(),
        }
    }
}
impl From<&JsonValue> for JsonOp {
    fn from(value: &JsonValue) -> Self {
        let name = value["name"].as_str().unwrap();
        let doc = value["long_name"].as_str().unwrap();
        let optype = value["type"].as_str().unwrap();
        let mut ops: Vec<Op> = Vec::new();
        for mode in value["operands"].members() {
            let op: Op = mode.into();
            ops.push(op);
        }
        Self::new(name, doc, optype, ops)
    }
}
// generates the code for the nes opcodes
fn main() {
    // add homebrew sdl2 install
    // aka this is a weird solution to get neoxide to compile on a school lock-downed mac
    println!("cargo:rustc-link-search={}/homebrew/Cellar/sdl2/2.28.5/lib/", std::env::var("HOME").unwrap());
    // return if opcodes.rs exists
    if !std::path::Path::new("src/ops/opcodes.rs").exists() {
        return; // uncomment this to prevent overwriting opcodes.rs
    }
    let mut codefile = std::fs::File::create("src/ops/opcodes.rs").unwrap();
    let mut code = String::from(HEADER);
    let ops = json::parse(include_str!("res/6502.json")).unwrap();
    let mut code_ops: Vec<JsonOp> = Vec::new();
    for op in ops.members() {
        let op: JsonOp = op.into();
        code_ops.push(op);
    }

    for op in &code_ops {
        code.push_str(&op.to_code());
    }

    let undoc_op = JsonOp::new(
        "UNDOC_NOP",
        "Undocumented No-Op",
        "no-op",
        vec![Op::new(0xEA, 2, 0, 1, "Implied", "IMPLIED")],
    );
    code.push_str(&undoc_op.to_code());
    // build ender optable
    let mut optable: [OptableEntry; 256] =
        array_init::array_init(|i: usize| OptableEntry::new(i as u8, "UNDOC_NOP", "Implied"));

    for jop in code_ops {
        for op in jop.ops {
            optable[op.code as usize] =
                OptableEntry::new(op.code, &jop.name.to_uppercase(), &op.addressing_mode_const);
        }
    }
    // generate optable
    code.push_str("\n\n lazy_static! {\n");
    code.push_str(format!("\n {} \n", OPTABLE_HEADER).as_str());
    code.push_str("    pub static ref OPTABLE: [Operation; 256] = [\n");
    for op in optable {
        code.push_str(&format!(
            "        *{}::{},\n",
            op.op,
            op.mode.to_uppercase()
        ));
    }
    code.push_str("    ];\n}\n");
    codefile.write_all(code.as_bytes()).unwrap();
}
