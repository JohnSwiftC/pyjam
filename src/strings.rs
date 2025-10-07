pub fn charbychar(input: &str) -> String {
    let mut result = String::new();

    for &c in input.as_bytes() {
        let value = crate::numbers::bracketify(c as i32);
        result += &format!("chr({})+", value);
    }

    result[0..result.len() - 1].to_owned()
}