use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if env::args().len() != 2 {
        println!("引数が正しくありません");
        std::process::exit(1);
    }

    println!(".intel_syntax noprefix");
    println!(".global main");
    println!("main:");
    println!("  mov rax, {}", args[1]);
    println!("  ret");
    std::process::exit(0);
}
