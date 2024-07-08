
pub mod game;
use game::Game;

pub fn run(secret_len: u8) {
    let game: Game = Game::new(secret_len);

    println!("The secret word is {} letters long.", secret_len);
    println!("Hello Hangman. {}", game.get_secret());

    let guess: char = loop {
        match game.get_user_guess() {
            Ok(c) => break c,
            Err(err) => println!("{}", err),
        }
    };

    println!("captured guess {}", guess);
}
