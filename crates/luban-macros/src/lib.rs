//! Compile-time equation macros for luban.
//!
//! `eq!` parses its argument while the crate compiles, so a malformed equation is a compiler
//! error rather than a runtime failure. Diagnostics cover the whole equation string, because a
//! proc macro on stable Rust cannot point at a sub-range of a string literal.

use proc_macro::TokenStream;
use quote::quote;
use syn::{LitStr, parse_macro_input};

/// Check an equation at compile time and produce a `luban::dsl::Equation`.
///
/// ```ignore
/// let eq = luban::eq!("partial_t b - kappa nabla^2 b + lift(tau_1) = -u dot nabla b");
/// ```
#[proc_macro]
pub fn eq(input: TokenStream) -> TokenStream {
    let lit = parse_macro_input!(input as LitStr);
    let text = lit.value();
    match luban_dsl::parse_equation(&text) {
        Ok(_) => quote! {
            ::luban::dsl::parse_equation(#lit)
                .expect("equation was validated by luban::eq! at compile time")
        }
        .into(),
        Err(err) => syn::Error::new(lit.span(), err.to_string())
            .to_compile_error()
            .into(),
    }
}
