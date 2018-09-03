pub mod bag;
pub mod tetromino_data;

// TODO: make these constants configurable at runtime

/// The number of pieces that the player can see in advance.
pub const NEXT_PREVIEW_LENGTH: usize = 5;

/// Width of the play field.
pub const MATRIX_WIDTH: usize = 10;
/// Height of the play field (both visible and hidden parts).
pub const MATRIX_HEIGHT: usize = 22;

/// Identifies one of the 7 types of tetrominoes
#[derive(Clone, Debug)]
pub enum TetrominoType { I, O, T, J, L, S, Z, }

/// Describes a tetromino of some type at some position and orientation on the
/// matrix.
#[derive(Debug, Clone)]
pub struct Tetromino {
    ttype: TetrominoType,
    center: (isize, isize),
    orientation: u32
}

impl Tetromino {

    /// Returns a vector of the positions of each mino in this tetromino.
    pub fn minoes(&self) -> Vec<(isize, isize)> {

        let shape = tetromino_data::tetromino_shape_from(&self.ttype);
        let center = self.center;

        shape.iter().map(|mino_offset| {

            let rotated_offset = match self.orientation {
                0 => (mino_offset.0, mino_offset.1),
                1 => (mino_offset.1, -mino_offset.0),
                2 => (-mino_offset.0, -mino_offset.1),
                3 => (-mino_offset.1, mino_offset.0),
                _ => (mino_offset.0, mino_offset.1),
            };

            (
                center.0 + rotated_offset.0,
                center.1 + rotated_offset.1
            )
        }).collect()
    }
}

impl Tetromino {

    /// Returns the grid coordinates of the center of this tetromino.
    /// 
    /// The first index identifies the row, where 0 is the bottom row. The second
    /// index identifies the column, where 0 is the leftmost column.
    pub fn center(&self) -> (isize, isize) {
        return self.center;
    }

    /// Returns the type of tetromino.
    pub fn ttype(&self) -> &TetrominoType {
        return &self.ttype;
    }

}

/// Describes the state of a Tetris game.
///
/// Rows are indexed from bottom to top, where the bottom row is indexed as 0.
/// Columns are indexed from left to right, where the left-most row is indexed
/// as 0. Coordinates are specified as (col, row).
#[derive(Debug)]
pub struct GameState {
    placed_squares: Vec<Vec<Option<TetrominoType>>>,

    falling_tetromino: Tetromino,

    /// The bag for determining the next tetrominoes.
    bag: bag::Bag,

    /// The next pieces that will drop.
    next_preview: Vec<TetrominoType>,

    /// The held tetromino, if any
    held: Option<TetrominoType>
}

impl GameState {

    /// Create a new `GameState` representing the initial state of a tetris
    /// game.
    pub fn new() -> Self {

        let mut bag = bag::Bag::new();

        let initial_falling_tetromino_type = bag.draw();
        let initial_next_preview =
            (0..NEXT_PREVIEW_LENGTH)
            .map(|_| bag.draw())
            .collect();

        GameState {
            placed_squares: vec![vec![None; MATRIX_HEIGHT]; MATRIX_WIDTH],
            falling_tetromino: Tetromino {
                ttype: initial_falling_tetromino_type,
                center: (4, (MATRIX_HEIGHT - 2) as isize),
                orientation: 0
            },
            bag: bag,
            next_preview: initial_next_preview,
            held: None
        }
    }

    // Getters

    /// Returns a representation of all the squares that have previously been
    /// placed the matrix. It is expressed as a `Vec<Vec<Option<TetrominoType>>>`
    /// where:
    ///
    /// - `placed_squares[i][j] == Some(tt)` indicates that there has been a
    ///   square placed at coordinates (`i`, `j`), which came from a tetromino
    ///   of type `tt`.
    /// - `placed_squares[i][j] == None` indicates that there has not been any
    ///   square placed at coordinates (`i`, `j`).
    pub fn placed_squares(&self) -> &Vec<Vec<Option<TetrominoType>>> {
        return &self.placed_squares;
    }

    pub fn falling_tetromino(&self) -> &Tetromino {
        return &self.falling_tetromino
    }

    pub fn next_preview(&self) -> &[TetrominoType] {
        return &self.next_preview.as_slice();
    }

    pub fn held(&self) -> &Option<TetrominoType> {
        return &self.held;
    }

    // Actions that can be made by the player

    /// Moves the currently falling piece down, due to gravity. Returns `true`
    /// if the piece was moved successfully, or `false` if it hit the floor.
    pub fn apply_gravity(&mut self) -> bool {

        let new_center = (self.falling_tetromino.center.0,
                            self.falling_tetromino.center.1 - 1);
        let orientation = self.falling_tetromino.orientation;
        self.move_tetromino_if_fits(new_center, orientation)
    }

    pub fn move_left(&mut self) -> bool {
        let new_center = (self.falling_tetromino.center.0 - 1,
                            self.falling_tetromino.center.1);
        let orientation = self.falling_tetromino.orientation;
        self.move_tetromino_if_fits(new_center, orientation)
    }

    pub fn move_right(&mut self) -> bool {
        // try to move tetromino right by 1
        let new_center = (self.falling_tetromino.center.0 + 1,
                          self.falling_tetromino.center.1);
        let orientation = self.falling_tetromino.orientation;
        self.move_tetromino_if_fits(new_center, orientation)
    }

    pub fn rotate_left(&mut self) {
        let old_orientation = self.falling_tetromino.orientation;
        self.rotate_to_orientation((old_orientation + 3) % 4);
    }

    pub fn rotate_right(&mut self) {
        let old_orientation = self.falling_tetromino.orientation;
        self.rotate_to_orientation((old_orientation + 1) % 4);
    }

    fn rotate_to_orientation(&mut self, new_orientation: u32) {
        let old_orientation = self.falling_tetromino.orientation;
        for offset_data in tetromino_data::tetromino_offset_data_from(&self.falling_tetromino.ttype) {

            let offset = (
                offset_data[old_orientation as usize].0 - offset_data[new_orientation as usize].0,
                offset_data[old_orientation as usize].1 - offset_data[new_orientation as usize].1
            );

            let candidate_tetromino = Tetromino {
                ttype: self.falling_tetromino.ttype.clone(),
                center: (
                    self.falling_tetromino.center.0 + offset.0,
                    self.falling_tetromino.center.1 + offset.1,
                ),
                orientation: new_orientation,
            };

            if self.tetromino_fits(&candidate_tetromino) {
                self.falling_tetromino = candidate_tetromino;
                return;
            }
        }
    }

    pub fn hard_drop(&mut self) {
        while self.apply_gravity() {}
        self.lock_piece();
    }

    pub fn hold(&mut self) {
        // TODO: this shouldn't be allowed twice in a row
        let new_held = self.falling_tetromino.ttype.clone();
        if let Some(ref old_held) = self.held {
            self.falling_tetromino = Tetromino {
                ttype: old_held.clone(),
                center: (4, (MATRIX_HEIGHT - 2) as isize),
                orientation: 0
            }
        } else {
            Some(self.falling_tetromino.ttype.clone());
            self.spawn_next_piece();
        }
        self.held = Some(new_held);
    }

    /// Locks the currently falling tetromino on the matrix and spawns the next
    /// tetromino.
    pub fn lock_piece(&mut self) {

        // place tetromino squares on matrix
        self.falling_tetromino.minoes().iter().for_each(|mino_position| {
            self.placed_squares[mino_position.0 as usize][mino_position.1 as usize] = Some(self.falling_tetromino.ttype.clone());
        });

        self.spawn_next_piece();

        // TODO: clear lines
        // TODO: detect game over
    }

    fn spawn_next_piece(&mut self) {
        
        // add a new item to the end of the preview
        self.next_preview.push(self.bag.draw());

        // FIXME: reuse this from initialization code
        // spawn the next tetromino
        self.falling_tetromino = Tetromino {
            ttype: self.next_preview.remove(0),
            center: (4, (MATRIX_HEIGHT - 2) as isize),
            orientation: 0
        }

    }

    // Helpers

    fn move_tetromino_if_fits(&mut self, new_center: (isize, isize), new_orientation: u32) -> bool {
        let candidate = Tetromino {
            ttype: self.falling_tetromino.ttype.clone(),
            center: new_center,
            orientation: new_orientation
        };
        let fits = self.tetromino_fits(&candidate);

        if fits {
            self.falling_tetromino = candidate;
        }

        fits
    }

    fn tetromino_fits(&self, t: &Tetromino) -> bool {
        t.minoes().iter().all(|mino_position| {
            mino_position.0 >= 0 &&
            mino_position.0 < (MATRIX_WIDTH as isize) &&
            mino_position.1 >= 0 &&
            mino_position.1 < (MATRIX_HEIGHT as isize) &&
            self.placed_squares[mino_position.0 as usize][mino_position.1 as usize].is_none()
        })
    }
}