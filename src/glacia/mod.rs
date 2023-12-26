// GLACIA - HIGHLY CUSTOMIZED PSEUDO-LEGAL MOVE GENERATOR FOR CONVERTING SAN TO LAN

// NOTES: Doesn't understand checks/checkmate. Doesn't generate castle moves

pub mod bitboard;
mod consts;
mod generators;

pub use bitboard::*;
use consts::{pieces::*, *};


#[derive(Debug, Clone, Copy, Hash)]
pub struct BareBonesBoard {
    pub pieces: [BitBoard; AMOUNT_PIECES_BOTH],
    pub side_pieces: [BitBoard; 3],   
}

#[derive(Debug, Clone, Copy, Hash)]
pub struct Board {
    pub board: BareBonesBoard,

    pub side: bool,
    pub en_passant_square: BitBoard,
}

impl From<&str> for Board {
    /// A function that takes in a FEN (https://en.wikipedia.org/wiki/Forsyth–Edwards_Notation) string and returns an initialized struct Board
    fn from(_string: &str) -> Board {
        let string = _string.as_bytes();
        let mut rank = 7;
        let mut file = 0;
        let mut to_return: Self = unsafe { std::mem::zeroed() };

        let mut i = 0;
        
        while i < _string.len() {
            let c = string[i];

            match c {
                b'/' => {
                    rank -= 1; file = 0; i += 1; continue;
                }

                b'0'..=b'9' => {
                    let as_number = c - b'1';
                    file += as_number;
                }

                b' ' => {
                    i += 1;
                    to_return.side = if string[i] == b'w' {false} else {true};
                    
                    i += 2;

                    while string[i] != b' ' {
                        let char = string[i];
                        
                        if char == b'-' {
                            i += 1;
                            break;
                        }

                        i += 1;
                    }

                    i += 1;

                    if string[i] != b'-' {
                        to_return.en_passant_square = 
                            1 << (
                            (string[i + 1] as u64 - '1' as u64) * 8
                          +  string[i]     as u64 - 'a' as u64);
                    } else {
                        to_return.en_passant_square = 0;
                    }

                    break;
                }

                b'a'..=b'z' | b'A'..=b'Z' => {
                    let square = 1u64 << (rank * 8 + file);

                    to_return.board.pieces[CHAR_TO_PIECE[&(c as char).to_string()]] |= square;
                    to_return.board.pieces[WHITE_PIECES + (c as char).is_lowercase() as usize] |= square;
                    to_return.board.pieces[ALL_PIECES] |= square;
                }

                _ => {
                    eprintln!("Invalid charachter {} in fen string specified.", c);
                    std::process::exit(1);
                }
            }

            file += 1;
            i += 1;
        }
        to_return
    }
}

impl Board {
    pub fn starting() -> Self {
        return Board::from(fens::STARTING_FEN);
    }

    pub fn empty() -> Self {
        return Board::from(fens::EMPTY_FEN);
    }
}