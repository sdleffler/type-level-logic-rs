use std::default::Default;


type_operators! {
    [A, B, C, D, E]

    concrete Nat: Default => usize where #[derive(Default)] {
        P => 0,
        O(N: Nat) => 2 * N,
        I(N: Nat) => 1 + 2 * N,
        Undefined => panic!("Error: This type-level Nat value is undefined, and cannot be reified!"),
        #[cfg(features = "specialization")]
        Error => panic!("Error: An unexpected, non-Nat type has been introduced into type-level arithmetic!"),
        #[cfg(features = "specialization")]
        DEFAULT => panic!("Error: This is not a Nat!"),
    }
}
