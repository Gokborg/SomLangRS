use std::{fs, io::Write, collections::HashMap};

use wasm_encoder::{
    CodeSection, ExportKind, ExportSection, Function, FunctionSection, Instruction,
    Module, TypeSection, ValType,
};

use crate::typechecking::{att::{self, Type}, scope::VarIndex};
use crate::ast;


pub struct WasmGen {}

struct Func {
    wasm: Function,
    indices: HashMap<VarIndex, u32>
}
impl Func {
    pub fn new(func: Function) -> Self {
        Self {wasm: func, indices: HashMap::new()}
    }
    pub fn index(&mut self, varid: &VarIndex, vartype: &Type) -> u32 {
        if let Some(&index) = self.indices.get(&varid) {
            return index;
        };
        let index = self.indices.len() as u32;
        self.indices.insert(*varid, index);
        return index as u32;
    }
}

impl WasmGen {
    pub fn gen(tree: &[att::TStat]) -> Vec<u8> {
        let mut gennerator = WasmGen {};
        
        let mut module = Module::new();

        // Encode the type section.
        let mut types = TypeSection::new();
        let params = vec![];
        let results = vec![];
        types.function(params, results);
        module.section(&types);

        // Encode the function section.
        let mut functions = FunctionSection::new();
        let type_index = 0;
        functions.function(type_index);
        module.section(&functions);

        // Encode the export section.
        let mut exports = ExportSection::new();
        exports.export("f", ExportKind::Func, 0);
        module.section(&exports);

        // Encode the code section.
        let mut codes = CodeSection::new();
        let locals = vec![(10, ValType::I32)];
        
        
        
        let mut f = Func::new(Function::new(locals));
        gennerator.gen_stmts(&mut f, tree);
        f.wasm.instruction(&Instruction::End);
        codes.function(&f.wasm);
        module.section(&codes);

        // let mut binary_file = fs::File::create("somoutput/test.wasm").unwrap();
        // Extract the encoded Wasm bytes for this module.
        module.finish()
        // binary_file.write_all(&wasm_bytes).unwrap();

        // let mut text_file = fs::File::create("somoutput/test.wat").unwrap();
        // let text = wasmprinter::print_bytes(wasm_bytes).unwrap();
        // write!(&mut text_file, "{}", text).unwrap();
    }
    fn gen_stmts(&mut self, func: &mut Func, tree: &[att::TStat]) {
        for node in tree {
            match node {
                att::TStat::Decl { span, varid, vartype, expr } => {
                    if let Some(expr) = expr {
                        let index = func.index(varid, vartype);
                        self.gen_expr(func, expr);
                        func.wasm.instruction(&Instruction::LocalSet(index));
                    }
                },
                att::TStat::Assignment { span, target, expr } => todo!(),
                att::TStat::Expr { span, expr } => todo!(),
                att::TStat::Body { span, content } => todo!(),
                att::TStat::IfStatement { span, cond, body, child } => todo!(),
            }
        }
    }

    fn gen_expr(&mut self, func: &mut Func, expr: &att::TExpr) {
        match expr {
            att::TExpr::Uint { span, value } => {
                func.wasm.instruction(&Instruction::I32Const(*value as i32));
            },
            att::TExpr::Var { span, varid, vartype } => {
                let index = func.index(varid, vartype);
                func.wasm.instruction(&Instruction::LocalGet(index));
            }
            att::TExpr::BinaryOp { span, left, op, right, vartype } => {
                self.gen_expr(func, left);
                self.gen_expr(func, right);
                match (left.vartype(), op, right.vartype()) {
                    (Type::UInt, ast::Op::Add(_), Type::UInt) => {func.wasm.instruction(&Instruction::I32Add);}
                    _ => todo!()
                }
            },
        }
    }
}