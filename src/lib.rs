pub enum Expression {
    Concatenation {
        lhs: Box<Expression>,
        rhs: Box<Expression>,
    },
    Literal(Literal),
    Variable {
        name: Name,
    },
}

pub type Literal = String;

pub type Name = String;
