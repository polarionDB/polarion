#[derive(Debug, Default, PartialEq, PartialOrd, Ord, Eq, Hash, Clone)]
pub struct Move {
    pub piece: char,
    pub rank_to: char,
    pub file_to: char,
    pub rank_from: char,
    pub file_from: char,
    pub promotion: char,

    pub is_short_castle: bool,
    pub is_long_castle: bool,
    pub captures: bool,
    pub checks: bool,
    pub checkmates: bool,

    pub annotations: String,
}