#[allow(dead_code)]
#[derive(Debug)]
enum BinaryNumber {
    Whole(String),
    Decimal { whole: String, fraction: String },
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

fn parse_binary(input: &str) -> Option<BinaryNumber> {
    if input.is_empty() {
        return None;
    }

    if let Some((whole_part, fraction_part)) = input.split_once(".") {
        if parse_number(whole_part) && parse_number(fraction_part) {
            Some(BinaryNumber::Decimal {
                whole: whole_part.to_string(),
                fraction: fraction_part.to_string(),
            })
        } else {
            None
        }
    } else if parse_number(input) {
        Some(BinaryNumber::Whole(input.to_string()))
    } else {
        None
    }
}

fn main() {
    let tests = [
        "0", "1", "01", "001", "1001", "102", "101.1", "10.02", "111.0001",
    ];

    for t in tests {
        match parse_binary(t) {
            Some(num) => println!("{t}: valid -> {:?}", num),
            None => println!("{t}: invalid"),
        }
    }
}
