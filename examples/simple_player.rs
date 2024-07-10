// The MIT License (MIT)
//
// Copyright (c) 2013 Jeremy Letang (letang.jeremy@gmail.com)
//
// Permission is hereby granted, free of charge, to any person obtaining a copy of
// this software and associated documentation files (the "Software"), to deal in
// the Software without restriction, including without limitation the rights to
// use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of
// the Software, and to permit persons to whom the Software is furnished to do so,
// subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS
// FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR
// COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER
// IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN
// CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.

extern crate ears;

use std::io::stdin;
use std::io::stdout;
use std::io::Write;

use ears::State::{Paused, Playing, Stopped};
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

    loop {
        // Make your choice
        println!("Commands :\n\tPlay  : l\n\tPause : p\n\tStop  : s\n\tExit  : x\n");
        let mut cmd = String::new();
        stdin.read_line(&mut cmd).ok();
        match &cmd[..1] {
            "l" => music.play(),
            "p" => music.pause(),
            "s" => music.stop(),
            "x" => {
                music.stop();
                break;
            }
            _ => println!("Unknown command."),
        }
        match music.get_state() {
            Playing => println!("State : Playing"),
            Stopped => println!("State : Stopped"),
            Paused => println!("State : Paused"),
            _ => unreachable!(),
        };
    }
    println!("Goodbye!");
}
