//! # Challenge {{project-name}}
//!


const INPUT: &str = include_str!("../data/input.txt");

pub fn challenge_{{project-name}}() -> String {
    format!("INPUT: {}", INPUT)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let msg = challenge_{{project-name}}();
        assert_eq!(msg, "INPUT: ");
    }
}
