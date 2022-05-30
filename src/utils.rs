pub (crate) fn extract_digits(s: &str) -> (&str, &str) {
    let digits_end = s.char_indices()
        .find_map(|(idx, c)| if c.is_ascii_digit() { None } else { Some(idx) })
        .unwrap_or_else(|| s.len());

    let digit  = &s[..digits_end];
    let reminder = &s[digits_end..];

    (reminder, digit)
}

pub (crate) fn extract_op(s: &str) -> (&str, &str) {
    match &s[0..1] {
        "+" | "-" | "*" | "/" => {},
        _ => panic!("bad operator")
    }

    (&s[1..], &s[0..1])
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn extract_one_digit() {
        assert_eq!(extract_digits("1+2"), ("+2", "1"))
    }

    #[test]
    fn extract_multiple_digit() {
        assert_eq!(extract_digits("10-20"), ("-20", "10"))
    }

    #[test]
    fn do_not_extract_anything_from_empty_input() {
        assert_eq!(extract_digits(""), ("", ""))
    }

    #[test]
    fn extract_digit_with_no_reminder() {
        assert_eq!(extract_digits("100"), ("", "100"))
    }

    #[test]
    fn extract_op_plus() {
        assert_eq!(extract_op("+100"), ("100", "+"))
    }

    #[test]
    fn extract_op_minus() {
        assert_eq!(extract_op("-100"), ("100", "-"))
    }

    #[test]
    fn extract_op_star() {
        assert_eq!(extract_op("*100"), ("100", "*"))
    }

    #[test]
    fn extract_op_slash() {
        assert_eq!(extract_op("/100"), ("100", "/"))
    }
}