type_operators! {
    [A, B, C, D, E]

    concrete Bool => bool {
        False => false,
        True => true,
        #[cfg(feature = "specialization")]
        Error => panic!("Error: An unexpected, non-Bool type has been introduced into type-level boolean logic!"),
        #[cfg(feature = "specialization")]
        DEFAULT => panic!("Error: This is not a Bool!"),
    }
}
