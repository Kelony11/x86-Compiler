use std::cell::RefCell;
use std::rc::Rc;
use std::collections::VecDeque;

#[derive(Debug, Clone)]
pub enum ASTNode {
    Number(String),
    Identifier(String),
    Multiply(Rc<RefCell<ASTNode>>, Rc<RefCell<ASTNode>>),
    Add(Rc<RefCell<ASTNode>>, Rc<RefCell<ASTNode>>),

    
}

pub fn print_level_order(tree_root: &Rc<RefCell<ASTNode>>) {
    // A queue for BFS traversal
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
                ASTNode::Add(left, right) => {
                    print!("+ ");
                    next_level_nodes.push(left.clone());
                    next_level_nodes.push(right.clone());
                }
                ASTNode::Multiply(left, right) => {
                    print!("* ");
                    next_level_nodes.push(left.clone());
                    next_level_nodes.push(right.clone());
                }
                ASTNode::Number(value) => print!("{} ", value),
                ASTNode::Identifier(name) => print!("{} ", name),
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
