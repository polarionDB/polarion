use crate::polarion_move::Move;
use super::parser::{self, PgnToken};
use std::{fs, str::FromStr, io::Read, os::unix::fs::MetadataExt};
use std::collections::HashMap;

#[derive(Debug, Default, PartialEq, Eq, Hash)]
pub struct PgnVariation {
    pub starts_at_half_move_number: u32,
    pub children_variations:        Vec<PgnVariation>,
    pub moves:                      Vec<Move>,
}

#[derive(Debug, Default, PartialEq, Eq)]
pub struct PgnGame {
    pub main_variation: PgnVariation,
    pub metadata:       HashMap<String, String>,
    pub last_move:      Box<Move>,
    pub result:         char,
}

#[derive(Debug, Default, PartialEq, Eq)]
pub struct PgnDatabase {
    games: Vec<PgnGame>,
}

impl TryFrom<fs::File> for PgnDatabase {
    type Error = Result<Self, String>;

    fn try_from(mut file: fs::File) -> Result<Self, Self::Error> {
        let metadata = file.metadata();
        
        if let Err(err) = metadata {return Err(Err(err.to_string()));}
        let metadata = metadata.unwrap();

        let mut string = String::with_capacity(metadata.size() as usize);
        let content = file.read_to_string(&mut string);

        if let Err(err) = content {return Err(Err(err.to_string()));}

        PgnDatabase::from_str(&string)
    }
}

impl FromStr for PgnDatabase {
    type Err = Result<Self, String>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut vec: Vec<PgnGame> = Vec::with_capacity(64);
        let mut game: PgnGame = PgnGame::default();
        let mut current_halfmove_number = 0;
        let mut current_colour = false;
        let mut is_first_move = true;
        let mut move_text_started = false;
        let mut has_result = false;
        let mut i = usize::MAX;
        let tokens = parser::parse_pgn(s.to_string());

        while i < tokens.len() || i == usize::MAX {
            i = i.wrapping_add(1);
            if i >= tokens.len() {break;}

            let token = &tokens[i];

            match token {
                parser::PgnToken::MetadataTag(key, value) => {
                    if move_text_started {
                        vec.push(game);
                        game = PgnGame::default();

                        move_text_started = false;
                        is_first_move = true;
                        has_result = false;
                    }

                    game.metadata.insert(key.to_string(), value.to_string());
                    continue;
                },

                parser::PgnToken::MoveNumber(number) => {
                    move_text_started = true;
                    handle_move_number(number.to_string(), &mut current_colour, &mut current_halfmove_number, &mut is_first_move, &mut game.main_variation);
                    continue;
                },

                parser::PgnToken::SanMove(san_move) => {
                    handle_san_move(san_move.to_string(), &mut game.main_variation, &mut game.last_move, &mut current_colour, &mut current_halfmove_number);
                    continue;
                }

                parser::PgnToken::Annoation(annotation) => {
                    (*game.last_move).annotations = annotation.to_string();
                    continue;
                },

                parser::PgnToken::VariationStart => {
                    i += handle_variation(&tokens[i+1..tokens.len()], &mut game.last_move, &mut game.main_variation, &mut current_colour, &mut current_halfmove_number)
                },
                
                parser::PgnToken::Result(result) => {
                    game.result = *result;
                    vec.push(game);
                    game = Default::default();

                    move_text_started = false;
                    is_first_move = true;
                    has_result = true;
                },
            
                _ => {}
            }
        }

        if !has_result {
            vec.push(game);
        }

        Ok(Self { games: vec })
    }
}

fn handle_move_number(mut value: String , colour: &mut bool, half_move_number: &mut u32, is_first_move: &mut bool, main_variation: &mut PgnVariation) {
    value.pop();

    if value.chars().nth(value.len() - 1) == Some('.') {
        *colour = true; value.pop(); value.pop();
    } else {
        *colour = false;
    }

    let move_number_n = u32::from_str(&value).unwrap();
    *half_move_number = (move_number_n - 1) * 2 + (*colour as u32);
    
    if *is_first_move {
        let mut variation = PgnVariation::default(); variation.starts_at_half_move_number = move_number_n;
        *main_variation = variation;
        *is_first_move = false;
    }
}

fn handle_san_move(mut value: String, variation: &mut PgnVariation, last_move: &mut Box<Move>, colour: &mut bool, half_move_number: &mut u32) {
    let mut san_move = Move::default();
    let mut promotes = false;

    if value.starts_with("O-O") {
        if value.len() == 3 {
            san_move.is_short_castle = true;
        } else {
            san_move.is_long_castle = true;
        }

        value.clear();
    }

    for c in  value.chars() {
        if promotes {
            san_move.promotion = c;
            promotes = false;
            continue;
        }

        if c == 'x' {
            san_move.captures = true;
            continue;
        }

        if c.is_lowercase() {
            if san_move.file_to != '\0' {
                san_move.file_from = san_move.file_to;                
            }

            san_move.file_to = c;
            continue;
        }

        if c.is_numeric() {
            if san_move.rank_to != '\0' {
                san_move.rank_from = san_move.rank_to;                
            }

            san_move.rank_to = c;
            continue;
        }

        if c.is_uppercase() {
            san_move.piece = c;
            continue;
        }

        if c == '+' {
            san_move.checks = true;
            continue;
        }

        if c == '#' {
            san_move.checkmates = true;
            continue;
        }

        if c == '=' {
            promotes = true;
            continue;
        }
    }

    if san_move.piece == '\0' && !san_move.is_short_castle && !san_move.is_long_castle {
        san_move.piece = 'P';
    }

    *colour = !*colour;
    *half_move_number += 1;

    variation.moves.push(san_move);
    *last_move = Box::new((*variation.moves.last().unwrap()).clone());
}

fn handle_variation(slice: &[PgnToken], last_move: &mut Box<Move>, inside_variation: &mut PgnVariation, colour: &mut bool, current_half_move_number: &mut u32) -> usize {
    let mut to_return = 0;
    
    let mut half_move_number: u32 = 0;
    let mut variation = PgnVariation::default(); variation.starts_at_half_move_number = *current_half_move_number;
    
    let mut i = usize::MAX;

    for token in slice {
        i = i.wrapping_add(1);

        match token {
            parser::PgnToken::MoveNumber(number) => {
                handle_move_number(number.to_string(), colour, &mut half_move_number, &mut false, &mut PgnVariation::default());
                continue;
            },

            parser::PgnToken::SanMove(san_move) => {
                handle_san_move(san_move.to_string(), &mut variation, last_move, colour, &mut half_move_number);
                continue;
            }

            parser::PgnToken::Annoation(annotation) => {
                (*last_move).annotations = annotation.to_string();
                continue;
            },

            parser::PgnToken::VariationStart => {
                to_return += handle_variation(&slice[i+1..slice.len()], last_move, &mut variation, colour, current_half_move_number);
                continue;
            },
            
            parser::PgnToken::VariationEnd => {
                to_return += i;
                break;
            },

            _ => {}
        }
    }

    inside_variation.children_variations.push(variation);
    to_return
}