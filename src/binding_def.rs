use crate::expr::Expr;

#[derive(Debug, PartialEq)]
pub struct BindingDef {
    name: String,
    var: Expr,
}

impl BindingDef {
    pub fn new(s: &str) -> (&str, Self) {
        todo!()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::expr::{Expr, Op, Number};

    #[test]
    fn parse_binding_def() {
        assert_eq!(
            BindingDef::new("let a = 10 / 2"),
            (
                "",
                BindingDef {
                    name: "a".to_string(),
                    var: Expr {
                        lhs: Number(10),
                        rhs: Number(2),
                        op: Op::Div,
                    },
                },
            ),
        );
    }
}