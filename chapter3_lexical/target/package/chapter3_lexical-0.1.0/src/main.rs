use std::io::{self, Write};
use std::fmt;
use std::thread;
use std::time::Duration;
use crossterm::event::{self, Event, KeyCode};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode, size};
use crossterm::{ cursor,
    execute,
    terminal::{Clear, ClearType},
};

#[derive(Debug, Clone)]
pub enum TokenType {
    If,
    Then,
    Else,
    Id,
    Num,
    Relop,
}

pub struct Token {
    t_type: TokenType,
    lexeme: String,
    pos: i8,
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Token {{ type: {:?}, lexeme: '{}', position: {} }}",
            self.t_type, self.lexeme, self.pos
        )
    }
}

fn intro() -> io::Result<()> {
    let mut stdout = io::stdout();
    let (width, height) = size()?;
    let line = "Welcome To My Terminal Lexical Analyzer!";
    let line2 = "Please Type In Your Code And It Will Be Recognized!";
    let line3 = "We Recognize: if, then, else, <, >, !=, =, <=, numbers and ids!";
    let col = width.saturating_sub(line.len() as u16) / 2;
    execute!(stdout, Clear(ClearType::All), cursor::MoveTo(col, (height / 2) - 5))?;
    println!("{}", line);
    let col = width.saturating_sub(line2.len() as u16) / 2;
    execute!(stdout, Clear(ClearType::CurrentLine), cursor::MoveTo(col, (height / 2) - 2))?;
    println!("{}", line2);
    let col = width.saturating_sub(line3.len() as u16) / 2;
    execute!(stdout, Clear(ClearType::CurrentLine), cursor::MoveTo(col, (height / 2) + 1))?;
    println!("{}", line3);
    thread::sleep(Duration::from_millis(6000));
    Ok(())
}

fn main() -> io::Result<()> {
    intro()?;
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    let mut input = String::new();
    let mut last_token = TokenType::Id;
    execute!(stdout, Clear(ClearType::All), cursor::MoveTo(0, 0))?;
    println!("USE <ESC> TO EXIT OR <RETURN>");
    println!("Write your program (Last Token: {:?}    )", last_token);
    stdout.flush()?;

    let mut tokens: Vec<Token> = Vec::new();
    loop {
        execute!(
            stdout,
            cursor::MoveTo(0, 0),
            Clear(ClearType::CurrentLine)
        )?;
        println!("USE <ESC> TO EXIT OR <RETURN>");
        execute!(stdout, Clear(ClearType::CurrentLine), cursor::MoveTo(0, 1))?;
        println!("Write your program (Last Token: {:?})", last_token);
        execute!(stdout, Clear(ClearType::CurrentLine), cursor::MoveTo(0, 2))?;
        print!("{}", input);
        stdout.flush()?;
        last_token = scanner(&input, &mut tokens);
        if event::poll(Duration::from_millis(200))? {
            if let Event::Key(key_event) = event::read()? {
                match key_event.code {
                    KeyCode::Char(c) => {
                        input.push(c);
                    }
                    KeyCode::Enter => {
                        println!();
                        break;
                    }
                    KeyCode::Esc => break,
                    KeyCode::Backspace => {
                        input.pop();
                    }
                    _ => {}
                }
            }
        }
    }
    // Outside of the loop
    tokens.clear();
    disable_raw_mode()?;
    scanner(&input, &mut tokens);
    // Printing Setup
    let (width, height) = size()?;
    let mut _token: TokenType = TokenType::If;
    execute!(stdout, Clear(ClearType::All), cursor::MoveTo(width / 2 - 20, height/2 - 5))?;
    stdout.flush()?;
    let mut i: i16 = -4;
    for t in tokens {
        println!("{}", t);
        execute!(stdout, Clear(ClearType::CurrentLine), cursor::MoveTo(width / 2 - 20, ((height/2) as i16 + i) as u16))?;
        i = i+1
    }
    thread::sleep(Duration::from_millis(3000));
    Ok(())
}

fn scanner(stream: &String, tokens: &mut Vec<Token>) -> TokenType {
    // Split lexemes by whitespace
    let lexemes: Vec<&str> = stream.split_whitespace().collect();
    let mut token = TokenType::If;
    // Create Reserved Keywords
    let keywords = ["if", "else", "then"];
    // Iterate all lexemes, grabbing there index as well and checking
    // their types to assign an appropriate Token Type
    for (i, l) in lexemes.iter().enumerate() {
        // Parse Numbers
        if let Ok(_) = l.parse::<i32>() { tokenize(TokenType::Num, l.to_string(), Some(i), &mut token, tokens); }
        else if let Ok(_) = l.parse::<f64>() { tokenize(TokenType::Num, l.to_string(), Some(i), &mut token, tokens); }
        // Keyword Matching
        else if keywords.contains(l) {
            match *l {
                "if" => tokenize(TokenType::If, l.to_string(), Some(i), &mut token, tokens),
                "then" => tokenize(TokenType::Then, l.to_string(), Some(i), &mut token, tokens),
                "else" => tokenize(TokenType::Else, l.to_string(), Some(i), &mut token, tokens),
                _ => {} 
            };
        }
        // Alphanumeric ID Match
        else if l.chars().all(char::is_alphanumeric) { tokenize(TokenType::Id, l.to_string(), Some(i), &mut token, tokens); }
        else {
            match *l {
                "<" => tokenize(TokenType::Relop, String::from("LT"), Some(i), &mut token, tokens),
                ">" => tokenize(TokenType::Relop, String::from("GT"), Some(i), &mut token, tokens),
                "<=" => tokenize(TokenType::Relop, String::from("LE"), Some(i), &mut token, tokens),
                ">=" => tokenize(TokenType::Relop, String::from("GE"), Some(i), &mut token, tokens),
                "=" => tokenize(TokenType::Relop, String::from("ET"), Some(i), &mut token, tokens),
                "!=" => tokenize(TokenType::Relop, String::from("NE"), Some(i), &mut token, tokens),
                _ => {
                }
            };
        }
    }
    token
}

fn tokenize(t: TokenType, lex: String, pos: Option<usize>, token: &mut TokenType, tokens: &mut Vec<Token>) {
    tokens.push(Token {
        t_type: t,
        lexeme: lex,
        pos: pos.unwrap_or(0) as i8,
    });
    *token = tokens[tokens.len() - 1].t_type.clone();
}
