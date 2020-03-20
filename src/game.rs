pub mod guess;
pub mod target;

use guess::Guess;
use target::Target;

pub struct GameLoop {
    target: Target,
    guess: Guess
}

impl GameLoop {
    pub fn new() -> GameLoop {
        let target = Target::new();
        let guess = Guess::empty();

        GameLoop { target, guess }
    }

    pub fn get_guess(&self) -> String {
        if *self.guess.get_value() == 0 {
            return String::from("Você ainda não escolheu nenhum número");
        }
        self.guess.get_value().to_string().clone()
    }

    pub fn set_guess(&mut self, guess_string: &String) -> Result<(), &'static str> {
        self.guess = match Guess::new(&guess_string) {
            Ok(value) =>  value,
            Err(msg) => return Err(msg)
        };

        return Ok(());
    }

    pub fn validate_guess(&self) -> Result<(), &'static str> {
        match self.target.is_correct(&self.guess) {
            Err(msg) => Err(msg),
            Ok(_) => Ok(())
        }
    }
}