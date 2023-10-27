fn parse_input(input: String) -> Vec<(u32, u32, u32)> {
    input
        .split('\n')
        .map(|present| {
            let (left, right) = present
                .split_once('x')
                .expect("present dimensions should contain two 'x'");
            let right = right
                .split_once('x')
                .expect("present dimensions should contain two 'x'");

            (
                left.parse::<u32>()
                    .expect("present `l` should be a positive 32-bit number"),
                right
                    .0
                    .parse::<u32>()
                    .expect("present `w` should be a positive 32-bit number"),
                right
                    .1
                    .parse::<u32>()
                    .expect("present `h` should be a positive 32-bit number"),
            )
        })
        .collect()
}

pub(super) fn pt1(input: String) {
    let presents = parse_input(input);
    let mut total_wrapping_paper = 0;

    for (l, w, h) in presents {
        let mut wrapping_paper = 0;
        let mut sides = [l * w, w * h, h * l];

        for side in sides {
            wrapping_paper += side * 2;
        }

        sides.sort();
        wrapping_paper += sides[0];
        total_wrapping_paper += wrapping_paper;
    }

    println!("{total_wrapping_paper}");
}

pub(super) fn pt2(input: String) {
    let presents = parse_input(input);
    let mut total_ribbon = 0;

    for (l, w, h) in presents {
        let mut ribbon = 0;
        let mut dimensions = [l, w, h];

        dimensions.sort();
        ribbon += dimensions[0] * 2 + dimensions[1] * 2 + (l * w * h);
        total_ribbon += ribbon;
    }

    println!("{total_ribbon}");
}
