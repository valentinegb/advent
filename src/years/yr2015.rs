mod dy1;

use clap::ValueEnum;

use self::dy1::pt1;

use super::AdventPuzzlePart;

#[derive(ValueEnum, Clone)]
pub(crate) enum Advent2015Day {
    #[value(name = "1")]
    Dy1,
}

pub(crate) fn yr2015(day: Advent2015Day, part: AdventPuzzlePart, input: String) {
    match day {
        Advent2015Day::Dy1 => match part {
            AdventPuzzlePart::Pt1 => pt1(input),
            AdventPuzzlePart::Pt2 => todo!(),
        },
    }
}
