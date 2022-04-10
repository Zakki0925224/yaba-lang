use core::panic;
use std::{str::FromStr};

use nom::{IResult, character::complete::{char, digit1}, branch::alt, combinator::{map, opt}, sequence::tuple};

use super::ast::*;

// 定数パーサ
// BNF: <const_val> := 1つ以上の数字
pub fn constant_val_parser(s: &str) -> IResult<&str, ConstantVal>
{
    let (no_used, used) = digit1(s)?;
    let val = FromStr::from_str(used).unwrap();

    return Ok((no_used, ConstantVal::new(val)));
}

// 式パーサ
// BNF: <expr> := <term> [ ('+'|'-') <expr> ]
pub fn expr_parser(s: &str) -> IResult<&str, Expr>
{
    // +-記号をOpKindに変換する
    let op_kind_parser = map
    (
        alt((char('+'), char('-'))),
        |c| match c
        {
            '+' => OpKind::Add,
            '-' => OpKind::Sub,
            _ => panic!("unexpected char")
        }
    );

    // 足し算・引き算をパース
    let bin_parser = tuple
    ((
        term_parser,
        opt(tuple((op_kind_parser, expr_parser)))
    ));

    // パースされた値をmapで調整
    return map
    (
        bin_parser, |(head_expr, tail_expr_opt)|
        {
            if let Option::Some((op_kind, tail_expr)) = tail_expr_opt
            {
                return Expr::BinOp(Box::new(BinOp::new(op_kind, head_expr, tail_expr)));
            }
            else
            {
                return head_expr;
            }
        }
    )(s);
}

// 丸括弧パーサ
// BNF: <paren_expr> := '(' <expr> ')'
pub fn paren_expr_parser(s: &str) -> IResult<&str, Expr>
{
    let (no_used, _) = char('(')(s)?;
    let (no_used, expr) = expr_parser(no_used)?;
    let (no_used, _) = char(')')(no_used)?;

    return Ok((no_used, expr));
}

// 因子パーサ
// BNF: <factor> := <const_val> | <paren_expr>
pub fn factor_parser(s: &str) -> IResult<&str, Expr>
{
    // alt => 和集合

    return
    alt
    ((
        map
        (
            constant_val_parser,
            |constant_val| Expr::ConstantVal(constant_val)
        ),
        paren_expr_parser
    ))(s);
}

// 項パーサ
// BNF: <term> := <factor> [ ('*'|'/') <term> ]
pub fn term_parser(s: &str) -> IResult<&str, Expr>
{
    // * と / をOpKindに変換
    let op_kind_parser = map
    (
        alt((char('*'), char('/'))),
        |op_char|
        match op_char
        {
            '*' => OpKind::Mul,
            '/' => OpKind::Div,
            _ => panic!("unexpected op_char")
        }
    );

    // 掛け算・割り算をパース
    // tuple => 複数のパーサを結合
    // opt => あってもなくても良いパーサを生成
    let bin_parser = tuple
    ((
        factor_parser,
        opt
        (
            tuple
            ((
                op_kind_parser,
                term_parser
            ))
        )
    ));

    // パースされた値をmapで調整
    return map
    (
        bin_parser, |(head_expr, tail_expr_opt)|
        {
            if let Option::Some((op_kind, tail_expr)) = tail_expr_opt
            {
                return Expr::BinOp(Box::new(BinOp::new(op_kind, head_expr, tail_expr)));
            }
            else
            {
                return head_expr;
            }
        }
    )(s);
}

#[test]

fn constant_val_parser_test()
{
    let (_, actual) = constant_val_parser("123").unwrap();
    let expected = ConstantVal::new(123);
    assert_eq!(actual, expected);
}

#[test]
fn paren_expr_parser_test()
{
    let (_, actual) = paren_expr_parser("(123)").unwrap();
    let expected = Expr::ConstantVal(ConstantVal::new(123));
    assert_eq!(actual, expected);
}

#[test]
fn factor_parser_test()
{
    let (_, actual) = factor_parser("123").unwrap();
    let expected = Expr::ConstantVal(ConstantVal::new(123));
    assert_eq!(actual, expected);
}

#[test]
fn term_parser_test()
{
    let (_, actual) = term_parser("4*2/1").unwrap();
    let expected = Expr::BinOp(Box::new(BinOp::new(OpKind::Mul, Expr::ConstantVal(ConstantVal::new(4)), Expr::BinOp(Box::new(BinOp::new(OpKind::Div, Expr::ConstantVal(ConstantVal::new(2)), Expr::ConstantVal(ConstantVal::new(1))))))));
    assert_eq!(actual, expected);
}