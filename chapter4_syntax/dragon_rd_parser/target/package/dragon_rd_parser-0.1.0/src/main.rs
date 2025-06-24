use std::process::exit;

use colored::Colorize;

#[derive(Debug)]
enum Expr {
    Id(String),
    BinaryOp {
        op: String,
        left: Box<Expr>,
        right: Box<Expr>,
    }
}

fn main() {
    // Read Input From User (This should be in the "parsed" form)
    // For example (id + id) as oppposed to (4 + 8).
    println!("{}", "Please Enter Some Code In Tokenized Form\nThis looks like id (+|-|*|/) id (+|-, etc.)\n".white());
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("Couldn't Retrieve Your Input");
    let mut pos: usize = 0;

    let start_time = std::time::Instant::now();
    // Turning String into a vec of &str
    let mut input_vec: Vec<&str> = Vec::new();
    for i in input.split_whitespace() {
        if input.is_empty() { continue; }
        input_vec.push(i);
    }
    input_vec.push("$");
    input_vec.reverse();

    // Clear The Screen Some 
    println!("\n\n\n");
    let mut error_count: usize = 0;

    // Parsing Steps
    let expr = expr(&mut input_vec, &mut pos, &mut error_count);
    let end_time = start_time.elapsed();
    println!("{} {:?} {} {} {}", "PARSE COMPLETE in".cyan(), end_time, "With ".bright_cyan(), error_count, " errors".bright_cyan());
    println!("{}", "AST:\n".bold().bright_red());
    println!("Expression");
    print_ast(&expr, "", true);
}

// GRAMMAR: 
// E => T E'
// E' => OP T E' | eps 
// OP => + | - | * | /
// T => id
//
// Error Prods:
// E => E' (Missing Beginning Operator)

fn expr(input: &mut Vec<&str>, pos: &mut usize, err_cnt: &mut usize) -> Expr {
    println!("(E) => T E'");
    let left = term(input, pos, err_cnt);
    println!("T Parsed: E => T (E')");
    expr_prime(left, input, pos, err_cnt)
}

fn expr_prime(left: Expr, input: &mut Vec<&str>, pos: &mut usize, err_cnt: &mut usize) -> Expr {
    let display: Vec<_> = input.iter().rev().collect();
    println!("Input: {:?}", display);

    if let Some(&op_token) = input.last() {
        if ["+", "-", "*", "/"].contains(&op_token) {
            let op = op(input, pos, err_cnt);
            let right = term(input, pos, err_cnt);
            let combined = Expr::BinaryOp {
                op, left: Box::new(left), right: Box::new(right)
            };
            return expr_prime(combined, input, pos, err_cnt);
        }
    }

    if input.ends_with(&["$"]) {
        input.pop();
    }
    left 
}

fn term(input: &mut Vec<&str>, pos: &mut usize, err_cnt: &mut usize) -> Expr {
    let display: Vec<_> = input.iter().rev().collect();
    println!("Input {:?}", display);
    let test = input.pop();
    if test != Some("id") {
        error_routine(ErrType::MissingId, err_cnt);
        return Expr::Id("error".into());
    }
    println!("{}", "T => id\n".bright_green());
    *pos += 1;
    Expr::Id("id".into())
}

fn op(input: &mut Vec<&str>, pos: &mut usize, err_cnt: &mut usize) -> String {
    // let display: Vec<_> = input.iter().rev().collect();
    // println!("Input : {:?}", display);
    if input.len() <= 0 {
        error_routine(ErrType::Misc, err_cnt);
    }
    match input.as_slice() {
        [..,"+"] => {
            *pos += 1;
            input.pop();
            println!("{}", "OP => +\n".bright_green());
            "+".to_string()
        }
        [..,"*"] =>{
            input.pop();
            println!("{}", "OP => *\n".bright_green());
            *pos += 1;
            "*".to_string()
        }
        [..,"-"] =>{
            input.pop();
            println!("{}", "OP => -\n".bright_green());
            *pos += 1;
            "-".to_string()
        }
        [..,"/"] =>{
            input.pop();
            println!("{}", "OP => /\n".bright_green());
            *pos += 1;
            "/".to_string()
        }
        [.., "$"] => {
            error_routine(ErrType::MissingOperator, err_cnt);
            input.pop();
            "$".to_string()
        }
        _ => {
            error_routine(ErrType::MissingOperator, err_cnt);
            "ERROR".to_string()
        },
    }
}

#[derive(Debug)]
enum ErrType {
    MissingOperator,
    MissingId,
    Misc,
}
fn error_routine(err_type: ErrType, err_cnt: &mut usize) {
    *err_cnt += 1;
    if *err_cnt >= 10 {
        eprintln!("{}", "Too Many Errors (>= 10), Exiting...");
        exit(1);
    }
    match err_type {
        ErrType::MissingOperator => {
            eprintln!("{} {:?} {}", "Parse Error".red().bold(), err_type, "Missing Operator In Expression".red());
            eprintln!("{} {} {}", "Suggestion:".blue(), " Use +, -, *, / between to id's to follow pattern.".white(), "id __ id".red());
        }
        ErrType::MissingId => {
            eprintln!("{} {:?} {}", "Parse Error".red().bold(), err_type, "Missing ID In Expression".red());
            eprintln!("Use 'id' in place of any numbers, or different values. Follow the pattern id OPERATION id");
        }
        ErrType::Misc => {
            eprintln!("{} {:?}", "Parse Failed Due To".red(), err_type);
            exit(1)
        }
    }
}

fn print_ast(expr: &Expr, prefix: &str, is_last: bool) {
    let connector = if is_last { format!("{}{}", prefix, "└── ".magenta()) } else { format!("{}{}", prefix, "├── ".magenta()) };
    match expr {
        Expr::Id(name) => println!("{}{}", connector, format!("Id({})", name).blue()),
        Expr::BinaryOp { op, left, right } => {
            println!("{}{}", connector, format!("BinaryOp({})", op).green());
            let new_prefix = if is_last { format!("{}{}", prefix, "    ".magenta()) } else { format!("{}{}", prefix, "│   ".magenta()) };
            print_ast(left, &new_prefix, false);
            print_ast(right, &new_prefix, true);
        }
    }
}
