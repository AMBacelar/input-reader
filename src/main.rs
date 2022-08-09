mod parse_input;

use fps_counter::*;
use gilrs::{Event, Gilrs};
use std::time::{Duration, Instant};

const MAX_MS_FOR_60FPS: f32 = 16.7;

fn main() {
    let mut gilrs = Gilrs::new().unwrap();
    let mut active_gamepad = None;
    let mut fps_count = FPSCounter::default();

    loop {
        let start = Instant::now();
        while let Some(Event { id, event, time }) = gilrs.next_event() {
            println!("{:?} New event from {}: {:?}", time, id, event);
            active_gamepad = Some(id);
        }
        if let Some(gamepad) = active_gamepad.map(|id| gilrs.gamepad(id)) {
            parse_input::parse(gamepad);
        }
        let elapsed = start.elapsed();
        let elapsed_as_milliseconds = elapsed.as_millis() as f32;
        if elapsed_as_milliseconds < MAX_MS_FOR_60FPS {
            let wait_duration = (MAX_MS_FOR_60FPS - elapsed_as_milliseconds) as u64;
            println!("Frame took how long?: {}, waited for {} [Note: 16.7ms is how long it needs to be for 60fps]", elapsed_as_milliseconds, wait_duration);
            std::thread::sleep(Duration::from_millis(wait_duration));
            println!("{}", fps_count.tick());
        }
    }
}
