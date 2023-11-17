use std::io::stdin;

const W: usize = 5;
const H: usize = 7;

fn read_sum() -> usize {
    let mut numbers: Vec<usize> = Vec::with_capacity(2);
    let mut buf = String::new();

    let lines: Vec<_> = stdin().lines().take(H).filter_map(Result::ok).collect();
    for col in (0..lines[0].len()).step_by(W + 1) {
        let mut count = 0;
        lines
            .iter()
            .for_each(|l| count += l[col..col + W].chars().filter(|ch| *ch == 'x').count());

        if count == 9 {
            numbers.push(buf.parse().unwrap());
            buf.clear();
            continue;
        }

        if let Some(c) = [20, 7, 14, 11, 23].iter().position(|n| *n == count) {
            buf.push("01478".chars().nth(c).unwrap());
            continue;
        }

        let left_bottom = lines[4].chars().nth(col).unwrap() == 'x';

        if count == 21 {
            buf.push(if left_bottom { '6' } else { '9' });
            continue;
        }

        let right_top = lines[1].chars().nth(col + W - 1).unwrap() == 'x';

        if count == 19 {
            let digit = match (left_bottom, right_top) {
                (true, true) => '2',
                (false, true) => '3',
                (false, false) => '5',
                _ => panic!(),
            };
            buf.push(digit);
        }
    }
    numbers.push(buf.parse().unwrap());
    numbers.iter().sum::<usize>()
}

static DIGITS: [&'static str; H] = [
    "xxxxx....xxxxxxxxxxxx...xxxxxxxxxxxxxxxxxxxxxxxxxx",
    "x...x....x....x....xx...xx....x........xx...xx...x",
    "x...x....x....x....xx...xx....x........xx...xx...x",
    "x...x....xxxxxxxxxxxxxxxxxxxxxxxxxx....xxxxxxxxxxx",
    "x...x....xx........x....x....xx...x....xx...x....x",
    "x...x....xx........x....x....xx...x....xx...x....x",
    "xxxxx....xxxxxxxxxxx....xxxxxxxxxxx....xxxxxxxxxxx",
];

fn main() {
    let s = read_sum().to_string();
    let mut outlines = std::iter::repeat(String::new()).take(7).collect::<Vec<_>>();
    for (i, digit) in s.chars().enumerate() {
        let idx = digit.to_digit(10).unwrap() as usize * W;
        for row in 0..outlines.len() {
            outlines[row].push_str(&DIGITS[row][idx..idx + W]);
            if i < s.len() - 1 {
                outlines[row].push('.');
            }
        }
    }
    for l in outlines {
        println!("{}", l);
    }
}
