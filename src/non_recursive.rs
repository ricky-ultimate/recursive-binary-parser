fn parse_digit(c: char) -> bool {
    c == '0' || c == '1'
}

fn parse_number(input: &str) -> bool {
    if input.is_empty() {
        return false;
    }

    for c in input.chars() {
        if !parse_digit(c) {
            return false;
        }
    }

    true
}

fn main() {
    let tests = ["0", "1", "01", "001", "1001", "102", ""];

    for &t in &tests {
        println!("{t}: {}", parse_number(t));
    }
}
