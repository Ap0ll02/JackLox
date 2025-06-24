use crate::slr_table::{SLRTable, Action};

pub fn parse(tokens: Vec<&str>, table: &SLRTable) {
    let mut stack: Vec<usize> = vec![0];
    let mut input = tokens.clone();
    let mut pos = 0;

    println!("Starting Parse: {:?}", input);

    loop {
        let state = *stack.last().unwrap();
        let token = input.get(pos).unwrap_or(&"$").to_string();

        let action = table.action.get(&(state, token.clone())).unwrap_or(&Action::Error);

        println!("State: {}, Token: {}, Action: {:?}", state, token, action);

        match action {
            Action::Shift(s) => {
                stack.push(*s);
                pos += 1;
            }
            Action::Reduce(rule_index) => {
                println!("Would reduce using rule #{}", rule_index);
            }
            Action::Accept => {
                println!("Input Accepted");
                break;
            }
            Action::Error => {
                println!("Parse Error at Token: {}", token);
                break;
            }
        }
    }
}
