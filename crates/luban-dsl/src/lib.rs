//! The luban equation language: Typst math syntax with luban semantics.
//!
//! Equations are written in Typst math mode, e.g.
//!
//! ```text
//! partial_t b - kappa nabla^2 b + lift(tau_1) = -u dot nabla b
//! ```
//!
//! Typst math is not an expression grammar: `typst-syntax` returns a flat sequence of atoms
//! (identifiers, text, attachments, fractions, calls, delimited groups) and leaves operator
//! precedence to the consumer. [`parse_equation`] runs the Typst parser, reports its syntax
//! errors, and splits the atom sequence at the single top-level `=` into a left-hand side
//! (implicit terms) and a right-hand side (explicit terms).

use typst_syntax::{SyntaxKind, SyntaxNode};

/// Errors produced while reading an equation.
#[derive(Debug, thiserror::Error, PartialEq, Eq)]
pub enum DslError {
    /// The Typst parser rejected the input.
    #[error("syntax error in `{text}`: {message}")]
    Syntax { text: String, message: String },
    /// The equation did not contain exactly one top-level `=`.
    #[error("expected exactly one top-level `=` in `{text}`, found {count}")]
    Relation { text: String, count: usize },
    /// One side of the `=` was empty.
    #[error("the {side}-hand side of `{text}` is empty")]
    EmptySide { text: String, side: &'static str },
}

/// An equation split into its two sides, still as Typst math atoms.
#[derive(Debug, Clone)]
pub struct Equation {
    text: String,
    lhs: Vec<SyntaxNode>,
    rhs: Vec<SyntaxNode>,
}

impl Equation {
    /// The source text of the equation.
    pub fn text(&self) -> &str {
        &self.text
    }

    /// Atoms of the left-hand side (implicit, linear terms).
    pub fn lhs(&self) -> &[SyntaxNode] {
        &self.lhs
    }

    /// Atoms of the right-hand side (explicit terms).
    pub fn rhs(&self) -> &[SyntaxNode] {
        &self.rhs
    }
}

fn is_relation(node: &SyntaxNode) -> bool {
    node.kind() == SyntaxKind::MathText && node.leaf_text() == "="
}

/// Parse one equation of the form `<lhs> = <rhs>`.
pub fn parse_equation(text: &str) -> Result<Equation, DslError> {
    let root = typst_syntax::parse_math(text);

    let (errors, _warnings) = root.errors_and_warnings();
    if let Some(first) = errors.first() {
        return Err(DslError::Syntax {
            text: text.to_owned(),
            message: first.message.to_string(),
        });
    }

    let atoms: Vec<SyntaxNode> = root
        .children()
        .filter(|n| !n.kind().is_trivia())
        .cloned()
        .collect();

    let positions: Vec<usize> = atoms
        .iter()
        .enumerate()
        .filter(|(_, n)| is_relation(n))
        .map(|(i, _)| i)
        .collect();
    if positions.len() != 1 {
        return Err(DslError::Relation {
            text: text.to_owned(),
            count: positions.len(),
        });
    }

    let split = positions[0];
    let rhs = atoms[split + 1..].to_vec();
    let mut lhs = atoms;
    lhs.truncate(split);

    for (side, atoms) in [("left", &lhs), ("right", &rhs)] {
        if atoms.is_empty() {
            return Err(DslError::EmptySide {
                text: text.to_owned(),
                side,
            });
        }
    }

    Ok(Equation {
        text: text.to_owned(),
        lhs,
        rhs,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn splits_buoyancy_equation() {
        let eq =
            parse_equation("partial_t b - kappa nabla^2 b + lift(tau_1) = -u dot nabla b").unwrap();
        // partial_t, b, -, kappa, nabla^2, b, +, lift(...)
        assert_eq!(eq.lhs().len(), 8);
        // -, u, dot, nabla, b
        assert_eq!(eq.rhs().len(), 5);
    }

    #[test]
    fn multi_letter_names_are_single_identifiers() {
        let eq = parse_equation("ub = u b").unwrap();
        assert_eq!(eq.lhs().len(), 1);
        assert_eq!(eq.lhs()[0].kind(), SyntaxKind::MathIdent);
        assert_eq!(eq.rhs().len(), 2);
    }

    #[test]
    fn boundary_condition_parses() {
        let eq = parse_equation("b(z: 0) = 0").unwrap();
        assert_eq!(eq.rhs().len(), 1);
    }

    #[test]
    fn rejects_missing_or_repeated_relation() {
        assert!(matches!(
            parse_equation("partial_t b"),
            Err(DslError::Relation { count: 0, .. })
        ));
        assert!(matches!(
            parse_equation("a = b = c"),
            Err(DslError::Relation { count: 2, .. })
        ));
    }

    #[test]
    fn rejects_empty_side() {
        assert!(matches!(
            parse_equation("= b"),
            Err(DslError::EmptySide { side: "left", .. })
        ));
    }
}
