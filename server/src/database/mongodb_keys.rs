use std::fmt::{Display, Formatter};

pub enum MongodbKey {
    // Databases
    MainDatabase,

    // Collections
    Game,
    Player,
}
impl Display for MongodbKey {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let k = match self {
            MongodbKey::MainDatabase => "skip_bo",

            MongodbKey::Game => "game",
            MongodbKey::Player => "player",
        };
        write!(f, "{k}")
    }
}

