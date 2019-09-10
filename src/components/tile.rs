#[macro_use]
use seed::prelude::*;

use super::super::Msg;

pub fn tile(value: i32) -> Node<Msg> {
    return div![
        style!{
            "display" => "inline-flex",
            "height" => "100%",
            "width" => "100%",
            "align-items" => "center",
            "justify-content" => "center",
            "background-color" => match value {
                0 => "#CCC1B4",
                2 => "#EEE4DA",
                4 => "#EDE0C8",
                8 => "#f2b179",
                16 => "#f59563",
                32 => "#f67c5f",
                64 => "#f65e3b",
                128 => "#edcf72",
                256 => "#edcc61",
                512 => "#edc850",
                1024 => "#edc53f",
                2048 => "#edc22e",
                _ => "red",
            },
            "border-radius" => "8px",
            "font-size" => "55px",
        },
        value.to_string()
    ]
}