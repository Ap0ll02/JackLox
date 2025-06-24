use crate::grammar::{self, Grammar, Rule};
use std::collections::{HashMap, HashSet};

pub struct Item {
    lhs: String,
    rhs: Vec<String>,
    dot: usize,
}

pub type State = usize;
pub type LR0DFA = Vec<HashSet<Item>>;

pub fn build_lr0_dfa(grammar: &Grammar) -> LR0DFA {
    let mut dfa = Vec::new();

    let start_rule = &grammar.rules[0];
    let start_item = Item {
        lhs: start_rule.lhs.clone(),
        rhs: start_rule.rhs.clone(),
        dot: 0,
    };

    let mut initial_set = HashSet::new();
    initial_set.insert(start_item);

    dfa.push(initial_set);
    dfa
}
