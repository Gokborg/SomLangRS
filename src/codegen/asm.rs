// pub trait Asm {
//     fn put_li(&mut self, dest: u32, num: u32);
//     fn put_add(&mut self, dest: u32, src_a: u32, src_b: u32);
//     fn put_sub(&mut self, dest: u32, src_a: u32, src_b: u32);
//     fn put_mlt(&mut self, dest: u32, src_a: u32, src_b: u32);
//     fn put_div(&mut self, dest: u32, src_a: u32, src_b: u32);
//     fn put_load(&mut self, dest: u32, addr: u32);
//     fn put_store(&mut self, addr: u32, src_a: u32);
// }

use core::fmt;
use std::{fs::File, path::Path};

pub struct URCLAsm {
    instrs: Vec<String>
}

impl fmt::Display for URCLAsm {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut result: String = String::new();
        for instr in &self.instrs {
            result += &instr;
            result.push('\n');
        }
        return write!(f, "{}", result);
    }
}

impl URCLAsm {
    pub fn new() -> Self {
        URCLAsm {
            instrs: Vec::new(),
        }
    }
    pub fn put_li(&mut self, dest: usize, num: u32) {
        self.instrs.push(
            format!("IMM R{} {}", dest, num)
        );
    }
    pub fn put_add(&mut self, dest: usize, src_a: usize, src_b: usize) {
        self.instrs.push(
            format!("ADD R{} R{} R{}", dest, src_a, src_b)
        );
    }
    pub fn put_sub(&mut self, dest: usize, src_a: usize, src_b: usize) {
        self.instrs.push(
            format!("SUB R{} R{} R{}", dest, src_a, src_b)
        );
    }
    pub fn put_mlt(&mut self, dest: usize, src_a: usize, src_b: usize) {
        self.instrs.push(
            format!("MLT R{} R{} R{}", dest, src_a, src_b)
        );
    }
    pub fn put_div(&mut self, dest: usize, src_a: usize, src_b: usize) {
        self.instrs.push(
            format!("DIV R{} R{} R{}", dest, src_a, src_b)
        );
    }
    pub fn put_load(&mut self, dest: usize, addr: u32) {
        self.instrs.push(
            format!("LOD R{} #{}", dest, addr)
        );
    }
    pub fn put_store(&mut self, addr: u32, dest: usize) {
        self.instrs.push(
            format!("STR #{} R{}", addr, dest)
        );
    }
    pub fn put_mov(&mut self, dest: usize, src_a: usize) {
        self.instrs.push(
            format!("MOV R{} R{}", dest, src_a)
        );
    }
}
/*
*/