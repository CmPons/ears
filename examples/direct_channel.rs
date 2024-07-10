extern crate ears;

use std::io::stdin;
use std::io::stdout;
use std::io::Write;

use ears::{AudioController, Music};

fn main() {
    // Read the inputs
    let stdin = stdin();

    print!("Insert the path to an audio file: ");
    stdout().flush().ok();

    let mut line = String::new();
    stdin.read_line(&mut line).ok();
    loop {
        match &line[line.len() - 1..] {
            "\n" => {
                line.pop();
                
            }
            "\r" => {
                line.pop();
                
            }
            _ => {
                break;
            }
        }
    }

    // Try to create the music
    let mut music = Music::new(&line[..]).expect("Error loading music.");

    // Play it
    music.play();
    music.set_looping(true);

    let mut toggle = false;

    loop {
        music.set_direct_channel(toggle);

        let direct_channel_enabled = music.get_direct_channel();

        if direct_channel_enabled != toggle {
            println!("Failed to enabled direct channel mode.");
            println!("Extension may not be available.");
        } else {
            match direct_channel_enabled {
                true => println!("Direct channel enabled."),
                false => println!("Direct channel disabled."),
            };
        }

        println!("Press enter to toggle direct channel mode, or 'x' to quit");
        let mut cmd = String::new();
        stdin.read_line(&mut cmd).ok();

        match &cmd[..1] {
            "x" => {
                music.stop();
                break;
            }
            _ => toggle = !toggle,
        };
    }
}
