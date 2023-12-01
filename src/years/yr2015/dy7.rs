use std::{collections::HashMap, str::FromStr};

use anyhow::anyhow;

enum Operator {
    And,
    Or,
    Not,
    LShift,
    RShift,
}

impl FromStr for Operator {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "AND" => Ok(Self::And),
            "OR" => Ok(Self::Or),
            "NOT" => Ok(Self::Not),
            "LSHIFT" => Ok(Self::RShift),
            "RSHIFT" => Ok(Self::LShift),
            _ => Err(anyhow!(
                r#"string must be "AND", "OR", "NOT", "LSHIFT", or "RSHIFT""#
            )),
        }
    }
}

pub(super) fn pt1(input: String) {
    let mut wires: HashMap<String, u16> = HashMap::new();

    for instruction in input.split('\n') {
        let mut input_a: Option<u16>;
        let mut operator: Option<Operator>;
        let mut input_b: Option<u16>;
        let mut output: &u16;

        let mut components = instruction.split_whitespace();
        let mut component = components
            .next()
            .expect("instruction in input should contain at least one component");

        input_a = component.parse().ok();

        if input_a.is_none() {
            input_a = wires.get(component).cloned();
        }

        if input_a.is_some() {
            component = components
                .next()
                .expect("instruction in input should contain another input");
        }

        operator = Operator::from_str(component).ok();
    }

    todo!();
}

pub(super) fn pt2(input: String) {
    todo!();
}
