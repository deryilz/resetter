use mki::Keyboard;
use xcap::Window;

use crate::actions::Action;
use crate::screen_capture::{SampleColor, ScreenCapture};
use crate::util::*;

#[derive(Debug)]
pub struct Resetter<'a> {
    window: &'a Window,
    seed: &'a str,
    last_color: Option<SampleColor>,
    last_color_average: Option<(u32, u32)>,
}

impl<'a> Resetter<'a> {
    pub fn new(window: &'a Window, seed: &'a str) -> Self {
        Self {
            window,
            seed,
            last_color: None,
            last_color_average: None,
        }
    }

    pub fn execute_actions(&mut self, actions: &[Action], interval: u64) {
        for action in actions {
            self.execute_action(action);
            sleep_ms(interval);
        }
    }

    pub fn execute_action(&mut self, action: &Action) {
        match action {
            Action::Key(key) => key.click(),
            Action::WriteText(text) => write_text(text),
            Action::PasteSeed => write_text(self.seed),
            Action::RecordColor(color) => {
                let capture = ScreenCapture::new(self.window);

                self.last_color = Some(*color);
                self.last_color_average = capture.average_of_color(*color);
            }
            Action::WaitForColorChange => loop {
                let last_color = self.last_color.expect("Out-of-order color actions.");

                let capture = ScreenCapture::new(self.window);
                let last_avg = self.last_color_average;
                let new_avg = capture.average_of_color(last_color);

                // println!("{:?} change: {:?} {:?}", last_color, last_avg, new_avg);
                match (last_avg, new_avg) {
                    (None, None) => {}
                    (Some(avg1), Some(avg2)) if dist(avg1, avg2) < 80.0 => {}
                    _ => break,
                };
            },
        }
    }

    pub fn wait_for_load(&mut self) {
        sleep_ms(200);
        self.execute_actions(
            &[
                Action::RecordColor(SampleColor::White),
                Action::WaitForColorChange,
            ],
            0,
        );
        sleep_ms(300);
    }

    pub fn run_commands(&mut self) {
        self.run_command("/gamerule sendcommandfeedback false");
        self.run_command("/setblock ~ ~69 ~ barrier");
        self.run_command("/tp ~ ~70 ~ 0 180");
        self.run_command("/effect @s night_vision 10000 20");
        self.run_command(&format!("/me {}", self.seed));
        sleep_ms(300);
    }
    

    pub fn run_command(&mut self, text: &str) {
        self.execute_actions(
            &[
                Action::Key(Keyboard::T),
                Action::Key(Keyboard::BackSpace),
                Action::WriteText(text.to_string()),
                Action::Key(Keyboard::Enter),
            ],
            250,
        );
    }
}
