pub(crate) fn extract_digits(s: &str) -> Result<(&str, &str), String> {
    take_while_err(|c| c.is_ascii_digit(), s, "expected digits".to_string())
}

pub(crate) fn extract_whitespace(s: &str) -> Result<(&str, &str), String> {
    take_while_err(|c| c == ' ', s, "expected a space".to_string())
}

pub(crate) fn skip_whitespace(s: &str) -> (&str, &str) {
    take_while(|c| c.is_whitespace(), s)
}

pub(crate) fn take_while_err(
    accept: impl Fn(char) -> bool,
    s: &str,
    err_msg: String,
) -> Result<(&str, &str), String> {
    let (reminder, extracted) = take_while(accept, s);
    if extracted.is_empty() {
        Err(err_msg)
    } else {
        Ok((reminder, extracted))
    }
}

pub(crate) fn take_while(accept: impl Fn(char) -> bool, s: &str) -> (&str, &str) {
    let extracted_end = s
        .char_indices()
        .find_map(|(idx, c)| if accept(c) { None } else { Some(idx) })
        .unwrap_or(s.len());

    let extracted = &s[..extracted_end];
    let reminder = &s[extracted_end..];
    (reminder, extracted)
}

pub(crate) fn extract_ident(s: &str) -> Result<(&str, &str), String> {
    let input_start_with_alphabetic = s
        .chars()
        .next()
        .map(|c| c.is_ascii_alphabetic())
        .unwrap_or(false);
    if input_start_with_alphabetic {
        Ok(take_while(|c| c.is_alphanumeric(), s))
    } else {
        Err("expected identifier".to_string())
    }
}

pub(crate) fn tag<'a, 'b>(starting_text: &'a str, s: &'b str) -> Result<&'b str, String> {
    if let Some(end) = s.strip_prefix(starting_text) {
        Ok(end)
    } else {
        Err(format!("expected {}", starting_text))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn extract_one_digit() {
        assert_eq!(extract_digits("1+2"), Ok(("+2", "1")))
    }

    #[test]
    fn extract_multiple_digit() {
        assert_eq!(extract_digits("10-20"), Ok(("-20", "10")))
    }

    #[test]
    fn do_not_extract_anything_from_empty_input() {
        assert_eq!(extract_digits(""), Err("expected digits".to_string()))
    }

    #[test]
    fn do_not_extract_when_input_is_not_digit() {
        assert_eq!(extract_digits("abs"), Err("expected digits".to_string()))
    }

    #[test]
    fn extract_digit_with_no_reminder() {
        assert_eq!(extract_digits("100"), Ok(("", "100")))
    }

    #[test]
    fn extract_spaces() {
        assert_eq!(extract_whitespace("    1"), Ok(("1", "    ")))
    }

    #[test]
    fn do_not_extract_spaces_when_input_does_not_have_space() {
        assert_eq!(extract_whitespace("1"), Err("expected a space".to_string()))
    }

    #[test]
    fn extract_alphabetic_ident() {
        assert_eq!(extract_ident("absc break"), Ok((" break", "absc")))
    }

    #[test]
    fn extract_alphanumeric_ident() {
        assert_eq!(extract_ident("absc1 break"), Ok((" break", "absc1")))
    }

    #[test]
    fn extract_ident_start_with_number() {
        assert_eq!(
            extract_ident("123f"),
            Err("expected identifier".to_string())
        )
    }

    #[test]
    fn tag_word() {
        assert_eq!(tag("let", "let a"), Ok(" a"))
    }
}
