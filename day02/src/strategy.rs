use crate::Shape;

pub trait Strategy {
    fn strategy(you: Shape, them: Shape) -> u32;
}

pub struct Guide;

impl Strategy for Guide {
    fn strategy(you: Shape, them: Shape) -> u32 {
        let choicepoint = you as u32;
        let matchpoint = match (you, them) {
            // you win
            (Shape::Rock, Shape::Scissors)
            | (Shape::Paper, Shape::Rock)
            | (Shape::Scissors, Shape::Paper) => 6,

            // you draw
            (Shape::Rock, Shape::Rock)
            | (Shape::Paper, Shape::Paper)
            | (Shape::Scissors, Shape::Scissors) => 3,

            // orelse you lose
            _ => 0,
        };
        choicepoint + matchpoint
    }
}

pub struct Choice;

impl Strategy for Choice {
    fn strategy(you: Shape, them: Shape) -> u32 {
        match you {
            // you need to lose
            Shape::Rock => match them {
                Shape::Rock => Shape::Scissors as u32,
                Shape::Paper => Shape::Rock as u32,
                Shape::Scissors => Shape::Paper as u32,
            },

            // you need to draw
            Shape::Paper => them as u32 + 3,

            // you need to win
            Shape::Scissors => {
                6 + match them {
                    Shape::Rock => Shape::Paper as u32,
                    Shape::Paper => Shape::Scissors as u32,
                    Shape::Scissors => Shape::Rock as u32,
                }
            }
        }
    }
}
