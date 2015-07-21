use std::env;

fn main() {
    let mut stack = Vec::new();
    for arg in env::args() {
        stack.push(arg);    
    }
    while let Some(item) = stack.pop() {
        println!("{:?}", item);
    }
    
    println!("{:#?}", env::args().collect::<Vec<String>>());
}
