use std::fs;

use itertools::Itertools;
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

    sleep_ms(1000);

    // Create a UnicodeKeyboard instance

    // Type each character3
    // mki::Sequence::text("376166226").unwrap().send();
    //

    let windows = xcap::Window::all().unwrap();

    let minecraft = windows
        .iter()
        .find(|it| it.title() == "Minecraft")
        .expect("Couldn't find Minecraft window. Launch it and run this again.");

    fs::create_dir_all("./output").expect("Couldn't create ./output folder.");

    let input = include_str!("../input/seeds.txt");

    let actions = Actions::default();

    for line in input.lines() {
        let split = line.split_whitespace().collect_vec();
        let seed = split[0];
        let coords: (i32, i32, i32) = split[2..]
            .iter()
            .map(|str| str.parse().unwrap())
            .collect_tuple()
            .unwrap();

        println!("Checking seed {}", seed);

        let mut resetter = Resetter::new(&minecraft, seed);
        resetter.execute_actions(&actions.world_creation, 150);
        resetter.wait_for_load();

        let tp_command = format!("/tp {} {} {} 0 180", coords.0, coords.1 + 1, coords.2);
        resetter.run_command(&tp_command);
        resetter.save_screenshot(&format!("./output/{}-1.jpg", seed));

        resetter.execute_actions(&actions.game_quitting, 250);
        resetter.wait_for_load();

        sleep_ms(500);

        println!("Done with seed!");
    }

    println!("Done!");
}
