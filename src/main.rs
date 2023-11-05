use core::panic;
use std::env;

#[derive(Debug)]
enum Token {
    RESERVED(char),
    NUM(usize),
}


impl Token {
    fn tokenize(input: &String) -> Vec<Token> {
        let mut result = vec![];

        let mut chars = input.chars().peekable();

        while let Some(&c) = chars.peek() {
            match c {
                '+' | '-' => {
                    chars.next();
                    result.push(Token::RESERVED(c));
                }
                _ if c.is_digit(10) => {
                    let mut num_str = String::new();
                    while let Some(&nc) = chars.peek() {
                        if nc.is_digit(10) {
                            num_str.push(chars.next().unwrap());
                        } else {
                            break;
                        }
                    }
                    result.push(Token::NUM(num_str.parse::<usize>().unwrap()));
                }
                _ => {
                    chars.next();
                }
            };
        }
        return result;
    }
}


fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        panic!("引数の個数が正しくありません");
    }

    let tokens = Token::tokenize(&args[1]);
    eprintln!("Input token: {:?}", tokens);

    println!(".intel_syntax noprefix");
    println!(".global main");
    println!("main:");

    let mut tokens_iter = tokens.iter().peekable();

    let first_token = tokens_iter.next().unwrap();
    if let Token::NUM(v_tmp) = first_token {
        println!("  mov rax, {:?}", v_tmp);
    } else {
        panic!("Invalid first token: {:?}", first_token);
    }

    while let Some(token) = tokens_iter.next() {
        match token {
            Token::RESERVED(c) => {
                let value = if let Token::NUM(v) = tokens_iter.next().unwrap() {
                    v
                } else {
                    panic!("NUMBER dosen't continue after RESERVED token");
                };
                let op = match c {
                    '+' => "add",
                    '-' => "sub",
                    _ => panic!("unreachable"),
                };
                println!("  {} rax, {}", op, value);
            }
            _ => {
                panic!("Invalid token order");
            }
        }
    }

    println!("  ret");

}
