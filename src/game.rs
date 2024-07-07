

pub struct Game {
    secret_word: String,
}


impl Game {
    pub fn new() -> Game {
        Game {
            secret_word: "secret".to_string(),
        }
    }

    pub fn get_secret(&self) -> &String {
        &self.secret_word
    }
}