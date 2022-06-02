use crate::game::GameState;
use chargrid::{
    app::{App as ChargridApp, ControlFlow},
    input::{keys, Input, KeyboardInput},
    render::{ColModify, Frame, View, ViewCell, ViewContext},
};
use coord_2d::Size;
use direction::CardinalDirection;
use rgb24::Rgb24;
use std::time::Duration;

struct AppData {
    game_state: GameState,
}

impl AppData {
    fn new(screen_size: Size) -> Self {
        Self {
            game_state: GameState::new(screen_size),
        }
    }
    fn handle_input(&mut self, input: Input) {
        match input {
            Input::Keyboard(key) => match key {
                KeyboardInput::Left => self.maybe_move_player(CardinalDirection::West),
                KeyboardInput::Right => self.maybe_move_player(CardinalDirection::East),
                KeyboardInput::Up => self.maybe_move_player(CardinalDirection::North),
                KeyboardInput::Down => self.maybe_move_player(CardinalDirection::South),
                _ => (),
            },
            _ => (),
        }
    }
}

struct AppView {}

impl AppView {
    fn new() -> Self {
        Self {}
    }
}

impl<'a> View<&'a AppData> for AppView {
    fn view<F: Frame, C: ColModify>(
        &mut self,
        data: &'a AppData,
        context: ViewContext<C>,
        frame: &mut F,
    ) {
        let view_cell = ViewCell::new()
            .with_character('@')
            .with_foreground(Rgb24::new_grey(255));
        frame.set_cell_relative(data.player_coord, 0, view_cell, context);
    }
}

pub struct App {
    data: AppData,
    view: AppView,
}

impl App {
    pub fn new(screen_size: Size) -> Self {
        Self {
            data: AppData::new(screen_size),
            view: AppView::new(),
        }
    }
}
 impl ChargridApp for App {}
