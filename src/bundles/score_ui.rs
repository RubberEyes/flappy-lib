use bevy::color::palettes::tailwind::GRAY_700;
use bevy::ecs::bundle::Bundle;
use bevy::prelude::{Node, Text, TextColor, TextFont, TextLayout};
use bevy::ui::{percent, px};

use crate::components::ScoreText;

#[derive(Bundle)]
pub struct ScoreUi {
    node: Node,
    text: Text,
    text_layout: TextLayout,
    text_font: TextFont,
    text_color: TextColor,
    score_text: ScoreText,
}

impl ScoreUi {
    pub fn new() -> Self {
        Self {
            node: Node {
                width: percent(100f32),
                margin: px(20f32).top(),
                ..Default::default()
            },
            text: Text::new("0"),
            text_layout: TextLayout::new_with_justify(bevy::text::Justify::Center),
            text_font: TextFont::from_font_size(33f32),
            text_color: TextColor(GRAY_700.into()),
            score_text: ScoreText,
        }
    }
}
