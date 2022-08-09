use gilrs::{Button, Gamepad};

pub fn parse(gamepad: Gamepad) {
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
