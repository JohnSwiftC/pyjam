pub fn charbychar(input: &[u8]) -> String {
    let mut result = String::new();

    for &c in input {
        let value = crate::numbers::bracketify(c as i32);
        result += &format!("chr({})+", value);
    }

    result[0..result.len() - 1].to_owned()
}