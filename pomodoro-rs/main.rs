#!/usr/bin/env run-cargo-script
use std::{
    io::{stdout, Write},
    thread::sleep,
    time::Duration,
};

pub fn focus(mins: u32) {
    let mut stdout = stdout();
    let (mut seconds, mut seconds_total) = (60, mins*60);
    loop {
        if seconds_total == 0{
            break;
        }
        else{
            seconds -= 1;
            seconds_total -= 1;
            let mins = seconds_total/60;
            if mins < 10 {
                if seconds == 0 {
                    stdout.write(format!("\rfocus...  {}:00", mins).as_bytes()).unwrap();
                    stdout.flush().unwrap();
                    seconds = 60;
                }
                else{
                    stdout.write(format!("\rfocus...  {}:{}", mins, seconds).as_bytes()).unwrap();
                    stdout.flush().unwrap();
                }
                sleep(Duration::from_secs(1));
            }
            else {
                if seconds == 0 {
                    stdout.write(format!("\rfocus... {}:00", mins).as_bytes()).unwrap();
                    stdout.flush().unwrap();
                    seconds = 60;
                }
                else{
                    stdout.write(format!("\rfocus... {}:{}", mins, seconds).as_bytes()).unwrap();
                    stdout.flush().unwrap();
                }
                sleep(Duration::from_secs(1));
            }
        }
    }

}
pub fn rest(mins: u32) {
    let mut stdout = stdout();
    let (mut seconds, mut seconds_total) = (60, mins*60);
    loop {
        if seconds_total == 0{
            stdout.write(format!("\rdone           ").as_bytes()).unwrap();
            stdout.flush().unwrap();
            break;
        }
        else{
            seconds -= 1;
            seconds_total -= 1;
            let mins = seconds_total/60;
            if mins < 10 {
                if seconds == 0 {
                    stdout.write(format!("\rrest...   {}:00", mins).as_bytes()).unwrap();
                    stdout.flush().unwrap();
                    seconds = 60;
                }
                else{
                    stdout.write(format!("\rrest...   {}:{}", mins, seconds).as_bytes()).unwrap();
                    stdout.flush().unwrap();
                }
                sleep(Duration::from_secs(1));
            }
            else {
                if seconds == 0 {
                    stdout.write(format!("\rrest...  {}:00", mins).as_bytes()).unwrap();
                    stdout.flush().unwrap();
                    seconds = 60;
                }
                else{
                    stdout.write(format!("\rrest... {}:{}", mins, seconds).as_bytes()).unwrap();
                    stdout.flush().unwrap();
                }
                sleep(Duration::from_secs(1));
            }
        }
    }
}
pub fn pomodoro(focus_len: u32, rest_len: u32, cycles: u32) {
    let mut cycle = 0;

    while cycle != cycles {
        focus(focus_len);
        rest(rest_len);
        cycle += 1;
        println!("  --pomodoro({}) complete", cycle );
    }
    println!("\npomodoro session complete!")
}
fn main () {
    pomodoro(25, 5, 2)
}