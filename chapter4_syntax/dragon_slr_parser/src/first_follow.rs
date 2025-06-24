use std::collections::{HashMap, HashSet};
use crate::grammar::Grammar;

pub fn compute_follow_sets(grammar: &Grammar) -> HashMap<String, HashSet<String>> {
    let mut follow = HashMap::new();

    for nt in &grammar.nonterminals {
        follow.insert(nt.clone(), HashSet::new());
    }

    follow.get_mut(&grammar.start_symbol).unwrap().insert("$".to_string());

    follow
}
