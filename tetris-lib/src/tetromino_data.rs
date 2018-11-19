
type TetrominoShape = [(isize, isize); 4];

const I_SHAPE: TetrominoShape = [(-1, 0), (0, 0), (1, 0), (2, 0)];
const J_SHAPE: TetrominoShape = [(-1, 1), (-1, 0), (0, 0), (1, 0)];
const L_SHAPE: TetrominoShape = [(-1, 0), (0, 0), (1, 0), (1, 1)];
const O_SHAPE: TetrominoShape = [(0, 0), (0, 1), (1, 0), (1, 1)];
const S_SHAPE: TetrominoShape = [(-1, 0), (0, 0), (0, 1), (1, 1)];
const T_SHAPE: TetrominoShape = [(-1, 0), (0, 0), (0, 1), (1, 0)];
const Z_SHAPE: TetrominoShape = [(-1, 1), (0, 1), (0, 0), (1, 0)];

type TetrominoOffsetData = [[(isize, isize); 4]; 5];

const JLSTZ_OFFSET_DATA: TetrominoOffsetData = [
    [(0, 0), (0, 0), (0, 0), (0, 0)],
    [(0, 0), (1, 0), (0, 0), (-1, 0)],
    [(0, 0), (1, -1), (0, 0), (-1, -1)],
    [(0, 0), (0, 2), (0, 0), (0, 2)],
    [(0, 0), (1, 2), (0, 0), (-1, 2)],
];

const I_OFFSET_DATA: TetrominoOffsetData = [
    [(0, 0), (-1, 0), (-1, 1), (0, 1)],
    [(-1, 0), (0, 0), (1, 1), (0, 1)],
    [(2, 0), (0, 0), (-2, 1), (0, 1)],
    [(-1, 0), (0, 1), (1, 0), (0, -1)],
    [(2, 0), (0, -2), (-2, 0), (0, 2)],
];

const O_OFFSET_DATA: TetrominoOffsetData = [
    [(0, 0), (0, -1), (-1, -1), (-1, 0)],
    [(0, 0), (0, -1), (-1, -1), (-1, 0)],
    [(0, 0), (0, -1), (-1, -1), (-1, 0)],
    [(0, 0), (0, -1), (-1, -1), (-1, 0)],
    [(0, 0), (0, -1), (-1, -1), (-1, 0)],
];

// FIXME: maybe make this a "From" trait implementation
pub fn tetromino_shape_from(tt: &::TetrominoType) -> &'static TetrominoShape {
    match tt {
        ::TetrominoType::I => &I_SHAPE,
        ::TetrominoType::J => &J_SHAPE,
        ::TetrominoType::L => &L_SHAPE,
        ::TetrominoType::O => &O_SHAPE,
        ::TetrominoType::S => &S_SHAPE,
        ::TetrominoType::T => &T_SHAPE,
        ::TetrominoType::Z => &Z_SHAPE,
    }
}

pub fn tetromino_offset_data_from(tt: &::TetrominoType) -> &'static TetrominoOffsetData {
    match tt {
        ::TetrominoType::I => &I_OFFSET_DATA,
        ::TetrominoType::J => &JLSTZ_OFFSET_DATA,
        ::TetrominoType::L => &JLSTZ_OFFSET_DATA,
        ::TetrominoType::O => &O_OFFSET_DATA,
        ::TetrominoType::S => &JLSTZ_OFFSET_DATA,
        ::TetrominoType::T => &JLSTZ_OFFSET_DATA,
        ::TetrominoType::Z => &JLSTZ_OFFSET_DATA,
    }
}