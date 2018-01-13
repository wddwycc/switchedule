use std::str;


#[derive(Serialize, Deserialize, Debug)]
pub struct Area {
    pub name: String,
    pub games: Vec<Game>,
}


#[derive(Serialize, Deserialize, Debug)]
pub struct Game {
    pub name: String,
}
