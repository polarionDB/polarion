mod pgn;
mod polarion_move;
mod board;

use std::{fs, env};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let arg = env::args().nth(1).unwrap();
    let file = fs::File::open(arg)?;
    let database = pgn::PgnDatabase::try_from(file).unwrap();
    
    println!("{:#?}", database);

    Ok(())
}
