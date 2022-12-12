mod token;
mod buffer;
mod lexer;

fn main() {
    let test_file: Vec<String> = vec![
        "uint a = 5;".to_string()
    ];
    let tokens: Vec<token::Token> = lexer::lex(test_file);
    for token in tokens.iter() {
        println!("{:?}", token);
    }
}