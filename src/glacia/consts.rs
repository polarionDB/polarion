#![allow(dead_code)]

use phf::{Map, phf_map};

pub mod pieces {
    pub const AMOUNT_PIECES_SIDE: usize = 6;
    pub const AMOUNT_PIECES_BOTH: usize = AMOUNT_PIECES_SIDE * 2;

    pub const WHITE_KING: usize = 0;
    pub const BLACK_KING: usize = 1;

    pub const WHITE_QUEEN: usize = 2;
    pub const BLACK_QUEEN: usize = 3;

    pub const WHITE_ROOK: usize = 4;
    pub const BLACK_ROOK: usize = 5;

    pub const WHITE_BISHOP: usize = 6;
    pub const BLACK_BISHOP: usize = 7;

    pub const WHITE_KNIGHT: usize = 8;
    pub const BLACK_KNIGHT: usize = 9;

    pub const WHITE_PAWN: usize = 10;
    pub const BLACK_PAWN: usize = 11;

    pub const WHITE_PIECES: usize = 0;
    pub const BLACK_PIECES: usize = 1;
    pub const ALL_PIECES:   usize = 2;
}

pub mod fens {
    pub const STARTING_FEN: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
    pub const EMPTY_FEN:    &str = "8/8/8/8/8/8/8/8 w - - 0 1";
}

pub const CHAR_TO_PIECE: Map<&str, usize> = phf_map! {
    "K" => pieces::WHITE_KING,
    "k" => pieces::BLACK_KING,
    "Q" => pieces::WHITE_QUEEN,
    "q" => pieces::BLACK_QUEEN,
    "R" => pieces::WHITE_ROOK,
    "r" => pieces::BLACK_ROOK,
    "B" => pieces::WHITE_BISHOP,
    "b" => pieces::BLACK_BISHOP,
    "N" => pieces::WHITE_KNIGHT,
    "n" => pieces::BLACK_KNIGHT,
    "P" => pieces::WHITE_PAWN,
    "p" => pieces::BLACK_PAWN,
};

pub const PIECE_TO_CHAR: Map<char, char> = phf_map! {
    '0' => 'K',
    '1' => 'k',
    '2' => 'Q',
    '3' => 'q',
    '4' => 'R',
    '5' => 'r',
    '6' => 'B',
    '7' => 'b',
    '8' => 'N',
    '9' => 'n',
    ':' => 'P',
    ';' => 'p',
};
