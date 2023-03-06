//! High-level representation of the source code.
//!
//! The HIR for Tin is an abstract syntax tree (AST) which
//! is constructed directly by parsing the source code input.
//!
//! Partial ASTs are constructed and built up throughout the
//! parsing process, with the complete AST available when parsing
//! is finished.
//!
//! This AST is then the basis for static analysis, optimization,
//! and code generation.
//!
//! Future versions of Tin may introduce further intermediate forms.

#![allow(dead_code)]

use std::borrow::Cow;
use std::collections::HashMap;
use std::fmt::{Display, Formatter, Result as FmtResult};

/// A complete parsed program.
///
/// A complete program is a collection of top-level statements
/// with the one requirement being the presence of a `main` function
/// declaration.
#[derive(Debug)]
pub struct Program<'prgrm>(pub Vec<TopStmt<'prgrm>>);

/// A top-level statement.
///
/// A few types of statements are allowed at the top of the
/// program, including comments, use-statements, and function
/// declarations.
#[derive(Debug)]
pub enum TopStmt<'prgrm> {
    /// A comment block.
    Comment(Comment<'prgrm>),
    /// A single use statement (may be expanded by glob resolution).
    Use(PathGlob<'prgrm>),
    /// A function declaration.
    FnDecl(FnDecl<'prgrm>),
    /// A type declaration.
    TyDecl(TyDecl<'prgrm>),
}

/// A statement.
///
/// Statements have no R-value. Generally expect them to be on their
/// own line.
#[derive(Debug)]
pub enum Stmt<'prgrm> {
    /// A comment is a statement.
    Comment(Comment<'prgrm>),
    /// Assigning, may be declaring a new variable.
    VarAssign(VarAssign<'prgrm>),
    /// An expression.
    ///
    /// Expressions may be treated as statements, implicitly throwing
    /// away the R-value.
    Expr(BExpr<'prgrm>),
}

/// A convenience type wrapping `Expr` in a `Box`.
type BExpr<'prgrm> = Box<Expr<'prgrm>>;

/// An expression.
///
/// Unlike statements, expressions _have_ R-values.
#[derive(Debug)]
pub enum Expr<'prgrm> {
    /// An `if` (optionally with an `else`)
    If(BExpr<'prgrm>, Block<'prgrm>, Option<Block<'prgrm>>),
    /// An `unless` (optionally with an `else`)
    Unless(BExpr<'prgrm>, Block<'prgrm>, Option<Block<'prgrm>>),
    /// An unconditional loop
    Loop(Block<'prgrm>),
    /// A `while` loop
    While(BExpr<'prgrm>, Block<'prgrm>),
    /// An `until` loop
    Until(BExpr<'prgrm>, Block<'prgrm>),
    /// A `for x in y` loop.
    For(BExpr<'prgrm>, BExpr<'prgrm>, Block<'prgrm>),
    /// A continue, jumping to the next loop iteration.
    Continue(&'prgrm str),
    /// A break, ending loop iteration, optionally with a value.
    Break(Option<BExpr<'prgrm>>, &'prgrm str),
    /// A function call.
    ///
    /// Function calls are also how operators are represented.
    FnCall(FnCall<'prgrm>),
    /// An identifier.
    Ident(Ident<'prgrm>),
    /// A field access.
    Dot(BExpr<'prgrm>, BExpr<'prgrm>),
}

/// A literal value.
#[derive(Debug)]
pub enum Literal<'prgrm> {
    /// An identifier (e.g. `some-name`)
    Ident(Ident<'prgrm>),
    /// A boolean value (e.g. `true` or `false`)
    Bool(Bool<'prgrm>),
    /// A 64-bit integer (e.g. `645`)
    Int(Int<'prgrm>),
    /// A 64-bit, double-precision floating point number (e.g. `5.36`)
    Float(Float<'prgrm>),
    /// A UTF-8 encoded string (e.g. `"hello"`)
    UStr(UStr<'prgrm>),
    /// An array of ASCII characters converted into bytes (e.g. `b"hello"`)
    BStr(BStr<'prgrm>),
    /// A UTF-8 code point (32-bits) (e.g. `'c'`)
    Char(Char<'prgrm>),
    /// An interned string value (e.g. `:hello`)
    Symbol(Symbol<'prgrm>),
    /// A sequence of contiguously-stored values of the same type (e.g. `#[1, 2, 3]`)
    Array(Array<'prgrm>),
    /// A sequence of values which may not be contiguously stored, and may have different
    /// types (e.g. `#(1, 'a', b"hello")`)
    Tuple(Tuple<'prgrm>),
    /// A hash map of symbols to values of the same type. (e.g. `#{ x: 2 + 3, y: -1 }`)
    Map(Map<'prgrm>),
}

/// A variable assignment.
///
/// If the variable identifier hasn't been declared in scope before,
/// this is treated as declaring a new variable.
#[derive(Debug)]
pub struct VarAssign<'prgrm> {
    /// The name of the variable being declared.
    pub name: Ident<'prgrm>,
    /// Optionally, a typing annotation.
    pub ty: Option<Ty<'prgrm>>,
    /// The RHS of the assignment, used to produce the value.
    pub rhs: BExpr<'prgrm>,
}

/// A type declaration.
///
/// Sum and product types are declared uniformly in Tin.
/// A type can have any number of variants, and each variant
/// can have a set of either named or unnamed fields.
#[derive(Debug)]
pub struct TyDecl<'prgrm>(pub Vec<TyVariant<'prgrm>>);

/// A single variant of a type.
///
/// A type variant may have a name. If only variant is present,
/// then the name is optional. If there are multiple variants,
/// they all must have names.
#[derive(Debug)]
pub struct TyVariant<'prgrm> {
    /// The name of the type.
    pub name: Option<Ident<'prgrm>>,
    /// The fields of the type.
    pub fields: Fields<'prgrm>,
}

/// The fields of a type variant.
///
/// Can either be named, in which case they include both the name
/// and the type, or anonymous, in which case they include only the
/// type.
#[derive(Debug)]
pub enum Fields<'prgrm> {
    /// All fields are named.
    Named(Vec<TyIdent<'prgrm>>),
    /// All fields are anonymous.
    Anonymous(Vec<Ty<'prgrm>>),
}

/// A combination of an identifier with a type.
///
/// This is generally used to express the assign of type to a variable
/// or field.
#[derive(Debug)]
pub struct TyIdent<'prgrm> {
    /// The identifier having a type assigned.
    pub ident: Ident<'prgrm>,
    /// The type being assigned.
    pub ty: Ty<'prgrm>,
}

/// A function call.
#[derive(Debug)]
pub struct FnCall<'prgrm> {
    /// The name of the function being called.
    pub name: Ident<'prgrm>,
    /// The actual args being passed.
    pub args: Vec<BExpr<'prgrm>>,
}

/// A function declaration.
#[derive(Debug)]
pub struct FnDecl<'prgrm> {
    /// The name of the new function.
    pub name: Ident<'prgrm>,
    /// The arguments to the function.
    pub args: Vec<TyIdent<'prgrm>>,
    /// (Optionally) the function return type.
    pub ret_ty: Option<Ty<'prgrm>>,
    /// The body of the function.
    pub body: Block<'prgrm>,
}

/// A type.
#[derive(Debug)]
pub struct Ty<'prgrm>(pub &'prgrm str);

/// A block of statements.
#[derive(Debug)]
pub struct Block<'prgrm>(pub Vec<Stmt<'prgrm>>);

/// A comment block.
///
/// Comments may be a reference to a string in the program (if single-line)
#[derive(Debug)]
pub enum Comment<'prgrm> {
    /// If a single line, a comment is a slice to the line.
    SingleLine(&'prgrm str),
    /// If multi-line, a comment is a collection of slices to each line.
    MultiLine(Vec<&'prgrm str>),
}

impl<'prgrm> Display for Comment<'prgrm> {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        use Comment::*;

        match self {
            SingleLine(comment) => write!(f, "{comment}"),
            // If multi-line, print all the lines.
            MultiLine(lines) => {
                for line in lines {
                    write!(f, "{line}")?
                }

                Ok(())
            }
        }
    }
}

/// A glob specifier for a path.
///
/// This may resolve into multiple paths.
#[derive(Debug)]
pub struct PathGlob<'prgrm>(pub &'prgrm str);

/// A single path to an external module.
///
/// Path contains a `Cow` to avoid allocating in cases where no globs are used.
#[derive(Debug)]
pub struct Path<'prgrm>(pub Cow<'prgrm, str>);

/// A boolean value.
#[derive(Debug)]
pub struct Bool<'prgrm>(pub bool, pub &'prgrm str);

/// A 64-bit signed integer.
#[derive(Debug)]
pub struct Int<'prgrm>(pub i64, pub &'prgrm str);

/// A 64-bit floating point number.
#[derive(Debug)]
pub struct Float<'prgrm>(pub f64, pub &'prgrm str);

/// An interned string used as keys in hashmaps or other
/// places where string identity is important.
#[derive(Debug)]
pub struct Symbol<'prgrm>(pub &'prgrm str);

/// A UTF-8 encoded string.
#[derive(Debug)]
pub struct UStr<'prgrm>(pub &'prgrm str);

/// A byte-vector built from ASCII text.
#[derive(Debug)]
pub struct BStr<'prgrm>(pub &'prgrm [u8], pub &'prgrm str);

/// An identifier.
#[derive(Debug)]
pub struct Ident<'prgrm>(pub &'prgrm str);

/// A UTF-8 code-point (32-bits).
#[derive(Debug)]
pub struct Char<'prgrm>(pub char, pub &'prgrm str);

/// A homogeneous container of values.
#[derive(Debug)]
pub struct Array<'prgrm>(pub Vec<Literal<'prgrm>>);

/// A heterogeneous container of values.
#[derive(Debug)]
pub struct Tuple<'prgrm>(pub Vec<Literal<'prgrm>>);

/// A hashmap of symbols to expressions.
#[derive(Debug)]
pub struct Map<'prgrm>(HashMap<Symbol<'prgrm>, BExpr<'prgrm>>);
