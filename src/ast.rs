// 定数
#[derive(Debug,PartialEq)]
pub struct ConstantVal(i32);

impl ConstantVal
{
    pub fn new(value: i32) -> ConstantVal
    {
        return ConstantVal(value);
    }

    pub fn eval(&self) -> i32
    {
        return self.0;
    }
}

// 任意の式
#[derive(Debug,PartialEq)]
pub enum Expr
{
    ConstantVal(ConstantVal),
    BinOp(Box<BinOp>)
}

impl Expr
{
    // 式を評価
    pub fn eval(&self) -> i32
    {
        return match self
        {
            Expr::ConstantVal(c) => c.eval(),
            Expr::BinOp(p) => p.eval()
        };
    }
}

// 演算子
#[derive(Debug,PartialEq)]
pub enum OpKind
{
    Add,
    Sub,
    Mul,
    Div
}

// 二項演算
#[derive(Debug,PartialEq)]
pub struct BinOp
{
    op_kind: OpKind,
    left_expr: Expr,
    right_expr: Expr
}

impl BinOp
{
    pub fn new(op_kind: OpKind, left_expr: Expr, right_expr: Expr) -> BinOp
    {
        return BinOp { op_kind, left_expr, right_expr };
    }

    pub fn eval(&self) -> i32
    {
        return match self.op_kind
        {
            OpKind::Add => self.left_expr.eval() + self.right_expr.eval(),
            OpKind::Sub => self.left_expr.eval() - self.right_expr.eval(),
            OpKind::Mul => self.left_expr.eval() * self.right_expr.eval(),
            OpKind::Div => self.left_expr.eval() / self.right_expr.eval()
        };
    }
}

#[test]
fn test_constant_val()
{
    let a = ConstantVal::new(33);
    assert_eq!(a.eval(), 33);
}

#[test]
fn test_bin_op()
{
    // 13*(5+1)
    let plus_op = BinOp::new
    (
        OpKind::Mul,
        Expr::ConstantVal(ConstantVal::new(13)),
        Expr::BinOp(Box::new(BinOp::new
        (
            OpKind::Add,
            Expr::ConstantVal(ConstantVal::new(5)),
            Expr::ConstantVal(ConstantVal::new(1))
        )))
    );

    let expect = 13 * (5 + 1);

    assert_eq!(plus_op.eval(), expect);
}