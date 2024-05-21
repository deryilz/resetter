use mki::Keyboard;

use crate::screen_capture::SampleColor;

#[derive(Debug, Clone)]
pub enum Action {
    Key(Keyboard),
    RecordColor(SampleColor),
    WriteText(String),
    PasteSeed,
    WaitForColorChange,
}

#[derive(Debug)]
pub struct Actions {
    pub world_creation: Vec<Action>,
    pub game_quitting: Vec<Action>,
}

impl Default for Actions {
    fn default() -> Self {
        use Action::*;
        use Keyboard::*;

        Self {
            world_creation: vec![
                Key(Up),
                Key(Up),
                RecordColor(SampleColor::Green),
                Key(Enter),
                WaitForColorChange,
                Key(Up),
                Key(Up),
                RecordColor(SampleColor::Green),
                Key(Enter),
                WaitForColorChange,
                Key(Right),
                Key(Right),
                Key(Down),
                Key(Enter),
                Key(Down),
                Key(Enter),
                Key(Up),
                Key(Enter),
                Key(Enter),
                Key(Down),
                Key(Down),
                Key(Down),
                Key(Down),
                Key(Down),
                Key(Down),
                Key(Down),
                Key(BackSpace),
                PasteSeed,
                Key(Escape),
                Key(Down),
                Key(Down),
                Key(Down),
                Key(Enter),
                Key(Left),
                Key(Up),
                RecordColor(SampleColor::Green),
                Key(Enter),
                WaitForColorChange,
            ],
            game_quitting: vec![
                Key(Escape),
                Key(Right),
                Key(Right),
                Key(Left),
                RecordColor(SampleColor::Green),
                Key(Enter),
                WaitForColorChange,
            ],
        }
    }
}
