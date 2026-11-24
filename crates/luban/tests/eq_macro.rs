#[test]
fn eq_macro_expands_to_a_parsed_equation() {
    let eq = luban::eq!("partial_t b - kappa nabla^2 b + lift(tau_1) = -u dot nabla b");
    assert_eq!(eq.lhs().len(), 8);
    assert_eq!(eq.rhs().len(), 5);
}
