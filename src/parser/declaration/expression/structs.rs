use crate::lexer::token::Token;
use std::fmt;
use ExpressionKind::*;

pub enum ExpressionKind {
    BinaryExpr(BinaryExprBody),
    UnaryExpr(UnaryExprBody),
    LiteralExpr(LiteralExprBody),
    GroupingExpr(GroupingExprBody),
    VariableExpr(VariableExprBody),
    MatrixExpr(MatrixExprBody),
    ListExpr(ListExprBody),
}

pub struct ListExprBody {
    pub expressions: Vec<ExpressionKind>,
}

pub struct MatrixExprBody {
    pub list_expressions: Vec<ListExprBody>,
}

pub struct BinaryExprBody {
    pub left: Box<ExpressionKind>,
    pub operator: Token,
    pub right: Box<ExpressionKind>,
}

pub struct VariableExprBody {
    pub name: String,
}

pub struct UnaryExprBody {
    pub operator: Token,
    pub expression: Box<ExpressionKind>,
}

pub struct LiteralExprBody {
    pub value: Token,
}

pub struct GroupingExprBody {
    pub expression: Box<ExpressionKind>,
}

impl fmt::Display for ExpressionKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BinaryExpr(expr) => write!(f, "{}", expr),
            UnaryExpr(expr) => write!(f, "{}", expr),
            LiteralExpr(expr) => write!(f, "{}", expr),
            GroupingExpr(expr) => write!(f, "{}", expr),
            VariableExpr(expr) => write!(f, "{}", expr),
            ListExpr(expr) => write!(f, "{}", expr),
            MatrixExpr(expr) => write!(f, "{}", expr),
        }
    }
}

impl fmt::Display for BinaryExprBody {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({} {} {})", self.operator, self.left, self.right)
    }
}

impl fmt::Display for UnaryExprBody {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({} {})", self.operator, self.expression)
    }
}

impl fmt::Display for LiteralExprBody {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl fmt::Display for GroupingExprBody {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "(group {})", self.expression)
    }
}

impl fmt::Display for VariableExprBody {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}
impl fmt::Display for ListExprBody {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[")?;
        let n = self.expressions.len();
        for (i, expression) in self.expressions.iter().enumerate() {
            if i == n - 1 {
                write!(f, "{}", expression)?;
            } else {
                write!(f, "{}, ", expression)?;
            }
        }
        write!(f, "]")
    }
}

impl fmt::Display for MatrixExprBody {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[")?;
        for (i, matrix) in self.list_expressions.iter().enumerate() {
            if i != 0 {
                write!(f, "|\n|")?;
            }
            let m = matrix.expressions.len();
            for (j, expression) in matrix.expressions.iter().enumerate() {
                if j == m - 1 {
                    write!(f, "{}", expression)?;
                } else {
                    write!(f, "{}, ", expression)?;
                }
            }
        }
        write!(f, "]")
    }
}
