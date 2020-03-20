pub struct Guess {
    value: u8
}

impl Guess {
    pub fn new(guess: &String) -> Result<Guess, &'static str> {
        let value = match guess.trim().parse() {
            Ok(value) => value,
            Err(_) => return Err("Valor inválido!")
        };

        if value > 100 {
            return Err("Escolha um valor menor  ou igual que 100")
        }

        if value < 1 {
            return Err("Escolha um valor igual ou maior que 1")
        }

        Ok(Guess { value })
    }

    pub fn empty() -> Guess {
        Guess { value: 0 }
    }

    pub fn is_empty(&self) -> bool {
        self.value == 0
    }

    pub fn get_value(&self) -> &u8 {
        &self.value
    }
}
