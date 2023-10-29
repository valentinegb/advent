pub(super) fn pt1(input: String) {
    let mut visiting = (0, 0);
    let mut visited = vec![(0, 0)];

    for char in input.chars() {
        match char {
            '^' => visiting.1 += 1,
            'v' => visiting.1 -= 1,
            '>' => visiting.0 += 1,
            '<' => visiting.0 -= 1,
            _ => unreachable!(
                "puzzle input should only contain the characters '^', 'v', '>', and '<'"
            ),
        }

        if !visited.contains(&visiting) {
            visited.push(visiting);
        }
    }

    println!("{}", visited.len());
}

pub(super) fn pt2(input: String) {
    todo!();
}
