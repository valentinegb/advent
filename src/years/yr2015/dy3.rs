fn visit(char: char, visiting: &mut (i32, i32), visited: &mut Vec<(i32, i32)>) {
    match char {
        '^' => visiting.1 += 1,
        'v' => visiting.1 -= 1,
        '>' => visiting.0 += 1,
        '<' => visiting.0 -= 1,
        _ => unreachable!("puzzle input should only contain the characters '^', 'v', '>', and '<'"),
    }

    if !visited.contains(visiting) {
        visited.push(*visiting);
    }
}

pub(super) fn pt1(input: String) {
    let mut visiting = (0, 0);
    let mut visited = vec![(0, 0)];

    for char in input.chars() {
        visit(char, &mut visiting, &mut visited);
    }

    println!("{}", visited.len());
}

pub(super) fn pt2(input: String) {
    let mut santa_visiting = (0, 0);
    let mut robo_santa_visiting = (0, 0);
    let mut visited = vec![(0, 0)];
    let mut robo_santa_moving = false;

    for char in input.chars() {
        if robo_santa_moving {
            visit(char, &mut santa_visiting, &mut visited);
        } else {
            visit(char, &mut robo_santa_visiting, &mut visited);
        }

        robo_santa_moving = !robo_santa_moving;
    }

    println!("{}", visited.len());
}
