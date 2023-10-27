pub(super) fn dy1(input: String) {
    let mut floor = 0;

    for char in input.chars() {
        match char {
            '(' => floor += 1,
            ')' => floor -= 1,
            _ => unreachable!("puzzle input should not contain characters other than '(' and ')'"),
        }
    }

    println!("{floor}");
}
