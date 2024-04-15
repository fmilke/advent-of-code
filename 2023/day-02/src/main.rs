use std::str::FromStr;


#[derive(Default, Debug)]
struct Game {
    draws: Vec<Draw>,
}

#[derive(Default, Debug)]
struct Draw {
}


impl FromStr for Game {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {


        
        let mut r = Game::default();



        todo!()
    }
}

impl FromStr for Draw {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        todo!()
    }
}


fn main() {

    println!("Hello, world!");


}
 

