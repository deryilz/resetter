use std::time::Instant;

use mki::{Keyboard, Mouse};
use xcap::Window;

use image::codecs::jpeg::JpegEncoder;
use std::fs::File;
use std::io::BufWriter;

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
            Action::RecordColor(color) => self.record_color(*color),
            Action::WaitForColorChange => {
                let start = Instant::now(); // 30-second cap

                while start.elapsed().as_secs() < 30 {
                    if self.has_color_changed() {
                        return;
                    }
                    sleep_ms(100);
                }
            }
        }
    }

    pub fn record_color(&mut self, color: SampleColor) {
        let capture = ScreenCapture::new(self.window);
        self.last_color = Some(color);
        self.last_color_average = capture.average_of_color(color);
    }

    pub fn has_color_changed(&self) -> bool {
        let last_color = self.last_color.expect("Out-of-order color actions.");

        let capture = ScreenCapture::new(self.window);
        let last_avg = self.last_color_average;
        let new_avg = capture.average_of_color(last_color);

        // println!("{:?} change: {:?} {:?}", last_color, last_avg, new_avg);
        match (last_avg, new_avg) {
            (None, None) => false,
            (Some(avg1), Some(avg2)) if dist(avg1, avg2) < 80.0 => false,
            _ => true,
        }
    }

    pub fn wait_for_load(&mut self) {
        self.execute_action(&Action::RecordColor(SampleColor::White));
        self.execute_action(&Action::WaitForColorChange);
        sleep_ms(300);
    }

    pub fn save_screenshot(&self, path: &str) {
        let image = self
            .window
            .capture_image()
            .expect("Couldn't capture screen.");
        let writer = BufWriter::new(File::create(path).unwrap());

        JpegEncoder::new_with_quality(writer, 50)
            .encode_image(&image)
            .expect("Couldn't save image.");
    }

    // pub fn run_overworld_commands(&mut self) {
    //     // self.run_command("/gamerule sendcommandfeedback false");
    //     self.run_command("/setblock ~ ~69 ~ barrier");
    //     self.run_command("/tp ~ ~70 ~ 0 180");
    //     // self.run_command("/effect @s night_vision 10000 20 true");
    //     // self.run_command("/locate buriedtreasure");
    //     self.run_command(&format!("/me {}", self.seed));
    //     sleep_ms(300);
    // }

    // pub fn run_enter_nether_commands(&mut self) {
    //     self.run_command("/tp ~ 50 ~50");
    //     self.run_command("/setblock ~ ~ ~ portal");
    // }

    // pub fn run_nether_commands(&mut self) {
    //     for bad_block in ["lava", "netherrack"] {
    //         self.run_command(&format!(
    //             "/fill ~-15 ~ ~-15 ~15 ~25 ~15 air 0 replace {}",
    //             bad_block
    //         ));
    //     }

    //     self.run_command("/setblock ~ ~20 ~ barrier");
    //     self.run_command("/tp ~ ~21 ~ 0 180");
    //     self.run_command("/locate ruinedportal");
    //     self.run_command(&format!("/me {}", self.seed));
    // }

    pub fn run_command(&mut self, text: &str) {
        self.execute_actions(
            &[
                Action::Key(Keyboard::Enter),
                Action::Key(Keyboard::BackSpace),
                Action::WriteText(text.to_string()),
                Action::Key(Keyboard::Enter),
            ],
            250,
        );
    }
}
