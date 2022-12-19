use std::collections::{HashMap};

use crate::ast;
use super::asm;

struct RegisterAllocator {
    regs: Vec<bool>,
    total_allocs: u32,
}

impl RegisterAllocator {
    pub fn new(max_regs: u32) -> Self {
        RegisterAllocator { 
            regs: vec![false; max_regs as usize],
            total_allocs: 0,
        }
    }

    fn mark_unavailable(&mut self, reg: usize) {
        self.regs[reg-1] = true;
        self.total_allocs += 1;
    }

    fn get_reg(&mut self) -> usize {
        for i in 0..self.regs.len() {
            if !self.regs[i] {
                self.regs[i] = true;
                self.total_allocs += 1;
                return i+1;
            }
        }
        return 0;
    }

    fn free_reg(&mut self, reg: usize) {
        self.regs[reg] = false;
        self.total_allocs -= 1;
    }
}

pub struct Allocator {
    mem: Vec<bool>,
    allocs: HashMap<u32, HashMap<String, usize>>,

    //lineno : registerallocator
    reg_allocs: HashMap<u32, RegisterAllocator>,

    //varname : memory address
    mem_allocs: HashMap<String, u32>,
    max_regvars: u32,
    max_regs: u32,
}

impl Allocator {
    pub fn new(max_regs: u32) -> Self {
        Allocator {
            mem: vec![false; 512],
            allocs: HashMap::new(),
            reg_allocs: HashMap::new(),
            mem_allocs: HashMap::new(),
            max_regvars: if max_regs > 2 {max_regs - 2} else {0},
            max_regs: max_regs,
        }
    }

    pub fn done_with_var(&mut self, asm: &mut asm::URCLAsm, lineno: u32, varname: &String, reg: usize) {
        match self.allocs.get(&lineno) {
            Some(varrecord) => {
                if let Some(var_reg) = varrecord.get(varname) {
                    return;
                }
                else if let Some(mem_alloc) = self.mem_allocs.get(varname) {
                    asm.put_store(*mem_alloc, reg);
                    if let Some(reg_alloc) = self.reg_allocs.get_mut(&lineno) {
                        reg_alloc.free_reg(reg);
                    }
                }
                else {
                    panic!("Could not find '{}' in varrecord", varname);
                }
            },
            None => {panic!("No allocations were created for lineno {}", lineno);},
        }
    }

    pub fn get_var(&mut self, lineno: u32, varname: &String) -> usize {
        match self.allocs.get(&lineno) {
            Some(varrecord) => {
                if let Some(var_reg) = varrecord.get(varname) {
                    return *var_reg;
                }
                else if let Some(mem_alloc) = self.mem_allocs.get(varname) {
                    return self.get_empty_reg(lineno);
                }
                else {
                    panic!("Could not find '{}' in varrecord", varname);
                }
            },
            None => {panic!("No allocations were created for lineno {}", lineno);},
        }
    }

    pub fn get_var_loaded(&mut self, asm: &mut asm::URCLAsm, lineno: u32, varname: &String) -> usize {
        match self.allocs.get(&lineno) {
            Some(varrecord) => {
                if let Some(var_reg) = varrecord.get(varname) {
                    return *var_reg;
                }
                else if let Some(mem_alloc) = self.mem_allocs.get(varname) {
                    let addr: u32 = *mem_alloc;
                    let free_reg = self.get_empty_reg(lineno);
                    asm.put_load(free_reg, addr);
                    return free_reg;
                }
                else {
                    panic!("Could not find memory allocation for '{}'", varname);
                }
            },
            None => {panic!("No allocations were created for lineno {}", lineno);}
        }
    }

    pub fn get_empty_reg(&mut self, lineno: u32) -> usize {
        if let Some(reg_alloc) = self.reg_allocs.get_mut(&lineno) {
            return self.reg_allocs.get_mut(&lineno).unwrap().get_reg();
        }
        else {
            let mut reg_alloc = RegisterAllocator::new(self.max_regs);
            let reg = reg_alloc.get_reg();
            self.reg_allocs.insert(lineno, reg_alloc);
            return reg;
        }
    }

    pub fn init(&mut self, ast_nodes: &[ast::Statement]) {
        let mut live_ranges: HashMap<String, Vec<u32>> = HashMap::new();
        self.gen_ranges(ast_nodes, &mut live_ranges);
        println!("{:?}", live_ranges);

        let mut new_liveranges: HashMap<String, Vec<u32>> = HashMap::new();

        for (varname, linenos) in live_ranges {
            for i in linenos[0]..=linenos[linenos.len()-1] {
                if let Some(vec_r) = new_liveranges.get_mut(&varname) {
                    vec_r.push(i);
                }
                else {
                    new_liveranges.insert(varname.clone(), vec![linenos[0]]);
                }
            }
        }

        let mut ranges : HashMap<u32, Vec<String>> = HashMap::new();
        for (varname, linenos) in new_liveranges {
            for i in linenos {
                if let Some(t) = ranges.get_mut(&i) {
                    t.push(varname.clone());
                }
                else {
                    ranges.insert(i, vec![varname.clone()]);
                }
            }
        }
        
        let mut least: u32 = 0;
        let mut max: u32 = 0;
        for (key, _) in &ranges {
            if least == 0 {
                least = *key;
            }
            if max == 0 {
                max = *key;
            }
            if *key < least {
                least = *key;
            }
            if *key > max {
                max = *key;
            }
        }

        for i in least..=max {
            if let Some(varlist) = ranges.get(&i) {

                let mut lineno_regalloc: &mut RegisterAllocator;
                match self.reg_allocs.get_mut(&i) {
                    Some(reg_allocator) => {
                        lineno_regalloc = reg_allocator;
                    }
                    None => {
                        self.reg_allocs.insert(i, RegisterAllocator::new(self.max_regs));
                        lineno_regalloc = self.reg_allocs.get_mut(&i).unwrap();
                    }
                }
                if i > 1 {
                    if let Some(alloc) = self.allocs.get(&(i-1)) {
                        let mut varrecord: HashMap<String, usize> = HashMap::new();
                        for (varname, reg) in alloc {
                            for currentvar in varlist {
                                if currentvar == varname {
                                    varrecord.insert(varname.clone(), *reg);
                                    lineno_regalloc.mark_unavailable(*reg);
                                }
                            }
                        }
                        self.allocs.insert(i, varrecord);
                    }
                }

                for varname in varlist {
                    
                    if let Some(varrecord) = self.allocs.get_mut(&i) {
                        match varrecord.get(varname) {
                            Some(_) => {
                            }
                            None => {
                                let var_mem = self.mem_allocs.get(varname);
                                if var_mem == None {
                                    //finds a free slot in memory and gives it to the variable
                                    self.mem_allocs.insert(varname.clone(), self.mem.iter().position(|x| !x).unwrap() as u32);
                                }
                                if lineno_regalloc.total_allocs < self.max_regvars {
                                    varrecord.insert(varname.clone(), lineno_regalloc.get_reg());
                                }
                            }
                        }
                    }
                    else {
                        let var_mem = self.mem_allocs.get(varname);
                        let mut varrecord: HashMap<String, usize> = HashMap::new();
                        if var_mem == None {
                            self.mem_allocs.insert(varname.clone(), self.mem.iter().position(|&x| x == false).unwrap() as u32);
                        }
                        if lineno_regalloc.total_allocs < self.max_regvars {
                            varrecord.insert(varname.clone(), lineno_regalloc.get_reg());
                            self.allocs.insert(i, varrecord);
                        }
                    }
                }
            }
        }

        for (lineno, varrecord) in &self.allocs {
            println!("Line: {}", lineno);
            for (varname, reg) in varrecord {
                println!("\t{}->R{}", varname, reg);
            }
        }


        // let alloc_lines: Vec<u32>;

        // for (varname, activelines) in ranges {
        //     let start_range = *activelines.get(0).unwrap_or(&0);
        //     let end_range = *activelines.get(activelines.len()-1).unwrap_or(&start_range);
        //     //ex: {a: [1, 2, 3]}
        //     //for i from line-1 to line-3
        //     for i in start_range..=end_range {
        //         //look for a register allocator on this line
        //         let mut lineno_regalloc: &mut RegisterAllocator;
        //         match self.reg_allocs.get_mut(&i) {
        //             Some(reg_allocator) => {
        //                 lineno_regalloc = reg_allocator;
        //             }
        //             None => {
        //                 self.reg_allocs.insert(i, RegisterAllocator::new(self.max_regs));
        //                 lineno_regalloc = self.reg_allocs.get_mut(&i).unwrap();
        //             }
        //         }
        //         if let Some(varrecord) = self.allocs.get_mut(&i) {
        //             match varrecord.get(&varname) {
        //                 Some(_) => {
        //                 }
        //                 None => {
        //                     let var_mem = self.mem_allocs.get(&varname);
        //                     if var_mem == None {
        //                         //finds a free slot in memory and gives it to the variable
        //                         self.mem_allocs.insert(varname.clone(), self.mem.iter().position(|x| !x).unwrap() as u32);
        //                     }
        //                     if lineno_regalloc.total_allocs < self.max_regvars {
        //                         varrecord.insert(varname.clone(), lineno_regalloc.get_reg());
        //                     }
        //                 }
        //             }
        //         }
        //         else {
        //             let var_mem = self.mem_allocs.get(&varname);
        //             let mut varrecord: LinkedHashMap<String, usize> = LinkedHashMap::new();
        //             if var_mem == None {
        //                 self.mem_allocs.insert(varname.clone(), self.mem.iter().position(|&x| x == false).unwrap() as u32);
        //             }
        //             if lineno_regalloc.total_allocs < self.max_regvars {
        //                 varrecord.insert(varname.clone(), lineno_regalloc.get_reg());
        //                 self.allocs.insert(i, varrecord);
        //             }
        //         }
        //     }
        // }

        // for (lineno, varrecord) in &self.allocs {
        //     println!("Line: {}", lineno);
        //     for (varname, reg) in varrecord {
        //         println!("\t{}->R{}", varname, reg);
        //     }
        // }
    }

    fn gen_ranges(&mut self, ast_nodes: &[ast::Statement], ranges: &mut HashMap<String, Vec<u32>>) {
        for node in ast_nodes {
            match node {
                ast::Statement::Declaration { span, vartype: _, target, expr } => {
                    self.put_range(&target.name, span.start().lineno, ranges);
                    self.gen_expr_ranges(expr, span.start().lineno, ranges);
                }
                ast::Statement::Assignment { span, target, expr } => {
                    match target {
                        ast::Expression::Identifier(identifier) => {
                            self.put_range(&identifier.name, span.start().lineno, ranges);
                            self.gen_expr_ranges(expr, span.start().lineno, ranges);
                        }
                        _ => {

                        }
                    }
                },
                ast::Statement::Body { content, span } => todo!(),
                ast::Statement::IfStatement { cond, body, child, span } => todo!(),
                ast::Statement::Expr { span, expr } => {self.gen_expr_ranges(expr, span.start().lineno, ranges)},
            }
        }
    }


    fn gen_expr_ranges(&mut self, expr: &ast::Expression, lineno: u32, ranges: &mut HashMap<String, Vec<u32>>) {
        match expr {
            ast::Expression::Identifier(identifier) => {
                self.put_range(&identifier.name, identifier.span.start().lineno, ranges);
            }
            ast::Expression::BinaryOp(_, expr1, _, expr2) => {
                self.gen_expr_ranges(&(*expr1), lineno, ranges);
                self.gen_expr_ranges(&(*expr2), lineno, ranges);
            }
            _ => {

            }
        }
    }

    #[inline]
    fn put_range(&self, name: &String, lineno: u32, ranges: &mut HashMap<String, Vec<u32>>) {
        if let Some(range_vec) = ranges.get_mut(name) {
            range_vec.push(lineno);
        }
        else {
            let range_vec: Vec<u32> = vec![lineno];
            ranges.insert(name.clone(), range_vec);
        }
    }
}