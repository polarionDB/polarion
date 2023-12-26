#![allow(dead_code)]

use crate::glacia::consts::{pieces::*, PIECE_TO_CHAR};
use crate::glacia::Board;
use crate::pmove::Move;
use super::constants::*;

type MoveGeneratorFunction = fn(&mut Board, u8, &mut Vec<Move>, usize);

pub(super) const PIECE_TO_MOVE_FUNCTION: [MoveGeneratorFunction; 12] = [
    Board::generate_moves_king,
    Board::generate_moves_king,
    Board::generate_moves_queen,
    Board::generate_moves_queen,
    Board::generate_moves_rook,
    Board::generate_moves_rook,
    Board::generate_moves_bishop,
    Board::generate_moves_bishop,
    Board::generate_moves_knight,
    Board::generate_moves_knight,
    Board::generate_moves_pawn,
    Board::generate_moves_pawn,
];

#[inline(always)]
fn square_to_rank(square: u8) -> char {
    (square.div_floor(8) + '1' as u8) as char
}

#[inline(always)]
fn square_to_file(square: u8) -> char {
    (square % 8 + 'a' as u8) as char
}

impl Board {
    #[inline(always)]
    fn generate_moves_general(&mut self, square: u8, vector: &mut Vec<Move>, piece: usize) {
        assert!(piece <= BLACK_PAWN);

        let mut moves = self.generate_attacks_piece_on_square(piece, square)
                           & !self.board.pieces[if piece % 2 == 0 {WHITE_PIECES} else {BLACK_PIECES}]
                           & !self.board.pieces[WHITE_KING]
                           & !self.board.pieces[BLACK_KING];

        let from = square;

        while moves != 0 {
            let removed = moves & (moves - 1);
            let to = (moves - removed).trailing_zeros() as u8;

            vector.push(Move {
                piece: PIECE_TO_CHAR[&((piece as u8 + '0' as u8) as char)],
                rank_from: square_to_rank(from),
                file_from: square_to_file(from),
                rank_to: square_to_rank(to),
                file_to: square_to_file(to),
                promotion: '\0',
                is_short_castle: false,
                is_long_castle: false,
                captures: false,
                checks: false,
                checkmates: false,
                annotations: "".to_string(),
            });

            moves = removed;
        }
    }

    // NON-SLIDING
    fn generate_moves_pawn(&mut self, square: u8, vector: &mut Vec<Move>, piece: usize) {
        let pawn = 1u64 << square;
        let mut moves_bb;
        let mut en_passants = 0u64;

        if piece % 2 == 0 {
            let single_move = pawn << 8     & !self.board.pieces[ALL_PIECES];
            let double_move = ((single_move & RANK_MASK[2]) << 8) & !self.board.pieces[ALL_PIECES];

            moves_bb = single_move | double_move | (PAWN_WHITE_ATTACKS[square as usize] & self.board.pieces[BLACK_PIECES]);

            if (pawn & RANK_MASK[4]) != 0 && ((self.en_passant_square & PAWN_WHITE_ATTACKS[square as usize]) != 0) {
                en_passants |= self.en_passant_square;
            }
        } else {
            let single_move = pawn >> 8     & !self.board.pieces[ALL_PIECES];
            let double_move = ((single_move & RANK_MASK[5]) >> 8) & !self.board.pieces[ALL_PIECES];

            moves_bb = single_move | double_move | (PAWN_BLACK_ATTACKS[square as usize] & self.board.pieces[WHITE_PIECES]);

            if (pawn & RANK_MASK[3]) != 0 && (self.en_passant_square & PAWN_BLACK_ATTACKS[square as usize] != 0) {
                en_passants |= self.en_passant_square;
            }
        }

        while moves_bb != 0 {
            let removed = moves_bb & (moves_bb - 1);
            let move_bb = moves_bb - removed;
            if move_bb == 0 {moves_bb = removed; continue;}

            if (move_bb & RANK_MASK[0]) != 0 || (move_bb & RANK_MASK[7]) != 0 {
                let start = if piece % 2 == 0 {2} else {3};
                let mut i = start;

                while i < WHITE_PAWN {

                    let to = move_bb.trailing_zeros() as u8;
                    
                    vector.push(Move {
                        piece: PIECE_TO_CHAR[&((piece as u8 + '0' as u8) as char)],
                        rank_from: square_to_rank(square),
                        file_from: square_to_file(square),
                        rank_to: square_to_rank(to),
                        file_to: square_to_file(to),
                        promotion: PIECE_TO_CHAR[&((i as u8 + '0' as u8) as char)],
                        is_short_castle: false,
                        is_long_castle: false,
                        captures: false,
                        checks: false,
                        checkmates: false,
                        annotations: "".to_string(),
                    });

                    i += 2;
                }

                moves_bb = removed;
                continue;
            }

            let to = move_bb.trailing_zeros() as u8;

            vector.push(Move {
                piece: PIECE_TO_CHAR[&((piece as u8 + '0' as u8) as char)],
                rank_from: square_to_rank(square),
                file_from: square_to_file(square),
                rank_to: square_to_rank(to),
                file_to: square_to_file(to),
                promotion: '\0',
                is_short_castle: false,
                is_long_castle: false,
                captures: false,
                checks: false,
                checkmates: false,
                annotations: "".to_string(),
            });

            moves_bb = removed;
        }

        while en_passants != 0 {
            let removed = en_passants & (en_passants - 1);
            let move_bb = en_passants - removed;

            if move_bb == 0 {en_passants = removed; continue;}

            let to = move_bb.trailing_zeros() as u8;

            vector.push(Move {
                piece: PIECE_TO_CHAR[&((piece as u8 + '0' as u8) as char)],
                rank_from: square_to_rank(square),
                file_from: square_to_file(square),
                rank_to: square_to_rank(to),
                file_to: square_to_file(to),
                promotion: '\0',
                is_short_castle: false,
                is_long_castle: false,
                captures: false,
                checks: false,
                checkmates: false,
                annotations: "".to_string(),
            });

            en_passants = removed;
        }
    }

    fn generate_moves_knight(&mut self, square: u8, vector: &mut Vec<Move>, piece: usize) {
        self.generate_moves_general(square, vector, piece)
    }

    fn generate_moves_king(&mut self, square: u8, vector: &mut Vec<Move>, piece: usize) {
        self.generate_moves_general(square, vector, piece);
    }

    // SLIDING
    fn generate_moves_bishop(&mut self, square: u8, vector: &mut Vec<Move>, piece: usize) {
        self.generate_moves_general(square, vector, piece)
    }

    fn generate_moves_rook(&mut self, square: u8, vector: &mut Vec<Move>, piece: usize) {
        self.generate_moves_general(square, vector, piece)
    }

    fn generate_moves_queen(&mut self, square: u8, vector: &mut Vec<Move>, piece: usize) {
        self.generate_moves_general(square, vector, piece)
    }

    #[inline(always)]
    pub fn generate_moves_piece_on_square(&mut self, piece_i: usize, square: u8, vector: &mut Vec<Move>) {
        PIECE_TO_MOVE_FUNCTION[piece_i](self, square, vector, piece_i);
    }

    #[inline(always)]
    pub fn generate_moves_piece(&mut self, piece_i: usize, vector: &mut Vec<Move>) {
        let mut piece = self.board.pieces[piece_i];


        while piece != 0 {
            let removed = piece & (piece - 1);
            let square = (piece - removed).trailing_zeros() as u8;

            self.generate_moves_piece_on_square(piece_i, square, vector);

            piece = removed;
        }
    }

    // OVERALL
    pub fn generate_moves(&mut self, generate_for_both_colours: bool) -> Vec<Move> {
        // Interesting source: https://chess.stcackexchange.com/questions/23135/what-is-the-average-number-of-legal-moves-per-turn
        // But this analyses legal moves, not pseudo-legal. Since we can only generate pseudo-legal moves, I'll double it to around 70
        let mut moves_vec: Vec<Move> = Vec::with_capacity(70);

        let start = if generate_for_both_colours || !self.side {0} else {1};

        let skip = if generate_for_both_colours {1} else {2};
        let mut i = start;

        while i <= BLACK_PAWN {
            self.generate_moves_piece(i, &mut moves_vec);
            i += skip;
        }

        moves_vec
    }
}
