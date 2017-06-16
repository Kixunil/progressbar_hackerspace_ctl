extern crate progressbar_hackerspace_ctl;

use progressbar_hackerspace_ctl::lights::Lights;
use std::env;

fn usage(zeroth: &str) -> ! {
    println!("Usage: {} ROOM LIGHT STATE", zeroth);
    println!();
    println!("ROOM: main | lab");
    println!("LIGHT[main]: front | back");
    println!("LIGHT[lab]: left | right");
    println!("STATE: on | off");

    std::process::exit(1);
}

fn main() {
    let mut args = env::args();
    let zeroth = args.next().expect("Not even zeroth argument given!");

    let room = args.next().unwrap_or_else(|| usage(&zeroth));
    let light = args.next().unwrap_or_else(|| usage(&zeroth));
    let state = args.next().unwrap_or_else(|| usage(&zeroth));

    if args.next().is_some() {
        println!("Warning: superfluous argument(s)");
    }

    let mut lights = Lights::new().unwrap();

    match (&room as &str, &light as &str, &state as &str) {
        ("main", "front", "on") => lights.main_room().front_light().turn_on(),
        ("main", "front", "off") => lights.main_room().front_light().turn_off(),
        ("main", "back", "on") => lights.main_room().back_light().turn_on(),
        ("main", "back", "off") => lights.main_room().back_light().turn_off(),
        ("lab", "left", "on") => lights.lab().left_light().turn_on(),
        ("lab", "left", "off") => lights.lab().left_light().turn_off(),
        ("lab", "right", "on") => lights.lab().right_light().turn_on(),
        ("lab", "right", "off") => lights.lab().right_light().turn_off(),
        _ => {
            println!("Error: Unknown combination of argumets.");
            std::process::exit(1);
        },
    }
}
