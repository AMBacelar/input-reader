use gilrs::{Button, Event, Gamepad, Gilrs};
use std::time::{Duration, Instant};

const MAX_MS_FOR_60FPS: f32 = 16.7;

fn parse_input(gamepad: Gamepad) {
    // println!("{:?}", gamepad);
    let mut direction = "5";
    // Direction
    if gamepad.is_pressed(Button::DPadUp) {
        if gamepad.is_pressed(Button::DPadLeft) {
            direction = "7";
        } else if gamepad.is_pressed(Button::DPadRight) {
            direction = "9";
        } else {
            direction = "8";
        }
    } else if gamepad.is_pressed(Button::DPadDown) {
        if gamepad.is_pressed(Button::DPadLeft) {
            direction = "1";
        } else if gamepad.is_pressed(Button::DPadRight) {
            direction = "3";
        } else {
            direction = "2";
        }
    } else if gamepad.is_pressed(Button::DPadLeft) {
        direction = "4";
    } else if gamepad.is_pressed(Button::DPadRight) {
        direction = "6";
    }

    // Face Buttons
    let mut action_buttons = String::from("");
    if gamepad.is_pressed(Button::West) {
        action_buttons.push('A')
    }
    if gamepad.is_pressed(Button::North) {
        action_buttons.push('B')
    }
    if gamepad.is_pressed(Button::East) {
        action_buttons.push('C')
    }
    if gamepad.is_pressed(Button::South) {
        action_buttons.push('D')
    }
    if gamepad.is_pressed(Button::LeftTrigger) {
        action_buttons.push('E')
    }
    if gamepad.is_pressed(Button::RightTrigger) {
        action_buttons.push('F')
    }

    println!(
        "Debug: gamepad: {}, direction: {}, buttons pressed: {}",
        gamepad.name(),
        direction,
        action_buttons
    );
}

fn main() {
    let mut gilrs = Gilrs::new().unwrap();
    let mut active_gamepad = None;

    loop {
        let start = Instant::now();

        while let Some(Event { id, event, time }) = gilrs.next_event() {
            println!("{:?} New event from {}: {:?}", time, id, event);
            active_gamepad = Some(id);
        }

        if let Some(gamepad) = active_gamepad.map(|id| gilrs.gamepad(id)) {
            parse_input(gamepad);
        }
        let elapsed = start.elapsed();
        let elapsed_as_milliseconds = elapsed.as_millis() as f32;
        if elapsed_as_milliseconds < MAX_MS_FOR_60FPS {
            let wait_duration = (MAX_MS_FOR_60FPS - elapsed_as_milliseconds) as u64;
            println!("Frame took how long?: {}, waited for {} [Note: 16.7ms is how long it needs to be for 60fps]", elapsed_as_milliseconds, wait_duration);
            std::thread::sleep(Duration::from_millis(wait_duration));
        }
    }
}
