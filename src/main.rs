use clap::Parser;
use rdev::{
    EventType::{KeyPress, KeyRelease},
    Key, SimulateError, listen, simulate,
};
use std::thread::sleep;
use std::time::Duration;

const DELAY: Duration = Duration::from_millis(4);

const fn key_from_char(c: char) -> Key {
    match c {
        '0' => Key::Num0,
        '1' => Key::Num1,
        '2' => Key::Num2,
        '3' => Key::Num3,
        '4' => Key::Num4,
        '5' => Key::Num5,
        '6' => Key::Num6,
        '7' => Key::Num7,
        '8' => Key::Num8,
        '9' => Key::Num9,
        _ => unreachable!(),
    }
}

fn parse_courses(arg: &str) -> Result<[Key; 5], String> {
    if arg.len() != 5 {
        return Err(format!("'{}' is not of length 5.", arg));
    }
    if !arg.chars().all(|c| c.is_ascii_digit()) {
        return Err(format!("'{}' needs to be all numbers.", arg));
    }
    Ok(arg
        .chars()
        .map(key_from_char)
        .collect::<Vec<Key>>()
        .try_into()
        .map_err(|_| "Failed to convert character vector into fixed array".to_string())?)
}

#[derive(Parser, Debug)]
#[command(author, version, about)]
pub struct Config {
    #[arg(required = true, num_args = ..=10, value_parser = parse_courses)]
    courses: Vec<[Key; 5]>,
}

fn send(k: Key) -> Result<(), SimulateError> {
    sleep(DELAY);
    simulate(&KeyPress(k))?;
    sleep(DELAY);
    simulate(&KeyRelease(k))
}

fn handle_sim(courses: Vec<[Key; 5]>) -> Result<(), SimulateError> {
    let n = courses.len();

    for (i, course) in courses.into_iter().enumerate() {
        for key in course {
            send(key)?;
        }
        if i < n - 1 {
            send(Key::Tab)?;
        }
    }
    send(Key::Return)?;
    Ok(())
}

fn main() {
    let courses = Config::parse().courses;

    println!("\nAction key: 'ESC'\nPress 'CTRL-C' to quit.\n");

    if let Err(e) = listen(move |e| {
        if let KeyPress(k) = e.event_type {
            if let Key::Escape = k {
                if let Err(e) = handle_sim(courses.clone()) {
                    eprintln!("{:?}", e)
                }
            }
        }
    }) {
        eprintln!("{:?}", e)
    }
}
