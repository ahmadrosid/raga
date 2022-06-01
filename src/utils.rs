pub(crate) fn extract_digits(s: &str) -> (&str, &str) {
    take_while(|c| c.is_ascii_digit(), s)
}

pub(crate) fn extract_op(s: &str) -> (&str, &str) {
    match &s[0..1] {
        "+" | "-" | "*" | "/" => {}
        _ => panic!("bad operator"),
    }

    (&s[1..], &s[0..1])
}

pub(crate) fn extract_whitespace(s: &str) -> (&str, &str) {
    take_while(|c| c == ' ', s)
}

pub(crate) fn take_while(accept: impl Fn(char) -> bool, s: &str) -> (&str, &str) {
    let extracted_end = s
        .char_indices()
        .find_map(|(idx, c)| if accept(c) { None } else { Some(idx) })
        .unwrap_or_else(|| s.len());
    let extracted = &s[..extracted_end];
    let reminder = &s[extracted_end..];
    (reminder, extracted)
}

pub(crate) fn extract_ident(s: &str) -> (&str, &str) {
    let input_start_with_alphabetic = s
        .chars()
        .next()
        .map(|c| c.is_ascii_alphabetic())
        .unwrap_or(false);
    if input_start_with_alphabetic {
        take_while(|c| c.is_alphanumeric(), s)
    } else {
        (s, "")
    }
}

pub(crate) fn tag<'a, 'b>(starting_text: &'a str, s: &'b str) -> &'b str {
    if s.starts_with(starting_text) {
        &s[starting_text.len()..]
    } else {
        panic!("expected {}", starting_text)
    }
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

    #[test]
    fn extract_spaces() {
        assert_eq!(extract_whitespace("    1"), ("1", "    "))
    }

    #[test]
    fn extract_alphabetic_ident() {
        assert_eq!(extract_ident("absc break"), (" break", "absc"))
    }

    #[test]
    fn extract_alphanumeric_ident() {
        assert_eq!(extract_ident("absc1 break"), (" break", "absc1"))
    }

    #[test]
    fn extract_ident_start_with_number() {
        assert_eq!(extract_ident("123f"), ("123f", ""))
    }

    #[test]
    fn tag_word() {
        assert_eq!(tag("let", "let a"), (" a"))
    }
}
