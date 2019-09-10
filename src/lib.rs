#[macro_use]
extern crate seed;
use seed::prelude::*;
mod game;
use game::{Direction, Game, GameStatus};

mod components;
use components::tile::tile;

// Model

struct Model {
    game: Game,
}

impl Default for Model {
    fn default() -> Self {
        Self {
            game: Game {
                field: vec![vec![0; 4]; 4],
                status: GameStatus::InProgress,
            },
        }
    }
}


// Update 

#[derive(Clone)]
pub enum Msg {
    Move(Direction),
    Dummy,
}

fn window_events(model: &Model) -> Vec<seed::events::Listener<Msg>> {
    let mut result = Vec::new();
    
    result.push(keyboard_ev("keydown", |ev| {
        match ev.key_code() {
            37 => Msg::Move(Direction::Left),
            38 => Msg::Move(Direction::Up),
            39 => Msg::Move(Direction::Right),
            40 => Msg::Move(Direction::Down),
            _ => Msg::Dummy,
        }
    }));
    
    result
}

fn update(msg: Msg, model: &mut Model, _: &mut impl Orders<Msg>) {
    match msg {
        Msg::Move(direction) => model.game.make_move(direction),
        Msg::Dummy => {},
    }
}

// View

fn view(model: &Model) -> impl View<Msg> {
    div![
        style!{
            "display" => "grid",
            "grid-template-rows" => format!("repeat({num_rows}, 1fr)", num_rows = model.game.field.len()),
            "grid-template-columns" => format!("repeat({num_cols}, 1fr)", num_cols = model.game.field[0].len()),
            "grid-gap" => "13px",
            "border-radius" => "11px",
            "width" => "500px",
            "height" => "500px",
            "background-color" => "#BBAD9F",
            "border" => "13px solid #BBAD9F"
        },
        model.game.field.iter().map(|row|
            row.iter().map(|column| tile(*column))
        )
    ]
}


#[wasm_bindgen(start)]
pub fn render() {
    seed::App::build(|_, _| Model::default(), update, view)
        .window_events(window_events)
        .finish()
        .run();
}