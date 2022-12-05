use crate::movement::Movement;
use std::{collections::VecDeque, str::FromStr};

pub struct Supplies {
    stacks: Vec<VecDeque<char>>,
    movements: Vec<Movement>,
}

impl Supplies {
    pub fn new(file_content: String) -> Self {
        let sections: Vec<_> = file_content.split("\n\n").collect();

        Supplies {
            stacks: Self::parse_stacks(sections[0]),
            movements: Self::parse_movements(sections[1]),
        }
    }

    fn parse_movements(section: &str) -> Vec<Movement> {
        section
            .split("\n")
            .filter_map(|line| Movement::from_str(line).ok())
            .collect()
    }

    fn parse_stacks(section: &str) -> Vec<VecDeque<char>> {
        let length_of_string = section.split("\n").next().unwrap().len();
        let num_stacks = ((length_of_string - 1) / 4) + 1;

        let mut stacks = vec![VecDeque::new(); num_stacks];

        section.split("\n 1").collect::<Vec<_>>()[0]
            .lines()
            .for_each(|line| {
                let chars = line.chars().collect::<Vec<_>>();
                (1..=line.len())
                    .step_by(4)
                    .map(|index| chars[index])
                    .enumerate()
                    .for_each(|(index, value)| {
                        if value != ' ' {
                            stacks[index].push_back(value)
                        }
                    })
            });

        stacks
    }

    pub fn make_single_movements(&mut self) -> &mut Self {
        self.movements
            .iter()
            .for_each(|Movement { amount, from, to }| {
                for _ in 0..*amount {
                    if let Some(value) = self.stacks[(from - 1) as usize].pop_front() {
                        self.stacks[(to - 1) as usize].push_front(value);
                    }
                }
            });

        self
    }

    pub fn make_bulk_movements(&mut self) -> &mut Self {
        self.movements
            .iter()
            .for_each(|Movement { amount, from, to }| {
                let mut temp = VecDeque::new();
                for _ in 0..*amount {
                    if let Some(value) = self.stacks[(from - 1) as usize].pop_front() {
                        temp.push_front(value);
                    }
                }
                temp.iter()
                    .for_each(|value| self.stacks[(to - 1) as usize].push_front(*value));
            });

        self
    }

    pub fn get_top_crates(&self) -> String {
        self.stacks
            .iter()
            .fold(vec![], |mut acc, stack| {
                if let Some(value) = stack.front() {
                    acc.push(*value);
                }
                acc
            })
            .iter()
            .collect()
    }
}
