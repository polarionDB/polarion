#![allow(dead_code, unused_variables)]

use crate::glacia::{Board, BitBoard, pieces::*};
use super::constants::{*, magics::*};

pub(super) type AttackGeneratorFunction = fn(&Board, u8) -> BitBoard;

pub(super) const PIECE_TO_ATTACK_FUNCTION: [AttackGeneratorFunction; 10] = [
    Board::generate_attacks_king,
    Board::generate_attacks_king,
    Board::generate_attacks_queen,
    Board::generate_attacks_queen,
    Board::generate_attacks_rook,
    Board::generate_attacks_rook,
    Board::generate_attacks_bishop,
    Board::generate_attacks_bishop,
    Board::generate_attacks_knight,
    Board::generate_attacks_knight,
];

impl Board {  
    // NON-SLIDING
    // Note: You can abstract with a general function which's signature would look
    //       something like this: fn(&self, pointer_to_lookup_table: &[BitBoard], square: u8)
    //       But that much abstraction is not necessary and does more harm than good.
    #[inline(always)]
    pub(super) fn generate_attacks_pawn(&self, square: u8, side: bool) -> BitBoard {
        if !side {
            PAWN_WHITE_ATTACKS[square as usize]
        } else {
            PAWN_BLACK_ATTACKS[square as usize]
        }
    }

    #[inline(always)]
    pub(super) fn generate_attacks_knight(&self, square: u8) -> BitBoard {
        KNIGHT_ATTACKS[square as usize]
    }

    #[inline(always)]
    pub(super) fn generate_attacks_king(&self, square: u8) -> BitBoard {
        KING_ATTACKS[square as usize]
    }

    // SLIDING

    #[inline(always)]
    fn magic_index(&self, entry: &MagicEntry) -> usize {
        let blockers = self.board.pieces[ALL_PIECES] & entry.mask;
        let hash = blockers.wrapping_mul(entry.magic);
        let index = (hash >> entry.shift) as usize;

        entry.offset as usize + index
    } 

    #[inline(always)]
    pub(super) fn generate_attacks_bishop(&self, square: u8) -> BitBoard { 
        BISHOP_MOVES[self.magic_index(&BISHOP_MAGICS[square as usize])]
    }

    #[inline(always)]
    pub(super) fn generate_attacks_rook(&self, square: u8) -> BitBoard {
        ROOK_MOVES[self.magic_index(&ROOK_MAGICS[square as usize])]
    }

    #[inline(always)]
    pub(super) fn generate_attacks_queen(&self, square: u8) -> BitBoard {
        self.generate_attacks_bishop(square) | self.generate_attacks_rook(square)
    }

    // OVERALL
    #[inline(always)]
    pub fn generate_attacks_piece_on_square(&mut self, piece: usize, square: u8) -> BitBoard {
        // This kinda bothers me but it's one if statement, it shouldn't be that bad
        assert!(piece <= BLACK_PAWN);

        let attacks;

        // If piece is more than 9, so piece is pawn
        if piece > 9 {
            attacks = Board::generate_attacks_pawn(self, square, 
                unsafe { std::mem::transmute((piece % 2) as u8) }
            )
        } else {
            // I use a table of function pointers so the lookup is fast and easy
            attacks = PIECE_TO_ATTACK_FUNCTION[piece](self, square)
        }

        attacks
    }

    #[inline(always)]
    pub fn generate_attacks_square(&mut self, square: u8) -> BitBoard {
        let mut piece_bb = 0;
        let mut piece = usize::MAX;

        let start = if !self.side {WHITE_PAWN} else {BLACK_PAWN};

        let square = 1u64 << square;
        println!("square: {square:?}");
        let mut i: i64 = start as i64;

        while i >= WHITE_KING as i64 {
            if self.board.pieces[i as usize] & square != 0 {
                piece_bb = self.board.pieces[i as usize]; 
                piece = i as usize;
                
                break;
            }

            i -= 2;
        }

        if piece == usize::MAX {return 0;}

        return self.generate_attacks_piece_on_square(piece, piece_bb.trailing_zeros() as u8);
    }

    #[inline(always)]
    pub fn generate_attacks_piece(&mut self, piece: usize) -> BitBoard {
        let mut n = self.board.pieces[piece];
        let mut attacks = 0;
    
        while n != 0 {
            let removed_square                = n & (n - 1);
            // Get's the position of the LSB.
            let square_of_rightest_most_piece = (n - removed_square).trailing_zeros() as u8;
            n = removed_square;
            
            attacks |= self.generate_attacks_piece_on_square(piece, square_of_rightest_most_piece);
        }
        
        attacks
    }
}