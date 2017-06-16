extern crate progressbar_hackerspace_ctl;

use progressbar_hackerspace_ctl::lights::{Lights, EnumerateLights};
use std::env;

fn usage(zeroth: &str) -> ! {
    println!("Usage: {} STATE [ROOM]", zeroth);
    println!();
    println!("STATE: on | off");
    println!("ROOM: main | lab");

    std::process::exit(1);
}

fn main() {
    let mut args = env::args();
    let zeroth = args.next().expect("Not even zeroth argument given!");

    let state = args.next().unwrap_or_else(|| usage(&zeroth));
    let room = args.next();
    let room = room.as_ref().map(AsRef::as_ref);

    if room.is_some() && args.next().is_some() {
        println!("Warning: superfluous argument(s)");
    }

    let mut lights = Lights::new().unwrap();

    match (&state as &str, room) {
        ("on", None) => lights.enumerate_lights(|mut light| light.turn_on()),
        ("off", None) => lights.enumerate_lights(|mut light| light.turn_off()),
        ("on", Some("main")) => lights.main_room().enumerate_lights(|mut light| light.turn_on()),
        ("off", Some("main")) => lights.main_room().enumerate_lights(|mut light| light.turn_off()),
        ("on", Some("lab")) => lights.lab().enumerate_lights(|mut light| light.turn_on()),
        ("off", Some("lab")) => lights.lab().enumerate_lights(|mut light| light.turn_off()),
        _ => {
            println!("Error: Unknown combination of argumets.");
            std::process::exit(1);
        },
    }
}
