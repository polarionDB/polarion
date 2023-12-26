use crate::pgn::{PgnDatabase, PgnVariation};
use crate::glacia::*;

impl PgnVariation {
    pub fn to_position_table(&self, board: &mut Board, table: &mut Vec<BareBonesBoard>) {
        todo!()
    }
}

impl PgnDatabase {
    pub fn generate_position_table(&self) -> Vec<BareBonesBoard> {
        let mut table = vec![];
        let starting_board = Board::starting();
        
        for game in &self.games {
            let mut board = starting_board.clone();

            game.main_variation.to_position_table(&mut board, &mut table);
        }

        table
    }
}