use crate::ast::ASTNode;
use std::cell::RefCell;
use std::rc::Rc;

// System V AMD64 argument registers for integer args
const ARG_REGS: [&str; 6] = ["%rdi", "%rsi", "%rdx", "%rcx", "%r8", "%r9"];

pub fn generate_expr_code(ast: &Rc<RefCell<ASTNode>>, args: &[String]) -> String {
    let mut out = String::new();

    // Function prologue
    out.push_str(".text\n");
    out.push_str(".global foo\n");
    out.push_str("foo:\n");
    out.push_str("pushq %rbp\n");
    out.push_str("movq %rsp, %rbp\n");
    out.push_str("pushq %rbx\n"); // save callee-saved temp register

    // Compute the expression; result ends up in %rax
    emit_expr(ast, &mut out, args);

    // Epilogue
    out.push_str("popq %rbx\n");  
    out.push_str("movq %rbp, %rsp\n");
    out.push_str("popq %rbp\n");
    out.push_str("ret\n");

    out
}

// After emit_expr(node): %rax holds the value of `node`
// Stack pointer is unchanged (every push has a matching pop).
fn emit_expr(node: &Rc<RefCell<ASTNode>>, out: &mut String, args: &[String]) {
    match &*node.borrow() {
        ASTNode::Number(n) => {
            // Literal → move immediate into %rax
            out.push_str(&format!("movq ${}, %rax\n", n));
        }

        ASTNode::Identifier(name) => {
            // Map identifier to its argument register (first-seen order decided by caller)
            let idx = args
                .iter()
                .position(|a| a == name)
                .expect("identifier not found in arg list");
            out.push_str(&format!("movq {}, %rax\n", ARG_REGS[idx]));
        }

        ASTNode::Add(l, r) => {
            // left → %rax, save it
            emit_expr(l, out, args);
            out.push_str("pushq %rax\n");

            // right → %rax
            emit_expr(r, out, args);

            // restore left into %rbx and add
            out.push_str("popq %rbx\n");
            out.push_str("addq %rbx, %rax\n");
        }

        ASTNode::Multiply(l, r) => {
            // left → %rax, save it
            emit_expr(l, out, args);
            out.push_str("pushq %rax\n");

            // right → %rax
            emit_expr(r, out, args);

            // restore left into %rbx and multiply
            out.push_str("popq %rbx\n");
            out.push_str("imulq %rbx, %rax\n");
        }
    }
}