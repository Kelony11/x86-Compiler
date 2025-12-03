use crate::ast::{ASTNode, BooleanExpression, Program, Statement};
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

pub struct X86Writer {
    pub out: String,
    lbl: usize,
    offsets: HashMap<String, i64>,
}

impl X86Writer {
    pub fn new() -> Self {
        Self {
            out: String::new(),
            lbl: 0,
            offsets: HashMap::new(),
        }
    }

    fn fresh_lbl(&mut self, base: &str) -> String {
        self.lbl += 1;
        format!("{}.{}", base, self.lbl)
    }

    fn slot(&self, var: &str) -> i64 {
        *self.offsets.get(var).expect("unknown variable in slot()")
    }
}

// ===============================================================
// Top-level: full program → x86-64
// ===============================================================
pub fn generate_program_x86(p: &Program) -> String {
    let mut w = X86Writer::new();

    // 1) Assign stack slots: args + vars, 8 bytes each

   
    let mut offset: i64 = -8;

    for a in &p.arg_declarations {
        w.offsets.insert(a.clone(), offset);
        offset -= 8;
    }
    for v in &p.var_declarations {
        w.offsets.insert(v.clone(), offset);
        offset -= 8;
    }


    let last_offset = offset + 8;
    let mut frame_size = -last_offset; // positive number of bytes

  
    if frame_size % 16 != 0 {
        frame_size += 8;
    }

    // 2) Prologue
    w.out.push_str(".text\n.global foo\nfoo:\n");
    w.out.push_str("pushq %rbp\n");
    w.out.push_str("movq %rsp, %rbp\n");

    if frame_size > 0 {
        w.out
            .push_str(&format!("subq ${}, %rsp\n", frame_size));
    }

    // 3) Store incoming args into their stack slots
    let arg_regs = ["%rdi", "%rsi", "%rdx", "%rcx", "%r8", "%r9"];
    for (i, a) in p.arg_declarations.iter().enumerate() {
        let off = w.slot(a);
        w.out
            .push_str(&format!("movq {}, {}(%rbp)\n", arg_regs[i], off));
    }

    // 4) Emit statements
    for s in &p.statements {
        emit_stmt(s, &mut w);
    }

    // 5) Load return value into %rax
    let roff = w.slot(&p.return_ident);
    w.out
        .push_str(&format!("movq {}(%rbp), %rax\n", roff));

    // 6) Epilogue
    if frame_size > 0 {
        w.out
            .push_str(&format!("addq ${}, %rsp\n", frame_size));
    }
    w.out.push_str("popq %rbp\n");
    w.out.push_str("ret\n");

    w.out
}

// ===============================================================
// Statements
// ===============================================================
fn emit_stmt(s: &Statement, w: &mut X86Writer) {
    match s {
        Statement::Assign { name, expression } => {
            emit_expr(expression, w); 
            let off = w.slot(name);
            w.out
                .push_str(&format!("movq %rax, {}(%rbp)\n", off));
        }

        Statement::If {
            condition,
            then_statements,
            else_statements,
        } => {
            let l_then = w.fresh_lbl("if.then");
            let l_else = w.fresh_lbl("if.else");
            let l_end = w.fresh_lbl("if.end");

            emit_bool(condition, &l_then, &l_else, w);

            w.out.push_str(&format!("{}:\n", l_then));
            for s in then_statements {
                emit_stmt(s, w);
            }
            w.out.push_str(&format!("jmp {}\n", l_end));

            w.out.push_str(&format!("{}:\n", l_else));
            for s in else_statements {
                emit_stmt(s, w);
            }
            w.out.push_str(&format!("jmp {}\n", l_end));

            w.out.push_str(&format!("{}:\n", l_end));
        }

        Statement::While { condition, body } => {
            let l_cond = w.fresh_lbl("while.cond");
            let l_body = w.fresh_lbl("while.body");
            let l_end = w.fresh_lbl("while.end");

            w.out.push_str(&format!("{}:\n", l_cond));
            emit_bool(condition, &l_body, &l_end, w);

            w.out.push_str(&format!("{}:\n", l_body));
            for s in body {
                emit_stmt(s, w);
            }
            w.out.push_str(&format!("jmp {}\n", l_cond));
            w.out.push_str(&format!("{}:\n", l_end));
        }
    }
}

// ===============================================================
// Expressions  (result always ends in %rax)
// ===============================================================
fn emit_expr(node: &Rc<RefCell<ASTNode>>, w: &mut X86Writer) {
    use ASTNode::*;

    match &*node.borrow() {
        Number(n) => {
            w.out.push_str(&format!("movq ${}, %rax\n", n));
        }

        Identifier(name) => {
            let off = w.slot(name);
            w.out.push_str(&format!("movq {}(%rbp), %rax\n", off));
        }

        Add(l, r) => {
            emit_expr(l, w);        
            w.out.push_str("pushq %rax\n");
            emit_expr(r, w);   
            w.out.push_str("popq %rcx\n");
            w.out.push_str("addq %rcx, %rax\n");
        }

        Multiply(l, r) => {
            emit_expr(l, w);     
            w.out.push_str("pushq %rax\n");
            emit_expr(r, w);      
            w.out.push_str("popq %rcx\n"); 
            w.out.push_str("imulq %rcx, %rax\n");
        }

        Sub(l, r) => {
            // result = left - right
            emit_expr(l, w);     
            w.out.push_str("pushq %rax\n");
            emit_expr(r, w);         
            w.out.push_str("popq %rcx\n"); // 
           
            w.out.push_str("subq %rax, %rcx\n");
            w.out.push_str("movq %rcx, %rax\n");
        }
    }
}

// ===============================================================
// Boolean expressions  (generate branches directly)
// ===============================================================
fn emit_bool(b: &BooleanExpression, l_true: &str, l_false: &str, w: &mut X86Writer) {
    match b {
        BooleanExpression::True => {
            w.out.push_str(&format!("jmp {}\n", l_true));
        }
        BooleanExpression::False => {
            w.out.push_str(&format!("jmp {}\n", l_false));
        }

        BooleanExpression::CompareLessThan(l, r) => {
            emit_cmp(l, r, "jl", l_true, l_false, w);
        }
        BooleanExpression::CompareLessThanEqualTo(l, r) => {
            emit_cmp(l, r, "jle", l_true, l_false, w);
        }
        BooleanExpression::CompareGreaterThan(l, r) => {
            emit_cmp(l, r, "jg", l_true, l_false, w);
        }
        BooleanExpression::CompareGreaterThanEqualTo(l, r) => {
            emit_cmp(l, r, "jge", l_true, l_false, w);
        }
        BooleanExpression::CompareEqualTo(l, r) => {
            emit_cmp(l, r, "je", l_true, l_false, w);
        }
    }
}

fn emit_cmp(
    l: &Rc<RefCell<ASTNode>>,
    r: &Rc<RefCell<ASTNode>>,
    jmp: &str,
    l_true: &str,
    l_false: &str,
    w: &mut X86Writer,
) {

    emit_expr(l, w);                 // %rax = left
    w.out.push_str("movq %rax, %rcx\n");
    emit_expr(r, w);                 // %rax = right
    w.out.push_str("cmpq %rax, %rcx\n");
    w.out.push_str(&format!("{} {}\n", jmp, l_true));
    w.out.push_str(&format!("jmp {}\n", l_false));
}
