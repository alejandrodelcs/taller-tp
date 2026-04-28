pub fn guess_output_name(input: &str) -> String {
    let mut s = input.to_string();
    s.pop();
    s.push('o');
    s
}
