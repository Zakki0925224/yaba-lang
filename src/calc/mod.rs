pub mod ast;
pub mod parser;

use nom::{Err, error::{Error}};

pub fn expr_eval(s: &str) -> Result<i32, Err<Error<&str>>>
{
    return parser::expr_parser(s).map(|(_, expr)| expr.eval());
}

#[test]
fn test_expr_eval()
{
    assert_eq!
    (
        expr_eval("1+2+3+4+5").unwrap(),
        1 + 2 + 3 + 4 + 5
    );

    assert_eq!
    (
        expr_eval("1+2*3-7").unwrap(),
        1 + 2 * 3 - 7
    );

    assert_eq!
    (
        expr_eval("(2*24)/(5+3)").unwrap(),
        (2 * 24) / (5 + 3)
    );
}