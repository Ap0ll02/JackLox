use std::collections::HashSet;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Rule {
    lhs: String,
    rhs: Vec<String>,
}

#[derive(Debug)]
pub struct Grammar {
    pub rules: Vec<Rule>,
    pub terminals: HashSet<String>,
    pub nonterminals: HashSet<String>,
    pub start_symbol: String,
}

pub fn load_sample_grammar() -> Grammar {
    let mut rules = vec![
        Rule { lhs: "E'".into(), rhs: vec!["E".into()] },
        Rule { lhs: "E".into(), rhs: vec!["E".into(), "+".into(), "T".into()] },
        Rule { lhs: "E".into(), rhs: vec!["T".into()] },
        Rule { lhs: "T".into(), rhs: vec!["T".into(), "*".into(), "F".into()] },
        Rule { lhs: "T".into(), rhs: vec!["F".into()] },
        Rule { lhs: "F".into(), rhs: vec!["(".into(), "E".into(), ")".into()] },
        Rule { lhs: "F".into(), rhs: vec!["id".into()] },
    ];

    let terminals = ["id", "+", "*", "(", ")", "$"].iter().map(|s| s.to_string()).collect();
    let nonterminals = ["E'", "E", "F", "T"].iter().map(|s| s.to_string()).collect();

    Grammar {
        rules,
        terminals,
        nonterminals,
        start_symbol: "E'".to_string(),
    }
}
