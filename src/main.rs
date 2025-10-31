#[allow(dead_code)]
#[derive(Debug)]
enum BinaryNumber {
    Whole(String),
    Decimal { whole: String, fraction: String },
}

#[derive(Debug)]
enum Operator {
    Add,
    Sub,
    Mul,
    Div,
}

#[allow(dead_code)]
#[derive(Debug)]
enum Expression {
    Single(BinaryNumber),
    Operation {
        left: BinaryNumber,
        op: Operator,
        right: BinaryNumber,
    },
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

fn parse_expression(input: &str) -> Option<Expression> {
    for op_char in ['+', '-', '*', '/'] {
        if let Some((left, right)) = input.split_once(op_char) {
            let left = left.trim();
            let right = right.trim();

            let op = match op_char {
                '+' => Operator::Add,
                '-' => Operator::Sub,
                '/' => Operator::Div,
                '*' => Operator::Mul,
                _ => unreachable!(),
            };

            if let (Some(lbin), Some(rbin)) = (parse_binary(left), parse_binary(right)) {
                return Some(Expression::Operation {
                    left: lbin,
                    op,
                    right: rbin,
                });
            } else {
                return None;
            }
        }
    }

    parse_binary(input).map(Expression::Single)
}

fn main() {
    let tests = [
        "101",
        "101.1",
        "10.01 + 1",
        "10 * 11",
        "111 - 10.1",
        "101 / 2",
        "10.1.1",
    ];

    for &t in &tests {
        match parse_expression(t) {
            Some(expr) => println!("{t}: valid -> {:#?}", expr),
            None => println!("{t}: invalid"),
        }
    }
}
