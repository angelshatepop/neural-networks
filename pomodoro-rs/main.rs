#!/usr/bin/env run-cargo-script
use std::{io::{stdout, Write}, thread::sleep, time::Duration};

pub fn pomodoro_timer(title: &str, mins: u32) {
    let mut stdout = stdout();
    let (mut seconds, mut seconds_total) = (60, mins*60);
    loop {
        if seconds_total == 0 {
            break;
        } else {
            seconds -= 1;
            seconds_total -= 1;
            let mins = seconds_total / 60;
            let time_str = format!("\r{}... {:02}:{:02}", title, mins, seconds);

            stdout.write(time_str.as_bytes()).unwrap();
            stdout.flush().unwrap();
            if seconds == 0 {seconds = 60};
            sleep(Duration::from_secs(1));
        }
    }
}

pub fn pomodoro(focus_len: u32, rest_len: u32, cycles: u32) {
    let mut cycle = 0;
    while cycle != cycles {
        pomodoro_timer("focus", focus_len);
        pomodoro_timer("rest", rest_len);
        println!("  --pomodoro({}) complete", cycle );
        cycle += 1;
    }
    println!("\npomodoro session complete!")
}

fn main () {
    pomodoro(25, 5, 4)
}