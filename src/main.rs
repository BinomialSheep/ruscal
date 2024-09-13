#[derive(Debug, PartialEq, Eq)]
enum Token {
    Ident,
    Number,
}

// 1文字先に進む。input[1:]を返す
fn advance_char(input: &str) -> &str {
    let mut chars = input.chars();
    chars.next();
    chars.as_str()
}

// 先頭の文字を読む
fn peek_char(input: &str) -> Option<char> {
    input.chars().next()
}

fn whitespace(mut input: &str) -> &str {
    while matches!(peek_char(input), Some(' ')) {
        input = advance_char(input);
    }
    input
}

fn number(mut input: &str) -> (&str, Option<Token>) {
    if matches!(peek_char(input), Some(_x @ ('-' | '+' | '.' | '0'..='9'))) {
        input = advance_char(input);
        while matches!(peek_char(input), Some(_x @ ('.' | '0'..='9'))) {
            input = advance_char(input);
        }
        (input, Some(Token::Number))
    } else {
        (input, None)
    }
}

fn ident(mut input: &str) -> (&str, Option<Token>) {
    if matches!(peek_char(input), Some(_x @ ('a'..='z' | 'A'..='Z'))) {
        input = advance_char(input);
        while matches!(
            peek_char(input),
            Some(_x @ ('a'..='z' | 'A'..='Z' | '0'..='9'))
        ) {
            input = advance_char(input);
        }
        (input, Some(Token::Ident))
    } else {
        (input, None)
    }
}

fn token(i: &str) -> (&str, Option<Token>) {
    if let (i, Some(ident_res)) = ident(whitespace(i)) {
        return (i, Some(ident_res));
    }
    if let (i, Some(ident_res)) = number(whitespace(i)) {
        return (i, Some(ident_res));
    }
    (i, None)
}

fn source(mut input: &str) -> Vec<Token> {
    let mut tokens = vec![];
    while !input.is_empty() {
        input = if let (next_input, Some(token)) = token(input) {
            tokens.push(token);
            next_input
        } else {
            break;
        }
    }
    tokens
}

fn main() {
    let input = "123 world";
    println!("source: {}, parsed: {:?}", input, source(input),);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_whitespace() {
        assert_eq!(whitespace("   "), "");
        assert_eq!(whitespace("  a "), "a ");
    }

    #[test]
    fn test_ident() {
        assert_eq!(ident("Adam1a"), ("", Some(Token::Ident)));
    }

    #[test]
    fn test_number() {
        assert_eq!(number("123.45 "), (" ", Some(Token::Number)));
        assert_eq!(number("123.45 6"), (" 6", Some(Token::Number)));
        assert_eq!(number("Adam1a"), ("Adam1a", None));
    }
}
