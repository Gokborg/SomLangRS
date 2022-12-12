mod token;
mod lexer;

fn main() {
    let test_file: Vec<String> = vec![
        "1".to_string()
    ];
    let tokens: Vec<token::Token> = lexer::lex(test_file);
    for token in tokens.iter() {
        println!("{:?}", token);
    }
}