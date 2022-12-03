use crate::object::Object;
use std::cmp::Ordering;

pub struct Round {
    player1: Object,
    player2: Object,
}

impl Round {
    pub fn value(&self) -> usize {
        match self.player1.cmp(&self.player2) {
            Ordering::Less => 6,
            Ordering::Equal => 3,
            Ordering::Greater => 0,
        }
    }

    pub fn total_score(&self) -> usize {
        self.player2.value() + self.value()
    }

    pub fn part_2_pre_process(mut self) -> Self {
        self.player2 = match self.player2 {
            // Need to lose
            Object::Rock => match self.player1 {
                Object::Rock => Object::Scissor,
                Object::Paper => Object::Rock,
                Object::Scissor => Object::Paper,
            },
            // Need to draw
            Object::Paper => self.player1,
            // Need to win
            Object::Scissor => match self.player1 {
                Object::Rock => Object::Paper,
                Object::Paper => Object::Scissor,
                Object::Scissor => Object::Rock,
            },
        };

        self
    }
}

impl FromIterator<Object> for Round {
    fn from_iter<I: IntoIterator<Item = Object>>(iter: I) -> Self {
        let mut iter = iter.into_iter();
        Round {
            player1: iter.next().unwrap(),
            player2: iter.next().unwrap(),
        }
    }
}
