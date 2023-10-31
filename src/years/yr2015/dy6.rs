use std::str::FromStr;

use anyhow::{anyhow, bail};

#[derive(Debug)]
struct Instruction {
    kind: InstructionKind,
    start: (usize, usize),
    end: (usize, usize),
}

#[derive(Debug)]
enum InstructionKind {
    TurnOn,
    TurnOff,
    Toggle,
}

impl FromStr for Instruction {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let kind = match s {
            i if i.starts_with("turn on") => InstructionKind::TurnOn,
            i if i.starts_with("turn off") => InstructionKind::TurnOff,
            i if i.starts_with("toggle") => InstructionKind::Toggle,
            _ => {
                bail!(r#"string did not start with "turn on", "turn off", or "toggle""#);
            }
        };

        let start_i = match kind {
            InstructionKind::TurnOn => 8,
            InstructionKind::TurnOff => 9,
            InstructionKind::Toggle => 7,
        };
        let through = s
            .find("through")
            .ok_or(anyhow!(r#"instruction should contain "through""#))?;
        let start = s[start_i..(through - 1)]
            .split_once(',')
            .ok_or(anyhow!("coord pairs should be separated with ','"))?;
        let start: (usize, usize) = (start.0.parse()?, start.1.parse()?);
        let end = s[(through + 8)..]
            .split_once(',')
            .ok_or(anyhow!("coord pairs should be separated with ','"))?;
        let end: (usize, usize) = (end.0.parse()?, end.1.parse()?);

        Ok(Instruction { kind, start, end })
    }
}

pub(super) fn pt1(input: String) {
    let mut grid = [[false; 1000]; 1000];

    for instruction in input.split('\n') {
        let instruction = Instruction::from_str(instruction).unwrap();

        for column in instruction.start.0..=instruction.end.0 {
            for row in instruction.start.1..=instruction.end.1 {
                match instruction.kind {
                    InstructionKind::TurnOn => grid[column][row] = true,
                    InstructionKind::TurnOff => grid[column][row] = false,
                    InstructionKind::Toggle => grid[column][row] = !grid[column][row],
                }
            }
        }
    }

    let mut lit_lights: u32 = 0;

    for column in grid {
        for light_lit in column {
            if light_lit {
                lit_lights += 1;
            }
        }
    }

    println!("{lit_lights}");
}

pub(super) fn pt2(input: String) {
    let mut grid: Vec<[u8; 1000]> = vec![[0; 1000]; 1000];

    for instruction in input.split('\n') {
        let instruction = Instruction::from_str(instruction).unwrap();

        for column in instruction.start.0..=instruction.end.0 {
            for row in instruction.start.1..=instruction.end.1 {
                match instruction.kind {
                    InstructionKind::TurnOn => grid[column][row] += 1,
                    InstructionKind::TurnOff => {
                        if grid[column][row] > 0 {
                            grid[column][row] -= 1
                        }
                    }
                    InstructionKind::Toggle => grid[column][row] += 2,
                }
            }
        }
    }

    let mut total_brightness: u32 = 0;

    for column in grid {
        for brightness in column {
            total_brightness += brightness as u32;
        }
    }

    println!("{total_brightness}");
}
