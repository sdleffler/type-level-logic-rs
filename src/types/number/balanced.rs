type_operators! {
    [A, B, C, D, E]

    concrete Int => isize {
        Zero => 0,
        Plus(X: Int = Zero) => 3 * X + 1,
        Minus(X: Int = Zero) => 3 * X - 1,
        Undefined => panic!("Error: This type-level Int value is undefined, and cannot be reified!"),
        #[cfg(features = "specialization")]
        Error => panic!("Error: An unexpected, non-Int type has been introduced into type-level arithmetic!"),
        #[cfg(features = "specialization")]
        DEFAULT => panic!("Error: This is not a Int!"),
    }
}
