use lalrpop_util::lalrpop_mod;


enum Value {
    Number(i32)

}

enum Expr{
    Value(Value) ,
    Stmt( Box<Expr > , Operation , Box<Expr>  )
}

enum Operation{
    Div,
    Mul,
    Sub,
    Add,
    Mod
}



lalrpop_mod!(pub language);


#[cfg(test)]
mod test {

    use crate::language;

    #[test]
    fn ee() {
        assert!(language::NumberParser::new().parse("22").is_ok());
    }


    macro_rules! test3 {
        ($expr:expr) => {
            println!("parsing {}", stringify!($expr));
            assert_eq!(
                language::ExprParser::new()
                    .parse(stringify!($expr))
                    .unwrap(),
                $expr
            );
        };
    }


    #[test]
    fn ttt() {
        test3!(34+35);
    }
}


fn main() {

}
