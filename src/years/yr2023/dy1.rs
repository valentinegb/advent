pub(super) fn pt1(input: String) {
    let mut sum = 0;

    for line in input.lines() {
        let mut number_str = String::new();

        for char in line.chars() {
            if char.is_digit(10) {
                number_str += &char.to_string();
                break;
            }
        }

        for char in line.chars().rev() {
            if char.is_digit(10) {
                number_str += &char.to_string();
                break;
            }
        }

        sum += number_str.parse::<u32>().unwrap();
    }

    println!("{sum}");
}

pub(super) fn pt2(input: String) {
    let mut sum = 0;

    for line in input.lines() {
        let mut number_str = String::new();
        let mut read_chars = String::new();

        for char in line.chars() {
            if char.is_digit(10) {
                number_str += &char.to_string();
                break;
            }

            read_chars += &char.to_string();

            if read_chars.contains("one") {
                number_str += "1";
                break;
            } else if read_chars.contains("two") {
                number_str += "2";
                break;
            } else if read_chars.contains("three") {
                number_str += "3";
                break;
            } else if read_chars.contains("four") {
                number_str += "4";
                break;
            } else if read_chars.contains("five") {
                number_str += "5";
                break;
            } else if read_chars.contains("six") {
                number_str += "6";
                break;
            } else if read_chars.contains("seven") {
                number_str += "7";
                break;
            } else if read_chars.contains("eight") {
                number_str += "8";
                break;
            } else if read_chars.contains("nine") {
                number_str += "9";
                break;
            }
        }

        let mut read_chars = String::new();

        for char in line.chars().rev() {
            if char.is_digit(10) {
                number_str += &char.to_string();
                break;
            }

            read_chars += &char.to_string();

            if read_chars.contains(&"one".chars().rev().collect::<String>()) {
                number_str += "1";
                break;
            } else if read_chars.contains(&"two".chars().rev().collect::<String>()) {
                number_str += "2";
                break;
            } else if read_chars.contains(&"three".chars().rev().collect::<String>()) {
                number_str += "3";
                break;
            } else if read_chars.contains(&"four".chars().rev().collect::<String>()) {
                number_str += "4";
                break;
            } else if read_chars.contains(&"five".chars().rev().collect::<String>()) {
                number_str += "5";
                break;
            } else if read_chars.contains(&"six".chars().rev().collect::<String>()) {
                number_str += "6";
                break;
            } else if read_chars.contains(&"seven".chars().rev().collect::<String>()) {
                number_str += "7";
                break;
            } else if read_chars.contains(&"eight".chars().rev().collect::<String>()) {
                number_str += "8";
                break;
            } else if read_chars.contains(&"nine".chars().rev().collect::<String>()) {
                number_str += "9";
                break;
            }
        }

        sum += number_str.parse::<u32>().unwrap();
    }

    println!("{sum}");
}
