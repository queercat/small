#[derive(Debug)]
enum Expression {
    Constant(Ast),
    Binary(BinaryOperator, Box<Expression>, Box<Expression>),
    Assignment(Ast, Box<Expression>),
    Invocation(Ast, Box<Expression>),
    Lambda(Vec<Ast>, Box<Expression>),
}

#[derive(Debug)]
enum BinaryOperator {
    Add,
    Subtract,
    Multiply,
    Divide,
}

#[derive(Debug)]
pub enum Ast {
    Symbol(String),
    Number(f64),
    Expression(Box<Expression>),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ast__with_basic_structure__works() {
        /// let x = 3;
        let ast = Ast::Expression(Box::new(Expression::Assignment(
            Ast::Symbol(String::from("x")),
            Box::new(Expression::Constant(Ast::Number(3f64))),
        )));
    }

    #[test]
    fn ast__with_function__works() {
        /// let x = lambda () {
        ///     x + 1
        /// }
        ///
        let ast = Ast::Expression(Box::new(Expression::Assignment(
            Ast::Symbol(String::from("x")),
            Box::new(Expression::Lambda(
                vec![],
                Box::new(Expression::Binary(
                    BinaryOperator::Add,
                    Box::new(Expression::Constant(Ast::Symbol(String::from("x")))),
                    Box::new(Expression::Constant(Ast::Number(1f64))),
                )),
            )),
        )));

        dbg!(ast);
    }
}
