mod bag;

pub mod game_state;
pub mod tetromino_data;

/// Identifies one of the 7 types of tetrominoes
#[derive(Clone, Debug)]
pub enum TetrominoType { I, O, T, J, L, S, Z, }
