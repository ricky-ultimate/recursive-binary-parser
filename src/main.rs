fn parse_digit(c: char) -> bool {
    c == '0' || c == '1'
}

fn parse_number(input: &str) -> bool {
    if input.is_empty() {
        return false;
    }

    if input.len() == 1 {
        return parse_digit(input.chars().next().unwrap());
    }

    if let Some((prefix, last)) = input.split_at_checked(input.len() - 1) {
        return parse_number(prefix) && parse_digit(last.chars().next().unwrap());
    }

    false
}

#[allow(dead_code)]
trait SplitAtChecked {
    fn split_at_checked(&self, mid: usize) -> Option<(&str, &str)>;
}

impl SplitAtChecked for &str {
    fn split_at_checked(&self, mid: usize) -> Option<(&str, &str)> {
        if mid <= self.len() {
            Some(self.split_at(mid))
        } else {
            None
        }
    }
}

fn main() {
    let tests = ["0", "1", "01", "001", "1001", "102", ""];

    for &t in &tests {
        println!("{t}: {}", parse_number(t));
    }
}
