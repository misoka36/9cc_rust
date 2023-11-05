use core::panic;
use std::env;

#[derive(Debug)]
enum Token {
    RESERVED(char),
    NUM(usize),
}

#[derive(Debug)]
struct TokenInfo {
    token: Token,
    pos: usize,
}

fn print_token_err(message: &String, pos: usize, source_code: &String) {
    eprintln!("{}", source_code);
    eprintln!(
        "{}^ {}",
        " ".repeat(pos),
        message
    );
}

impl Token {
    fn tokenize(input: &String) -> Vec<TokenInfo> {
        let mut result = vec![];

        let mut chars = input.chars().enumerate().peekable();

        while let Some((i, c)) = chars.peek().cloned() {
            match c {
                '+' | '-' => {
                    chars.next();
                    result.push(
                        TokenInfo {
                            token: Token::RESERVED(c),
                            pos: i,
                        }
                    );
                },
                _ if c.is_digit(10) => {
                    let mut num_str = String::new();
                    while let Some((_, nc)) = chars.peek().cloned() {
                        if nc.is_digit(10) {
                            num_str.push(chars.next().expect("unreachable").1);
                        } else {
                            break;
                        }
                    }
                    result.push(
                        TokenInfo {
                            token: Token::NUM(num_str.parse::<usize>().expect("unreachable")),
                            pos: i,
                        }
                    );
                }
                ' ' => {
                    chars.next();
                }
                _ => {
                    print_token_err(&String::from("トークナイズできません"), i, input);
                    panic!();
                }
            }
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
    if let Token::NUM(v_tmp) = first_token.token {
        println!("  mov rax, {:?}", v_tmp);
    } else {
        panic!("Invalid first token: {:?}", first_token);
    }

    while let Some(token) = tokens_iter.next() {
        match token.token {
            Token::RESERVED(c) => {
                let next_token = tokens_iter.next().unwrap();
                let value = if let Token::NUM(v) = next_token.token {
                    v
                } else {
                    print_token_err(&String::from("数ではありません"), next_token.pos, &args[1]);
                    panic!();
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
