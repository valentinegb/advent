mod dy1;

use clap::ValueEnum;

use self::dy1::dy1;

#[derive(ValueEnum, Clone)]
pub(crate) enum Advent2015Day {
    #[value(name = "1")]
    Dy1,
}

pub(crate) fn yr2015(day: Advent2015Day, input: String) {
    match day {
        Advent2015Day::Dy1 => dy1(input),
    }
}
