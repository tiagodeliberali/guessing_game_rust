use std::io;
use guessing_game::GameLoop;

fn main() {
    println!("Adivinhe o número!");
    let mut game_loop = GameLoop::new();

    loop {
        println!("Qual é o seu número escolhido?");
        
        let mut guess = String::new();
        io::stdin().read_line(&mut guess)
            .expect("Não consegui ler a linha");

        if let Err(msg) = game_loop.set_guess(&guess) {
            println!("{}", msg);
            continue;
        };

        println!("Você escolheu o número {}", game_loop.get_guess());

        match game_loop.validate_guess() {
            Err(msg) => println!("{}", msg),
            Ok(_) => {
                println!("Você acertou!");
                break;
            }
        }
    }
}
