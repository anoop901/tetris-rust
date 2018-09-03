extern crate rand;
use self::rand::Rng;

/// Array of all 7 tetromino types.
const ALL_TETROMINO_TYPES: [::TetrominoType; 7] = [
    ::TetrominoType::I,
    ::TetrominoType::O,
    ::TetrominoType::T,
    ::TetrominoType::J,
    ::TetrominoType::L,
    ::TetrominoType::S,
    ::TetrominoType::Z
];

/// An object that randomly gives `TetrominoType`s in such an order that every
/// 7 values returned contains one of each variant.
#[derive(Debug)]
pub struct Bag {
    remaining: Vec<::TetrominoType>
}

impl Bag {

    /// Creates a new `Bag` in its initial state.
    pub fn new() -> Self {
        Bag {
            /// A `Vec` of all the pieces that have not yet been returned in
            /// the current set of 7.
            remaining: Vec::from(&ALL_TETROMINO_TYPES as &[::TetrominoType])
        }
    }

    /// Returns the next `TetrominoType`.
    pub fn draw(&mut self) -> ::TetrominoType {

        // Remove random element from remaining tetromino types.
        let index = rand::thread_rng().gen_range(0, self.remaining.len());
        let result = self.remaining.remove(index);

        // Refill bag if it is empty
        if self.remaining.len() == 0 {
            self.remaining = Vec::from(&ALL_TETROMINO_TYPES as &[::TetrominoType])
        }

        result
    }
}