//! Type-level booleans.

type_operators! {
    [A, B, C, D, E]

    /// The `True` type reifies to `true` (as you might expect) and the `False` type reifies to
    /// `false`. If the crate is compiled with the `specialization` feature turned on, then an
    /// `Error` type is also present. When reified, the `Error` type panics with an error message
    /// explaining that the only way an `Error` type can be introduced into type-level boolean
    /// logic is through a non-`Bool` type being used. If `specialization` is on, a default
    /// implementation is also generated for *all* types. When `reify` is called on this default
    /// implementation, it panics with an error message explaining that the type is not a `Bool`.
    concrete Bool => bool {
        False => false,
        True => true,
        #[cfg(feature = "specialization")]
        Error => panic!("Error: An unexpected, non-Bool type has been introduced into type-level boolean logic!"),
        #[cfg(feature = "specialization")]
        DEFAULT => panic!("Error: This is not a Bool!"),
    }
}
