// mod token;
mod buffer;
mod lexer;
// mod lex;

fn main() {
    // let test_file: Vec<String> = vec![
    //     "uint a = 5;".to_string()
    // ];
    // let tokens: Vec<token::Token> = lexer::lex(test_file);
    // for token in tokens.iter() {
    //     println!("{:?}", token);
    // }

    let line = "hello world";
    let mut chars = line.chars().peekable();
    let first = chars.next_if(|c| c.is_alphabetic()).is_some() && chars.next_if(|c| c.is_alphabetic()).is_some();
    // chars.peek();
    // println!("{:?}", chars.peek());
    // println!("{:?}", chars.next());
    // println!("{:?}", chars.peek());
    // println!("{:?}", chars.peek());
/*á*/é
}
