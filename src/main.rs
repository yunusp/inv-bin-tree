use std::fmt::Display;

type Noderef<T> = Option<Box<Node<T>>>;

#[derive(Debug, Default)]
struct Node<T> {
    value: T,
    right: Noderef<T>,
    left: Noderef<T>,
}
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
        node.left = generate_tree(level - 1, counter);
        //the counter stays updates becuause its borrowed
        node.right = generate_tree(level - 1, counter);
        Some(Box::new(node))
    }
}

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
                    arg_stack.push(Action::Call(level - 1));
                    arg_stack.push(Action::Call(level - 1));
                }else {
                    ret_stack.push(None);
                }
            }
            Action::Handle(value) => {
                let left = ret_stack.pop().unwrap();
                let right = ret_stack.pop().unwrap();
                ret_stack.push(Some(Box::new(Node {
                    value,
                    right,
                    left,
                })));
                counter += 1;
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
fn visit_nodes<T: Display>(root: &Noderef<i32>) {
    match root {
        Some(node) => {
            println!("{}", node.value);
            visit_nodes::<i32>(&node.left);
            visit_nodes::<i32>(&node.right);
        }
        None => {}
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
fn main() {
    let tree = gen_tree_norec(3);
    print_tree_norec(&tree);
    println!("-------------------");
    print_tree_norec(&invert_tree_norec(&tree));
}
