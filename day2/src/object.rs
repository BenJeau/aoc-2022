use std::cmp::Ordering;

#[derive(Eq, PartialEq)]
pub enum Object {
    Rock,
    Paper,
    Scissor,
}

impl Ord for Object {
    fn cmp(&self, other: &Self) -> Ordering {
        match self {
            Object::Rock => match other {
                Object::Rock => Ordering::Equal,
                Object::Paper => Ordering::Less,
                Object::Scissor => Ordering::Greater,
            },
            Object::Paper => match other {
                Object::Rock => Ordering::Greater,
                Object::Paper => Ordering::Equal,
                Object::Scissor => Ordering::Less,
            },
            Object::Scissor => match other {
                Object::Rock => Ordering::Less,
                Object::Paper => Ordering::Greater,
                Object::Scissor => Ordering::Equal,
            },
        }
    }
}

impl PartialOrd for Object {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Object {
    pub fn value(&self) -> usize {
        match self {
            Self::Rock => 1,
            Self::Paper => 2,
            Self::Scissor => 3,
        }
    }

    pub fn convert_char(char: &str) -> Option<Self> {
        match char {
            "A" | "X" => Some(Self::Rock),
            "B" | "Y" => Some(Self::Paper),
            "C" | "Z" => Some(Self::Scissor),
            _ => None,
        }
    }
}
