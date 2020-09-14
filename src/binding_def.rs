use crate::expr::Expr;
use crate::util::extract_whitespace;

#[derive(Debug, PartialEq)]
pub struct BindingDef {
    name: String,
    val: Expr,
}

impl BindingDef {
    pub fn new(s: &str) -> (&str, Self) {
        let s = if s.starts_with("let") {
            &s[3..]
        } else {
            panic!("expected let")
        };
        let (s, _) = extract_whitespace(s);

        let (s, name) = extract_ident(s);
        let (s, _) = extract_whitespace(s);

        let s = if s.starts_with('=') {
            &s[1..]
        } else {
            panic!("expected =")
        };
        let (s, _) = extract_whitespace(s);

        let (s, val) = Expr::new(s);

        (
            s,
            Self {
                name: name.to_string(),
                val,
            },
        )
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
                    val: Expr {
                        lhs: Number(10),
                        rhs: Number(2),
                        op: Op::Div,
                    },
                },
            ),
        );
    }
}