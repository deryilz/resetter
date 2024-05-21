use std::fs;

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

    let input = fs::read_to_string("./input/seeds.txt").expect("Couldn't read ./input/seeds.txt");
    let seeds = input.lines();

    let actions = Actions::default();

    for seed in seeds {
        println!("Checking seed {}", seed);

        let mut resetter = Resetter::new(&minecraft, seed);
        resetter.execute_actions(&actions.world_creation, 150);
        resetter.wait_for_load();

        resetter.run_overworld_commands();
        resetter.save_screenshot(&format!("./output/{}-1.jpg", seed));

        resetter.run_enter_nether_commands();
        resetter.wait_for_load();
        
        sleep_ms(2500);

        resetter.run_nether_commands();
        resetter.save_screenshot(&format!("./output/{}-2.jpg", seed));

        resetter.execute_actions(&actions.game_quitting, 250);
        resetter.wait_for_load();

        sleep_ms(500);

        println!("Done with seed!");
    }

    println!("Done!");
}
