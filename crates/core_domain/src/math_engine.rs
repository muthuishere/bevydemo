// SRP: Only handles math puzzle generation and validation
use serde::{Deserialize, Serialize};
use rand::Rng;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Operation {
    Addition,
    Subtraction,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Puzzle {
    pub operand_a: i32,
    pub operand_b: i32,
    pub operation: Operation,
    pub correct_answer: i32,
    pub choices: Vec<i32>,
}

impl Puzzle {
    pub fn new(a: i32, b: i32, op: Operation) -> Self {
        let answer = match op {
            Operation::Addition => a + b,
            Operation::Subtraction => a - b,
        };
        let mut rng = rand::thread_rng();
        let mut choices = vec![answer];
        while choices.len() < 4 {
            let wrong = answer + rng.gen_range(-5..=5);
            if wrong != answer && !choices.contains(&wrong) && wrong >= 0 {
                choices.push(wrong);
            }
        }
        use rand::seq::SliceRandom;
        choices.shuffle(&mut rng);
        Puzzle {
            operand_a: a,
            operand_b: b,
            operation: op,
            correct_answer: answer,
            choices,
        }
    }

    pub fn is_correct(&self, answer: i32) -> bool {
        answer == self.correct_answer
    }
}

pub struct PuzzleGenerator;

impl PuzzleGenerator {
    // OCP: generate puzzles based on level config without modifying core
    pub fn generate(max_a: i32, max_b: i32, op: Operation) -> Puzzle {
        let mut rng = rand::thread_rng();
        let a = rng.gen_range(1..=max_a);
        let b = rng.gen_range(1..=max_b.min(a));
        Puzzle::new(a, b, op)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_addition_correct() {
        let p = Puzzle::new(3, 4, Operation::Addition);
        assert_eq!(p.correct_answer, 7);
        assert!(p.is_correct(7));
        assert!(!p.is_correct(6));
        assert!(p.choices.contains(&7));
        assert_eq!(p.choices.len(), 4);
    }

    #[test]
    fn test_subtraction_correct() {
        let p = Puzzle::new(8, 3, Operation::Subtraction);
        assert_eq!(p.correct_answer, 5);
        assert!(p.is_correct(5));
    }

    #[test]
    fn test_puzzle_generator_addition() {
        let p = PuzzleGenerator::generate(10, 10, Operation::Addition);
        assert!(p.correct_answer > 0);
        assert!(p.choices.contains(&p.correct_answer));
    }

    #[test]
    fn test_puzzle_generator_subtraction() {
        let p = PuzzleGenerator::generate(10, 10, Operation::Subtraction);
        assert!(p.correct_answer >= 0);
    }
}
