use std::collections::HashMap;
use crate::grammar::{self, Grammar, Rule};
use crate::lr0::{LR0DFA, Item};

#[derive(Debug, Clone)]
pub enum Action {
    Shift(usize),
    Reduce(usize),
    Accept,
    Error,
}

#[derive(Debug)]
pub struct SLRTable {
    pub action: HashMap<(usize, String), Action>,
    pub goto: HashMap<(usize, String), usize>,
}

pub fn build_slr_table(
    grammar: &Grammar,
    dfa: &LR0DFA,
    _follow: &HashMap<String, std::collections::HashSet<String>>,
) -> SLRTable {
    let mut action = HashMap::new();
    let mut goto = HashMap::new();

    action.insert((0, "$".to_string()), Action::Accept);

    SLRTable { action, goto }
}
