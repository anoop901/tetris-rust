extern crate cfg_if;
extern crate wasm_bindgen;
extern crate web_sys;
extern crate tetris;

mod utils;

use cfg_if::cfg_if;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use std::sync::{Arc,Mutex};

cfg_if! {
    // When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
    // allocator.
    if #[cfg(feature = "wee_alloc")] {
        extern crate wee_alloc;
        #[global_allocator]
        static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;
    }
}

#[wasm_bindgen]
pub fn main() -> Result<(), JsValue> {
    let window = web_sys::window().expect("no global window");
    let document = window.document().expect("should have a document on window");
    let body = document.body().expect("document should have a body");

    let svg = document.create_element_ns(Some("http://www.w3.org/2000/svg"), "svg")?
        .dyn_into::<web_sys::SvgsvgElement>()?;

    let gui_width = 800;
    let gui_height = 800;

    let matrix_left_px = 250;
    let matrix_top_px = 70;
    let matrix_width_px = 300;
    let matrix_height_px = 660;

    (svg.as_ref() as &web_sys::Element)
        .set_attribute("viewBox", &format!("0 0 {} {}", gui_width, gui_height))?;

    (svg.as_ref() as &web_sys::SvgElement).style().set_property("display", "block")?;
    (svg.as_ref() as &web_sys::SvgElement).style().set_property("position", "absolute")?;
    (svg.as_ref() as &web_sys::SvgElement).style().set_property("top", "0%")?;
    (svg.as_ref() as &web_sys::SvgElement).style().set_property("left", "0%")?;
    (svg.as_ref() as &web_sys::SvgElement).style().set_property("width", "100%")?;
    (svg.as_ref() as &web_sys::SvgElement).style().set_property("height", "100%")?;

//    let background_rect = document.create_element_ns(Some("http://www.w3.org/2000/svg"), "rect")?
//        .dyn_into::<web_sys::SvgRectElement>()?;
//
//    (background_rect.as_ref() as &web_sys::Element).set_attribute("x", "0")?;
//    (background_rect.as_ref() as &web_sys::Element).set_attribute("y", "0")?;
//    (background_rect.as_ref() as &web_sys::Element).set_attribute("width", &gui_width.to_string())?;
//    (background_rect.as_ref() as &web_sys::Element).set_attribute("height", &gui_height.to_string())?;
//    (background_rect.as_ref() as &web_sys::Element).set_attribute("fill", "#e0e0e0")?;
//
//    (svg.as_ref() as &web_sys::Node).append_child(background_rect.as_ref())?;

    let matrix_background_rect = document.create_element_ns(Some("http://www.w3.org/2000/svg"), "rect")?
        .dyn_into::<web_sys::SvgRectElement>()?;

    (matrix_background_rect.as_ref() as &web_sys::Element).set_attribute("x", &matrix_left_px.to_string())?;
    (matrix_background_rect.as_ref() as &web_sys::Element).set_attribute("y", &matrix_top_px.to_string())?;
    (matrix_background_rect.as_ref() as &web_sys::Element).set_attribute("width", &matrix_width_px.to_string())?;
    (matrix_background_rect.as_ref() as &web_sys::Element).set_attribute("height", &matrix_height_px.to_string())?;
    (matrix_background_rect.as_ref() as &web_sys::Element).set_attribute("stroke-width", "3")?;
    (matrix_background_rect.as_ref() as &web_sys::Element).set_attribute("stroke", "#606060")?;
    (matrix_background_rect.as_ref() as &web_sys::Element).set_attribute("fill", "#c0c0c0")?;

    (svg.as_ref() as &web_sys::Node).append_child(matrix_background_rect.as_ref())?;

    for i in 1..tetris::game_state::MATRIX_WIDTH {

        let vertical_line = document.create_element_ns(Some("http://www.w3.org/2000/svg"), "line")?
            .dyn_into::<web_sys::SvgLineElement>()?;
        let x = matrix_left_px + i * matrix_width_px / tetris::game_state::MATRIX_WIDTH;

        (vertical_line.as_ref() as &web_sys::Element).set_attribute("x1", &x.to_string())?;
        (vertical_line.as_ref() as &web_sys::Element).set_attribute("y1", &matrix_top_px.to_string())?;
        (vertical_line.as_ref() as &web_sys::Element).set_attribute("x2", &x.to_string())?;
        (vertical_line.as_ref() as &web_sys::Element).set_attribute("y2", &(matrix_top_px + matrix_height_px).to_string())?;
        (vertical_line.as_ref() as &web_sys::Element).set_attribute("stroke-width", "1")?;
        (vertical_line.as_ref() as &web_sys::Element).set_attribute("stroke", "#a0a0a0")?;

        (svg.as_ref() as &web_sys::Node).append_child(vertical_line.as_ref())?;
    }

    for j in 1..tetris::game_state::MATRIX_HEIGHT {

        let vertical_line = document.create_element_ns(Some("http://www.w3.org/2000/svg"), "line")?
            .dyn_into::<web_sys::SvgLineElement>()?;
        let y = matrix_top_px + j * matrix_height_px / tetris::game_state::MATRIX_HEIGHT;

        (vertical_line.as_ref() as &web_sys::Element).set_attribute("x1", &matrix_left_px.to_string())?;
        (vertical_line.as_ref() as &web_sys::Element).set_attribute("y1", &y.to_string())?;
        (vertical_line.as_ref() as &web_sys::Element).set_attribute("x2", &(matrix_left_px + matrix_width_px).to_string())?;
        (vertical_line.as_ref() as &web_sys::Element).set_attribute("y2", &y.to_string())?;
        (vertical_line.as_ref() as &web_sys::Element).set_attribute("stroke-width", "1")?;
        (vertical_line.as_ref() as &web_sys::Element).set_attribute("stroke", "#a0a0a0")?;

        (svg.as_ref() as &web_sys::Node).append_child(vertical_line.as_ref())?;
    }

    let next_preview_background_rect = document.create_element_ns(Some("http://www.w3.org/2000/svg"), "rect")?
        .dyn_into::<web_sys::SvgRectElement>()?;

    (next_preview_background_rect.as_ref() as &web_sys::Element).set_attribute("x", "600")?;
    (next_preview_background_rect.as_ref() as &web_sys::Element).set_attribute("y", "100")?;
    (next_preview_background_rect.as_ref() as &web_sys::Element).set_attribute("width", "150")?;
    (next_preview_background_rect.as_ref() as &web_sys::Element).set_attribute("height", "500")?;
    (next_preview_background_rect.as_ref() as &web_sys::Element).set_attribute("stroke-width", "3")?;
    (next_preview_background_rect.as_ref() as &web_sys::Element).set_attribute("stroke", "#606060")?;
    (next_preview_background_rect.as_ref() as &web_sys::Element).set_attribute("fill", "#c0c0c0")?;

    (svg.as_ref() as &web_sys::Node).append_child(next_preview_background_rect.as_ref())?;

    let next_piece_highlight_rect = document.create_element_ns(Some("http://www.w3.org/2000/svg"), "rect")?
        .dyn_into::<web_sys::SvgRectElement>()?;

    (next_piece_highlight_rect.as_ref() as &web_sys::Element).set_attribute("x", "600")?;
    (next_piece_highlight_rect.as_ref() as &web_sys::Element).set_attribute("y", "100")?;
    (next_piece_highlight_rect.as_ref() as &web_sys::Element).set_attribute("width", "150")?;
    (next_piece_highlight_rect.as_ref() as &web_sys::Element).set_attribute("height", "100")?;
    (next_piece_highlight_rect.as_ref() as &web_sys::Element).set_attribute("stroke-width", "3")?;
    (next_piece_highlight_rect.as_ref() as &web_sys::Element).set_attribute("stroke", "#606060")?;
    (next_piece_highlight_rect.as_ref() as &web_sys::Element).set_attribute("fill", "#d0d0d0")?;

    (svg.as_ref() as &web_sys::Node).append_child(next_piece_highlight_rect.as_ref())?;

    let hold_piece_background_rect = document.create_element_ns(Some("http://www.w3.org/2000/svg"), "rect")?
        .dyn_into::<web_sys::SvgRectElement>()?;

    (hold_piece_background_rect.as_ref() as &web_sys::Element).set_attribute("x", "50")?;
    (hold_piece_background_rect.as_ref() as &web_sys::Element).set_attribute("y", "100")?;
    (hold_piece_background_rect.as_ref() as &web_sys::Element).set_attribute("width", "150")?;
    (hold_piece_background_rect.as_ref() as &web_sys::Element).set_attribute("height", "100")?;
    (hold_piece_background_rect.as_ref() as &web_sys::Element).set_attribute("stroke-width", "3")?;
    (hold_piece_background_rect.as_ref() as &web_sys::Element).set_attribute("stroke", "#606060")?;
    (hold_piece_background_rect.as_ref() as &web_sys::Element).set_attribute("fill", "#c0c0c0")?;

    (svg.as_ref() as &web_sys::Node).append_child(hold_piece_background_rect.as_ref())?;

    let placed_squares_g = document.create_element_ns(Some("http://www.w3.org/2000/svg"), "g")?
        .dyn_into::<web_sys::SvggElement>()?;
    (svg.as_ref() as &web_sys::Node).append_child(placed_squares_g.as_ref())?;

    (body.as_ref() as &web_sys::Node).append_child(svg.as_ref())?;

    let timed_game_state = Arc::new(Mutex::new(tetris::game_state::TimedGameState::new()));
    let timed_game_state_clone = timed_game_state.clone();


    let a = Closure::wrap(Box::new(move || {
        let window = web_sys::window().expect("no global window");
        let document = window.document().expect("should have a document on window");
        let mut timed_game_state = timed_game_state.lock().unwrap();
        timed_game_state.advance_time(40);

        // empty existing children
        while let Some(child) = (placed_squares_g.as_ref() as &web_sys::Node).first_child() {
            (placed_squares_g.as_ref() as &web_sys::Node).remove_child(&child).unwrap();
        }

        let placed_squares = timed_game_state.placed_squares();
        for (col, placed_squares_row) in placed_squares.iter().enumerate() {
            for (row, placed_square) in placed_squares_row.iter().enumerate() {
                if let Some(placed_square_type) = placed_square {

                    place_square_on_matrix(placed_square_type, col, row, &placed_squares_g, &document);
                }
            }
        }

        for (col, row) in timed_game_state.falling_tetromino().minoes() {
            place_square_on_matrix(timed_game_state.falling_tetromino().ttype(), col as usize, row as usize, &placed_squares_g, &document);
        }

        for (i, preview_piece_type) in timed_game_state.next_preview().iter().enumerate() {
            place_square(preview_piece_type, 600 + 75 - 15, 100 + 50 - 15 + i * 100, &placed_squares_g, &document);
        }

        if let Some(held_type) = timed_game_state.held() {
            place_square(held_type, 125 - 15, 100 + 50 - 15, &placed_squares_g, &document);
        }

    }) as Box<FnMut()>);
    window.set_interval_with_callback_and_timeout_and_arguments_0(
        a.as_ref().unchecked_ref(),
        40
    )?;
    a.forget();

    let timed_game_state = timed_game_state_clone;

    let a = Closure::wrap(Box::new(move |event: web_sys::KeyboardEvent| {
//        let window = web_sys::window().expect("no global window");

        if event.repeat() {
            return;
        }

        web_sys::console::log_1(&format!("keydown with code={}", event.code()).into());
        let mut timed_game_state = timed_game_state.lock().unwrap();
        match event.code().as_str() {
            "ArrowLeft" => { timed_game_state.move_left(); },
            "ArrowRight" => { timed_game_state.move_right(); },
            "ArrowUp" => { timed_game_state.rotate_right(); },
            "ArrowDown" => { timed_game_state.hard_drop(); }, // TODO: implement soft drop
            "Space" => { timed_game_state.hard_drop(); },
            "KeyZ" => { timed_game_state.rotate_left(); },
            "KeyX" => { timed_game_state.rotate_right(); },
            "KeyA" => { timed_game_state.hold(); }
            "ShiftLeft" => { timed_game_state.hold(); }
            "ShiftRight" => { timed_game_state.hold(); }
            _ => {},
        }
    }) as Box<FnMut(_)>);
    (document.as_ref() as &web_sys::EventTarget).add_event_listener_with_callback("keydown", a.as_ref().unchecked_ref())?;
    a.forget();

    Ok(())
}

const STROKE_WIDTH: usize = 3;

fn place_square_on_matrix(placed_square_type: &tetris::TetrominoType, col: usize, row: usize, placed_squares_g: &web_sys::SvggElement, document: &web_sys::Document) {

    place_square(placed_square_type, 250 + col * 30 + STROKE_WIDTH, 700 - row * 30 + STROKE_WIDTH, placed_squares_g, document);
}

fn place_square(placed_square_type: &tetris::TetrominoType, x: usize, y: usize, placed_squares_g: &web_sys::SvggElement, document: &web_sys::Document) {

    let placed_square_rect = document.create_element_ns(Some("http://www.w3.org/2000/svg"), "rect").unwrap()
        .dyn_into::<web_sys::SvgRectElement>().unwrap();
    let (stroke_color, fill_color) = tetromino_type_to_colors(placed_square_type);

    (placed_square_rect.as_ref() as &web_sys::Element).set_attribute("x", &x.to_string()).unwrap();
    (placed_square_rect.as_ref() as &web_sys::Element).set_attribute("y", &y.to_string()).unwrap();
    (placed_square_rect.as_ref() as &web_sys::Element).set_attribute("width", &(30 - 2 * STROKE_WIDTH).to_string()).unwrap();
    (placed_square_rect.as_ref() as &web_sys::Element).set_attribute("height", &(30 - 2 * STROKE_WIDTH).to_string()).unwrap();
    (placed_square_rect.as_ref() as &web_sys::Element).set_attribute("rx", &(STROKE_WIDTH).to_string()).unwrap();
    (placed_square_rect.as_ref() as &web_sys::Element).set_attribute("ry", &(STROKE_WIDTH).to_string()).unwrap();
    (placed_square_rect.as_ref() as &web_sys::Element).set_attribute("stroke-width", &(2 * STROKE_WIDTH).to_string()).unwrap();
    (placed_square_rect.as_ref() as &web_sys::Element).set_attribute("stroke", stroke_color).unwrap();
    (placed_square_rect.as_ref() as &web_sys::Element).set_attribute("fill", fill_color).unwrap();

    (placed_squares_g.as_ref() as &web_sys::Node).append_child(placed_square_rect.as_ref()).unwrap();
}

fn tetromino_type_to_colors(tt: &tetris::TetrominoType) -> (&str, &str) {
    match tt {
        tetris::TetrominoType::I => ("#00c0c0", "#40ffff"),
        tetris::TetrominoType::J => ("#0000c0", "#4040ff"),
        tetris::TetrominoType::L => ("#c06000", "#ffa040"),
        tetris::TetrominoType::O => ("#c0c000", "#ffff40"),
        tetris::TetrominoType::S => ("#00c000", "#40ff40"),
//        tetris::TetrominoType::T => ("#6000c0", "#a040ff"),
        tetris::TetrominoType::T => ("#c000c0", "#ff40ff"),
        tetris::TetrominoType::Z => ("#c00000", "#ff4040"),
    }
}
