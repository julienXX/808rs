extern crate ears;
extern crate nix;

use nix::sys::termios;
use std::io::Read;
use ears::{Music, AudioController};

// a = 97
// z = 122
// e = 101
// q = 113
// s = 115
// d = 100
// w = 119

fn main() {
    let saved_term = termios::tcgetattr(0).unwrap();
    let mut term = saved_term;
    // Unset canonical mode, so we get characters immediately
    term.c_lflag.remove(termios::ICANON);
    // Don't generate signals on Ctrl-C and friends
    term.c_lflag.remove(termios::ISIG);
    // Disable local echo
    term.c_lflag.remove(termios::ECHO);
    termios::tcsetattr(0, termios::TCSADRAIN, &term).unwrap();

    println!("Welcome to 808rs!");
    println!("Press Ctrl-C to quit");

    for byte in std::io::stdin().bytes() {
        let byte = byte.unwrap();
        match byte {
            97 =>  play("kick".to_owned()),
            122 => play("snare".to_owned()),
            101 => play("hihat".to_owned()),
            113 => play("cymbal".to_owned()),
            115 => play("bass".to_owned()),
            100 => play("clap".to_owned()),
            119 => play("tom".to_owned()),
            3 =>   break,
            _ =>   play("bass".to_owned()),
        }
    }

    println!("Goodbye!");
    termios::tcsetattr(0, termios::TCSADRAIN, &saved_term).unwrap();
}

fn play(part: String) {
    let file = "./res/".to_string() + part.as_str() + ".wav";

    let mut sound = match Music::new(file.as_str()) {
        Some(sound) => sound,
        None        => panic!("Cannot load the file.")
    };

    sound.play();

    while sound.is_playing() {};
}
