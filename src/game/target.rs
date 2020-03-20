use rand::Rng;
use std::cmp::Ordering;
use super::guess;

pub struct Target {
    secret_number: u8
}

impl Target {
    pub fn new() -> Target {
        let secret_number = rand::thread_rng().gen_range(1, 101);
        Target { secret_number }
    }

    pub fn is_correct(&self, guess: &guess::Guess) -> Result<(), &'static str> {
        if guess.is_empty() {
            return Err("Você esqueceu de passar um valor!")
        }

        match guess.get_value().cmp(&self.secret_number) {
            Ordering::Less => return Err("Pequeno demais"),
            Ordering::Greater => return Err("Grande demais"),
            Ordering::Equal => return Ok(())
        }
    }
}