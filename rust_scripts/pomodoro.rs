use std::{io::{stdout, Write}, thread::sleep, time::Duration};
use clap::Parser;

#[derive(Parser, Default, Debug)]
#[clap(author="angelshatepop", version, about="pomodoro clock")]
struct Args {
   focus: u32,
   rest: u32,
   pomodoros: u32,
}

pub fn pomodoro_timer(title: &str, mins: u32, state: bool) {
    let mut stdout = stdout();
    let (mut seconds, mut seconds_total) = (60, mins*60);
    loop {
        if seconds_total == 0 {
            break;
        } else {
            seconds -= 1;
            seconds_total -= 1;
            let mins = seconds_total / 60;
            let mut time_str = format!("\r{}... {:02}:{:02}", title, mins, seconds);
            if state == false {
                time_str = format!("\r{}...  {:02}:{:02}", title, mins, seconds);
            }
            stdout.write(time_str.as_bytes()).unwrap();
            stdout.flush().unwrap();
            if seconds == 0 {seconds = 60};
            sleep(Duration::from_secs(1));
        }
    }
}

pub fn pomodoro(focus_len: u32, rest_len: u32, cycles: u32) {
    print!("{esc}c", esc = 27 as char);
    let mut cycle = 0;
    while cycle != cycles {
        pomodoro_timer("focus", focus_len, true);
        pomodoro_timer("rest", rest_len, false);
        println!("  --pomodoro({}) complete", cycle+1);
        cycle += 1;
    }
    println!("\npomodoro session complete!")
}

fn main () {
    let args = Args::parse();
    pomodoro(args.focus, args.rest, args.pomodoros)
}