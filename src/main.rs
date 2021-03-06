/*
ret_stack patterns simulate the stack of return values that
the function calls unwrap onto.
arg_stack patterns simulate the stack of function calls that unwrap

we maintain the call stack manually thus no recursion needed
*/
use std::fmt::Display;

type Noderef<T> = Option<Box<Node<T>>>;

#[derive(Debug, Default)]
struct Node<T> {
    value: T,
    right: Noderef<T>,
    left: Noderef<T>,
}
///# Call stack
/// this emulates the call stack.
/// ### ```Call<T>```
/// simlulates a recusrsive call
/// ### ```Handle<U>```
/// simulates other stuff thats not the recursive call
#[derive(Debug)]
enum Action<T, U> {
    Call(T),
    Handle(U),
}

#[allow(dead_code)]
fn generate_tree(level: usize, counter: &mut i32) -> Noderef<i32> {
    if level == 0 {
        return None;
    } else {
        let mut node = Node {
            value: *counter,
            left: None,
            right: None,
        };
        node.value = *counter;
        *counter += 1;
        node.right = generate_tree(level - 1, counter);
        //the counter stays updates becuause its borrowed
        node.left = generate_tree(level - 1, counter);
        Some(Box::new(node))
    }
}

#[allow(dead_code)]
fn gen_tree_norec(level: usize) -> Noderef<i32> {
    let mut counter = 1;
    let mut arg_stack = Vec::<Action<usize, i32>>::new();
    let mut ret_stack = Vec::<Noderef<i32>>::new();

    arg_stack.push(Action::Call(level));

    while let Some(action) = arg_stack.pop() {
        match action {
            Action::Call(level) => {
                if level > 0 {
                    arg_stack.push(Action::Handle(counter));
                    counter += 1;
                    arg_stack.push(Action::Call(level - 1));
                    arg_stack.push(Action::Call(level - 1));
                } else {
                    ret_stack.push(None);
                }
            }
            Action::Handle(value) => {
                let left = ret_stack.pop().unwrap();
                let right = ret_stack.pop().unwrap();
                ret_stack.push(Some(Box::new(Node { value, right, left })));
            }
        }
    }
    ret_stack.pop().unwrap()
}

#[allow(dead_code)]
fn invert_tree<T: Clone>(root: &Noderef<T>) -> Noderef<T> {
    match root {
        Some(node) => Some(Box::new(Node {
            value: node.value.clone(),
            left: invert_tree(&node.right),
            right: invert_tree(&node.left),
        })),
        None => None,
    }
}

#[allow(dead_code)]
fn invert_tree_norec<T: Clone>(root: &Noderef<T>) -> Noderef<T> {
    let mut arg_stack = Vec::<Action<&Noderef<T>, &T>>::new();
    let mut ret_stack = Vec::<Noderef<T>>::new();
    arg_stack.push(Action::Call(root));
    while let Some(action) = arg_stack.pop() {
        match action {
            Action::Call(root) => {
                if let Some(node) = root {
                    arg_stack.push(Action::Handle(&node.value));
                    arg_stack.push(Action::Call(&node.right));
                    arg_stack.push(Action::Call(&node.left));
                } else {
                    ret_stack.push(None);
                }
            }
            Action::Handle(value) => {
                let left = ret_stack.pop().unwrap();
                let right = ret_stack.pop().unwrap();
                ret_stack.push(Some(Box::new(Node {
                    value: value.clone(),
                    left,
                    right,
                })));
            }
        }
    }
    ret_stack.pop().unwrap()
}

#[allow(dead_code)]
fn visit_nodes_preorder<T: Display>(root: &Noderef<T>) {
    if let Some(node) = root {
        println!("{}", node.value);
        visit_nodes_preorder(&node.left);
        visit_nodes_preorder(&node.right);
    }
}

#[allow(dead_code)]
fn visit_nodes_preorder_norec<T: Display>(root: &Noderef<T>) {
    let mut stack = Vec::<Action<&Noderef<T>, &T>>::new();
    stack.push(Action::Call(root));
    while let Some(action) = stack.pop() {
        match action {
            Action::Call(root) => {
                if let Some(node) = root {
                    //reverse the order because stack
                    stack.push(Action::Call(&node.right));
                    stack.push(Action::Call(&node.left));
                    stack.push(Action::Handle(&node.value));
                }
            }

            Action::Handle(value) => {
                println!("{}", value);
            }
        }
    }
}

#[allow(dead_code)]
fn visit_nodes_postorder<T: Display>(root: &Noderef<T>) {
    if let Some(node) = root {
        visit_nodes_postorder(&node.left);
        visit_nodes_postorder(&node.right);
        println!("{}", node.value);
    }
}

#[allow(dead_code)]
fn visit_nodes_postorder_norec<T: Display>(root: &Noderef<T>) {
    let mut stack = Vec::<Action<&Noderef<T>, &T>>::new();
    stack.push(Action::Call(root));
    while let Some(action) = stack.pop() {
        match action {
            Action::Call(root) => {
                if let Some(node) = root {
                    //reverse the order because stack
                    stack.push(Action::Handle(&node.value));
                    stack.push(Action::Call(&node.right));
                    stack.push(Action::Call(&node.left));
                }
            }

            Action::Handle(value) => {
                println!("{}", value);
            }
        }
    }
}

#[allow(dead_code)]
fn visit_nodes_inorder<T: Display>(root: &Noderef<T>) {
    if let Some(node) = root {
        visit_nodes_inorder(&node.left);
        println!("{}", node.value);
        visit_nodes_inorder(&node.right);
    }
}

#[allow(dead_code)]
fn visit_nodes_inorder_norec<T: Display>(root: &Noderef<T>) {
    let mut stack = Vec::<Action<&Noderef<T>, &T>>::new();
    stack.push(Action::Call(root));
    while let Some(action) = stack.pop() {
        match action {
            Action::Call(root) => {
                if let Some(node) = root {
                    //reverse the order because stack
                    stack.push(Action::Call(&node.right));
                    stack.push(Action::Handle(&node.value));
                    stack.push(Action::Call(&node.left));
                }
            }

            Action::Handle(value) => {
                println!("{}", value);
            }
        }
    }
}

#[allow(dead_code)]
fn print_tree(root: &Noderef<i32>, level: usize) {
    match root {
        None => {}
        Some(node) => {
            print_tree(&node.right, level + 1);
            for _ in 0..level {
                print!("  ");
            }
            println!("{}", node.value);
            print_tree(&node.left, level + 1);
        }
    }
}

#[allow(dead_code)]
fn print_tree_norec<T: Display>(root: &Noderef<T>) {
    let mut stack = Vec::<Action<(&Noderef<T>, usize), (&T, usize)>>::new();
    use Action::*;
    stack.push(Call((root, 0)));
    while let Some(action) = stack.pop() {
        match action {
            Call((root, level)) => {
                if let Some(node) = root {
                    stack.push(Call((&node.left, level + 1)));
                    stack.push(Handle((&node.value, level)));
                    stack.push(Call((&node.right, level + 1)));
                }
            }
            Handle((value, level)) => {
                for _ in 0..level {
                    print!("  ")
                }
                println!("{}", value);
            }
        }
    }
}

//proof of concept function really doesnt fit here
//but does show that "ANY" recursive function can
//be made non recursive
#[allow(dead_code)]
fn fact_norec(num: u128) -> u128 {
    let mut current_num = num; //probably not needed but i cannot be bothered
    let mut arg_stack: Vec<Action<u128, u128>> = Vec::new();
    let mut ret_stack: Vec<u128> = Vec::new();
    arg_stack.push(Action::Call(num));
    while let Some(action) = arg_stack.pop() {
        match action {
            Action::Call(n) => {
                if n == 0 {
                    ret_stack.push(1);
                } else {
                    //note that the order here is reversed as its a stack so FIFO
                    arg_stack.push(Action::Handle(current_num));
                    current_num -= 1;
                    arg_stack.push(Action::Call(n - 1));

                }
            }
            Action::Handle(n) => {
                let prev_res = ret_stack.pop().unwrap() ;
                ret_stack.push((prev_res as u128) * n);
            }
        }
    }
    ret_stack.pop().unwrap()
}

fn main() {
    // let tree = generate_tree(3,&mut 1);
    let tree1 = gen_tree_norec(20);
    // print_tree(&tree1, 0);
    visit_nodes_inorder_norec(&tree1);
    // println!("{}", fact_norec(30));
}
