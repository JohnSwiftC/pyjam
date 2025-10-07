const ONE: &str = "[[[]]>[]][[[]]<[]]<<[[[]]<[]][[]>[[]]]";
const ZERO: &str = "[[[]]<[]][[]>[[]]]<<[[[]]>[]][[[]]<[]]";
const XOR: &str = "^";
const SHIFT: &str ="<<";
pub fn bracketify(mut input: i32) -> String {
    let mut result = String::new();
    if input & 1 == 1 {
        result += ONE;
    } else {
        result += ZERO;
    }

    input = input >> 1;

    for i in 1..=31 {
        if input & 1 == 1 {
            result += XOR;
            for _ in 0..i {
                result += ONE;
                result += SHIFT;
            }

            result += ZERO;
        }

        input = input >> 1;
    }

    result
}