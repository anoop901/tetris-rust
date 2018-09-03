extern crate colored;
extern crate tetris;
#[macro_use] extern crate prettytable;

// use colored::*;
use std::io::BufRead;

// fn tetromino_type_to_color(tt: &tetris::TetrominoType) -> colored::Color {
//     match *tt {
//         tetris::TetrominoType::I => colored::Color::Cyan,
//         tetris::TetrominoType::J => colored::Color::Blue,
//         tetris::TetrominoType::L => colored::Color::BrightRed,
//         tetris::TetrominoType::O => colored::Color::Yellow,
//         tetris::TetrominoType::S => colored::Color::Green,
//         tetris::TetrominoType::T => colored::Color::Magenta,
//         tetris::TetrominoType::Z => colored::Color::Red,
//     }
// }


fn print_game_state(gs: &tetris::GameState) {

    let hold_display = render_hold_display(gs);
    let matrix_display = render_matrix_display(gs);
    let piece_queue_display = render_next_preview(gs);

    let mut table = table![
        [hold_display, matrix_display, piece_queue_display]
    ];

    table.set_format(*prettytable::format::consts::FORMAT_CLEAN);

    table.printstd();
}

fn render_hold_display(gs: &tetris::GameState) -> prettytable::Table {
    let mut hold_table = table![
        ["HOLD:"],
        [table![[
            if let Some(held) = gs.held() {
                render_tetromino(&held)
            } else {
                String::from("        \n        \n")
            }
        ]]]
    ];

    hold_table.set_format(*prettytable::format::consts::FORMAT_CLEAN);

    hold_table
}

fn render_matrix_display(gs: &tetris::GameState) -> prettytable::Table {
    let mut squares_to_print = gs.placed_squares().clone();

    let falling_tetromino = gs.falling_tetromino();

    for (mino_col, mino_row) in falling_tetromino.minoes().iter() {
        squares_to_print[*mino_col as usize][*mino_row as usize]
            = Some(falling_tetromino.ttype().clone());
    }

    let mut matrix_display: String = String::new();

    for ii in 0..tetris::MATRIX_HEIGHT - 2 {
        let i = tetris::MATRIX_HEIGHT - 3 - ii;
        matrix_display.push_str(" ");
        for j in 0..tetris::MATRIX_WIDTH {
            let c = if let Some(/*ref tt*/_) = squares_to_print[j][i] {
                format!("{}", "▣ "/*.color(tetromino_type_to_color(&tt))*/)
            } else { format!("{}",  "· "/*.white()*/) };
            matrix_display.push_str(&c);
        }
        matrix_display.push_str("\n");
    }

    table![[matrix_display]]
}

fn render_next_preview(gs: &tetris::GameState) -> prettytable::Table {

    let next_preview = gs.next_preview();

    let next = next_preview[0].clone();

    let mut subsequent_table = prettytable::Table::new();
    for tt in next_preview.iter().skip(1) {
        subsequent_table.add_row(row![render_tetromino(&tt)]);
    }

    let mut next_preview_table = table![
        [table![[render_tetromino(&next)]]],
        [subsequent_table]
    ];

    next_preview_table.set_format(*prettytable::format::consts::FORMAT_NO_BORDER);

    let mut next_preview_table_with_label = table![
        ["NEXT:"],
        [next_preview_table]
    ];

    next_preview_table_with_label.set_format(*prettytable::format::consts::FORMAT_CLEAN);

    next_preview_table_with_label
}

fn render_tetromino(tt: &tetris::TetrominoType) -> String {

    let mut squares_to_print: Vec<Vec<bool>> = vec![vec![false; 2]; 4];
    for (offset_col, offset_row) in tetris::tetromino_data::tetromino_shape_from(tt) {
        squares_to_print[(*offset_col + 1) as usize][*offset_row as usize] = true;
    }

    let mut tetromino_display = String::new();

    for ii in 0..2 {
        let i = 1 - ii;
        for j in 0..4 {
            let c = if squares_to_print[j][i] {
                format!("{}", "▣ "/*.color(tetromino_type_to_color(&tt))*/)
            } else { format!("{}",  "  "/*.white()*/) };
            tetromino_display.push_str(&c);
        }

        tetromino_display.push_str("\n");
    }

    tetromino_display
}

fn main() {

    let stdin = std::io::stdin();
    let mut gs = tetris::GameState::new();

    print_game_state(&gs);
    println!("");
    // TODO: proper error handling
    for line in stdin.lock().lines().map(|l| l.unwrap()) {
        match line.as_str() {
            "" => { gs.apply_gravity(); }
            "l" => { gs.move_left(); }
            "r" => { gs.move_right(); }
            "rl" => { gs.rotate_left(); }
            "rr" => { gs.rotate_right(); }
            "hd" => { gs.hard_drop(); }
            "h" => { gs.hold(); }
            _ => { println!("unknown command") }
        }
        print_game_state(&gs);
        println!("");
    }
}