use std::cell::RefCell;
use std::rc::Rc;
use std::collections::VecDeque;

#[derive(Debug, Clone)]
pub enum ASTNode {
    Number(String),
    Identifier(String),
    Multiply(Rc<RefCell<ASTNode>>, Rc<RefCell<ASTNode>>),
    Add(Rc<RefCell<ASTNode>>, Rc<RefCell<ASTNode>>),
    Sub(Rc<RefCell<ASTNode>>, Rc<RefCell<ASTNode>>),
    
}

// START *********: Control Flow Extension :********* ///

#[derive(Debug, Clone)]
pub enum BooleanExpression {

    True, False,

    CompareLessThan(Rc<RefCell<ASTNode>>, Rc<RefCell<ASTNode>>),
    CompareLessThanEqualTo(Rc<RefCell<ASTNode>>, Rc<RefCell<ASTNode>>),
    CompareGreaterThan(Rc<RefCell<ASTNode>>, Rc<RefCell<ASTNode>>),
    CompareGreaterThanEqualTo(Rc<RefCell<ASTNode>>, Rc<RefCell<ASTNode>>),
    CompareEqualTo(Rc<RefCell<ASTNode>>, Rc<RefCell<ASTNode>>),
}


#[derive(Debug, Clone)]
pub enum Statement{
    Assign { 
        name: String, expression: Rc<RefCell<ASTNode>>
    },

    If {
        condition: BooleanExpression, then_statements: Vec<Statement>, else_statements: Vec<Statement>
    },

    While {
        condition: BooleanExpression, body: Vec<Statement>
    },
}


#[derive(Debug, Clone)]
pub struct Program {
    
    pub arg_declarations: Vec<String>,
    pub var_declarations: Vec<String>,
    pub statements: Vec<Statement>,
    pub return_ident: String,

}


// ENDS *********: Control Flow Extension :********* ///

pub fn print_level_order(tree_root: &Rc<RefCell<ASTNode>>) {
    // A queue for BFS traversal
    use ASTNode::*;

    let mut traversal_queue: VecDeque<Rc<RefCell<ASTNode>>> = VecDeque::new();
    traversal_queue.push_back(tree_root.clone());

    // Continue until there are no more nodes to process
    while !traversal_queue.is_empty() {
        // Collect the nodes for the next level
        let mut next_level_nodes: Vec<Rc<RefCell<ASTNode>>> = Vec::new();

        // Process every node in the current level
        for _ in 0..traversal_queue.len() {
            let current = traversal_queue.pop_front().unwrap();
            match &*current.borrow() {
                Add(left, right) => {
                    print!("+ ");
                    next_level_nodes.push(left.clone());
                    next_level_nodes.push(right.clone());
                }
                Multiply(left, right) => {
                    print!("* ");
                    next_level_nodes.push(left.clone());
                    next_level_nodes.push(right.clone());
                }
                Sub(left, right) => {            
                    print!("- ");
                    next_level_nodes.push(left.clone());
                    next_level_nodes.push(right.clone());
                }

                Number(value) => print!("{} ", value),
                Identifier(name) => print!("{} ", name),
            }
        }

        // Print a newline between each level for readability
        println!();

        // Push the collected child nodes back into the queue
        for child in next_level_nodes {
            traversal_queue.push_back(child);
        }
    }
}
