use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    println!(".intel_syntax noprefix");
    println!(".global main");
    println!("main:");
    println!("  mov rax, {}", args[1].parse::<i32>().unwrap());
    println!("  ret");
}
