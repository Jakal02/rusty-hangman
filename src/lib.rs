
pub mod game;
use game::Game;

pub fn run(secret_len: u8) {
    let game = Game::new(secret_len);

    println!("The secret word is {} letters long.", secret_len);
    println!("Hello Hangman. {}", game.get_secret())
}
