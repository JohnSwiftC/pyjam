pub fn charbychar(input: &[u8]) -> String {
    let mut result = String::new();

    let length = input.len();
    for (i, &c) in input.into_iter().enumerate() {
        let value = crate::numbers::bracketify(c as i32);
        if i != length - 1 {
            result += &format!("chr({})+\\\n", value);
        } else {
            result += &format!("chr({})", value);
        }
    }

    result
}
