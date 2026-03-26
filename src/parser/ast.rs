use crate::lexer::token::{Token};

#[derive(Debug)]
pub enum Expr<'a> {
    Literal(Token<'a>),
    Grouping(Box<Expr<'a>>),
    Unary { operator: Token<'a>, right: Box<Expr<'a>> },
    Binary { left: Box<Expr<'a>>, operator: Token<'a>, right: Box<Expr<'a>> },
    // In the future: Variable, Assignment, etc.
}

#[derive(Debug)]
pub enum Stmt<'a> {
    Print(Expr<'a>),
    Expression(Expr<'a>),
    // Later: Var, If, While, etc.
    Var { name: Token<'a>, initializer: Option<Expr<'a>> },
}

// use crate::lexer::token::Token;

// #[derive(Debug)]
// pub enum Expr<'a> {
//     Literal(Token<'a>),
//     Grouping(Box<Expr<'a>>),
//     Unary { operator: Token<'a>, right: Box<Expr<'a>> },
//     Binary { left: Box<Expr<'a>>, operator: Token<'a>, right: Box<Expr<'a>> },

//     // Variables & assignment
//     Variable { name: Token<'a> },
//     Assign { name: Token<'a>, value: Box<Expr<'a>> },

//     // Logical operators (and/or short-circuit)
//     Logical { left: Box<Expr<'a>>, operator: Token<'a>, right: Box<Expr<'a>> },

//     // Calls / indexing / property access
//     Call { callee: Box<Expr<'a>>, paren: Token<'a>, arguments: Vec<Expr<'a>> },
//     Get { object: Box<Expr<'a>>, name: Token<'a> },
//     Set { object: Box<Expr<'a>>, name: Token<'a>, value: Box<Expr<'a>> },
//     Index { object: Box<Expr<'a>>, index: Box<Expr<'a>>, bracket: Token<'a> },

//     // Functions / lambdas
//     Function { params: Vec<Token<'a>>, body: Vec<Stmt<'a>> },
//     Lambda { params: Vec<Token<'a>>, body: Box<Expr<'a>> },

//     // Object-oriented
//     This,
//     Super { keyword: Token<'a>, method: Token<'a> },

//     // Collections / conditional
//     ArrayLiteral { elements: Vec<Expr<'a>> },
//     Conditional { condition: Box<Expr<'a>>, then_branch: Box<Expr<'a>>, else_branch: Option<Box<Expr<'a>>> },
// }

// #[derive(Debug)]
// pub enum Stmt<'a> {
//     Expression(Expr<'a>),
//     Print(Expr<'a>),

//     // Declarations & blocks
//     Var { name: Token<'a>, initializer: Option<Expr<'a>> },
//     Block { statements: Vec<Stmt<'a>> },

//     // Control flow
//     If { condition: Expr<'a>, then_branch: Box<Stmt<'a>>, else_branch: Option<Box<Stmt<'a>>> },
//     While { condition: Expr<'a>, body: Box<Stmt<'a>> },
//     For {
//         initializer: Option<Box<Stmt<'a>>>,
//         condition: Option<Expr<'a>>,
//         increment: Option<Expr<'a>>,
//         body: Box<Stmt<'a>>,
//     },

//     // Functions, classes, returns
//     Function { name: Token<'a>, params: Vec<Token<'a>>, body: Vec<Stmt<'a>> },
//     Return { keyword: Token<'a>, value: Option<Expr<'a>> },

//     Class {
//         name: Token<'a>,
//         superclass: Option<Expr<'a>>,
//         methods: Vec<Stmt<'a>>, // typically Function variant nodes
//     },

//     // Loop control
//     Break,
//     Continue,

//     // Exception handling (optional)
//     TryCatch {
//         try_block: Box<Stmt<'a>>,
//         catch_param: Option<Token<'a>>,
//         catch_block: Option<Box<Stmt<'a>>>,
//         finally: Option<Box<Stmt<'a>>>,
//     },

//     // Module system (placeholders)
//     Import { path: Token<'a> },
//     Export { name: Token<'a> },
// }
