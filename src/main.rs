use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alpha1, alphanumeric1, char, multispace0},
    combinator::recognize,
    multi::{fold_many0, many0},
    number::complete::recognize_float,
    sequence::{delimited, pair},
    IResult,
};

fn main() {
    let input = "123";
    println!("source: {:?}, parsed: {:?}", input, expr(input));

    let input = "Hello + world";
    println!("source: {:?}, parsed: {:?}", input, expr(input));

    let input = "(123 + 456 ) + world";
    println!("source: {:?}, parsed: {:?}", input, expr(input));

    let input = "car + cdr + cdr";
    println!("source: {:?}, parsed: {:?}", input, expr(input));

    let input = "((1 + 2) + (3 + 4)) + 5 + 6";
    println!("source: {:?}, parsed: {:?}", input, expr(input));
}

#[derive(Debug, PartialEq, Clone)]
enum Token<'src> {
    Ident(&'src str),
    Number(f64),
}

#[derive(Debug, PartialEq, Clone)]
enum Expression<'src> {
    Value(Token<'src>),
    Add(Box<Expression<'src>>, Box<Expression<'src>>),
}

// number, ident, parensのいずれかにマッチした場合、Ok(残りの文字列, パーサが生成した出力)を返す
// 例：Ok(("", 123))という結果は、入力全体をパースして数値123を得たことを示す
fn term(i: &str) -> IResult<&str, Expression> {
    alt((number, ident, parens))(i)
}

fn ident(input: &str) -> IResult<&str, Expression> {
    let (r, res) = delimited(multispace0, identifier, multispace0)(input)?;
    Ok((r, Expression::Value(Token::Ident(res))))
}

fn identifier(input: &str) -> IResult<&str, &str> {
    recognize(pair(
        alt((alpha1, tag("_"))),
        many0(alt((alphanumeric1, tag("_")))),
    ))(input)
}

fn number(input: &str) -> IResult<&str, Expression> {
    let (r, v) = delimited(multispace0, recognize_float, multispace0)(input)?;
    Ok((
        r,
        Expression::Value(Token::Number(v.parse().map_err(|_| {
            nom::Err::Error(nom::error::Error {
                input,
                code: nom::error::ErrorKind::Digit,
            })
        })?)),
    ))
}

fn parens(i: &str) -> IResult<&str, Expression> {
    delimited(
        multispace0,
        delimited(tag("("), expr, tag(")")),
        multispace0,
    )(i)
}

fn expr(i: &str) -> IResult<&str, Expression> {
    let (i, init) = term(i)?;

    // クロージャ「move || init.clone()」について：noveはキャプチャする変数の所有権の指定、||は引数なし

    fold_many0(
        pair(delimited(multispace0, char('+'), multispace0), term),
        move || init.clone(),
        |acc, (_op, val): (char, Expression)| Expression::Add(Box::new(acc), Box::new(val)),
    )(i)
}

#[cfg(test)]
mod tests {
    use super::*;
    use nom::error::ErrorKind;

    #[test]
    fn test_term_with_number() {
        let input = "42";
        let result = term(input);
        assert_eq!(result, Ok(("", Expression::Value(Token::Number(42.0)))));
    }

    #[test]
    fn test_term_with_identifier() {
        let input = "variable";
        let result = term(input);
        assert_eq!(
            result,
            Ok(("", Expression::Value(Token::Ident("variable"))))
        );
    }

    #[test]
    fn test_term_with_parentheses() {
        let input = "(123)";
        let result = term(input);
        assert_eq!(result, Ok(("", Expression::Value(Token::Number(123.0)))));
    }

    #[test]
    fn test_term_with_invalid_input() {
        let input = "!invalid";
        let result = term(input);
        assert!(result.is_err());
        if let Err(nom::Err::Error(nom::error::Error { code, .. })) = result {
            assert_eq!(code, ErrorKind::Tag);
        }
    }
}
