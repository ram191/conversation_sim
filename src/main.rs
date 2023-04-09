use fancy_print::{Animation, FancyPrinter};
use std::io;
use std::time::Duration;

mod conversation;
mod occupation;
mod participant;

fn main() {
    let printer = FancyPrinter::builder()
        .animation(Animation::Typing)
        .time_delay(Duration::from_millis(20))
        .build();

    printer.print("Welcome! Please input your name ......");

    let mut name = String::new();
    io::stdin()
        .read_line(&mut name)
        .expect("Failed to read line");

    let mut ram = participant::Participant::new(
        String::from("ram"),
        27,
        occupation::OccupationEnum::Unemployed,
        Vec::from([String::from("humble")]),
    );
}
