use std::{
    fs::{self, File},
    io::BufWriter,
};

use actions::Action;
use image::codecs::jpeg::JpegEncoder;
use resetter::Resetter;

use crate::{actions::Actions, util::sleep_ms};

pub mod actions;
pub mod resetter;
pub mod screen_capture;
pub mod util;

fn main() {
    mki::Keyboard::CapsLock.bind(|_| {
        println!("Caps lock was pressed, giving up!");
        std::process::exit(0);
    });

    // Create a UnicodeKeyboard instance

    // Type each character3
    // mki::Sequence::text("376166226").unwrap().send();
    //

    let windows = xcap::Window::all().unwrap();

    let minecraft = match windows.iter().find(|it| it.title() == "Minecraft") {
        Some(window) => window,
        None => return eprintln!("Couldn't find Minecraft window. Launch it and run this again."),
    };

    fs::create_dir_all("./output").expect("Couldn't create ./output folder.");

    let input = fs::read_to_string("./input/seeds.txt").expect("Couldn't read ./input/seeds.txt");
    let seeds = input.lines();

    let actions = Actions::default();

    for seed in seeds {
        println!("Checking seed {}", seed);

        sleep_ms(1000);

        let mut resetter = Resetter::new(&minecraft, seed);
        resetter.execute_actions(&actions.world_creation, 100);
        resetter.wait_for_load();

        sleep_ms(1000);

        resetter.run_commands();

        let image = minecraft.capture_image().expect("Couldn't capture screen.");
        let path = format!("./output/{}.jpg", seed);
        let writer = BufWriter::new(File::create(path).unwrap());
        let mut encoder = JpegEncoder::new_with_quality(writer, 50);
        encoder.encode_image(&image).expect("Couldn't save image.");

        resetter.execute_actions(&actions.game_quitting, 200);
        resetter.wait_for_load();

        println!("Done with seed!");
    }

    println!("Done!");
}
