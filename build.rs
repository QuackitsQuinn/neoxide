const HEADER: &str = r#"

use crate::{addressing::AddressingMode, cpu::CPU, ops::op::{Operation, nop}};

"#;

struct Op {
    code: u8,
    cycles: u8,
    page_cross_incr: u8,
    addressing_mode: &'static str, // enum is in project but cant be used from build script
}
struct JsonOp {
    name: String,
    doc: String,
    ops: Vec<Op>,
}

impl JsonOp {
    fn new(name: &str, doc: &str, ops: Vec<Op>) -> Self {
        Self {
            name: name.to_owned(),
            doc: doc.to_owned(),
            ops,
        }
    }
    /*
    
    pub mod INST {
        lazy_static! {
            ops ...
        }
    }
    
    
     */
    fn to_code(&self) -> String {
        let mut code = String::new();
        code.push_str(&format!("/// {}\n", self.doc));
        code.push_str(&format!("pub mod {} {{\n", self.name.to_uppercase()));
        code.push_str("use crate::{addressing::AddressingMode, cpu::CPU, ops::op::{Operation, nop}};\n");
        code.push_str("lazy_static! {\n");
        for op in &self.ops {
            code.push_str(&format!("pub static ref {}: Operation = Operation::new(\"{}\", {}, nop, {}, {}, AddressingMode::{});\n", op.addressing_mode.to_uppercase(), op.addressing_mode, op.code, op.cycles, op.page_cross_incr, op.addressing_mode));
        }
        code.push_str("}}\n\n");

        code
    }
}
// generates the code for the nes opcodes
fn main() {

    // return if opcodes.rs exists
    if std::path::Path::new("src/ops/opcodes.rs").exists() {
       // return; // uncomment this to prevent overwriting opcodes.rs
    }
    //let mut codefile = std::fs::File::create("src/ops/opcodes.rs").unwrap();
    let mut code = String::from(HEADER);
    let ops = json::parse(include_str!("res/6502_ops.json")).unwrap();
    for op in ops.members() {
            let opname = op["instruction"].as_str().unwrap();
            let opname = opname.to_lowercase();
            let doc = "/// ".to_owned() + op["description"].as_str().unwrap();
        
    }
    println!("{}", JsonOp::new("TST", "test op", vec![Op {code: 0x00, cycles: 0, page_cross_incr: 0, addressing_mode: "Implied"}]).to_code())
}