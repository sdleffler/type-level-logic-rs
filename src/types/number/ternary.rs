type_operators! {
    [A, B, C, D, E]

    concrete Nat: Default => usize where #[derive(Default)] {
        Term => 0,
        Zero(X: Nat = Term) => 3 * X,
        One(X: Nat = Term) => 3 * X + 1,
        Two(X: Nat = Term) => 3 * X + 2,
        Undefined => panic!("Error: This type-level Nat value is undefined, and cannot be reified!"),
        #[cfg(features = "specialization")]
        Error => panic!("Error: An unexpected, non-Nat type has been introduced into type-level arithmetic!"),
        #[cfg(features = "specialization")]
        DEFAULT => panic!("Error: This is not a Nat!"),
    }

    concrete NatPair: Default => (usize, usize) where #[derive(Default)] {
        Nat2(X: Nat, Y: Nat) => (X, Y),
    }

    (Nat2First) Nat2P1(NatPair): Nat {
        forall (A: Nat, B: Nat) {
            [(Nat2 A B)] => A
        }
    }

    (Nat2Second) Nat2P2(NatPair): Nat {
        forall (A: Nat, B: Nat) {
            [(Nat2 A B)] => B
        }
    }
}


pub type U0 = Term;
pub type U1 = One<Term>;
pub type U2 = Two<Term>;
pub type U3 = Zero<One<Term>>;
pub type U4 = One<One<Term>>;
pub type U5 = Two<One<Term>>;
pub type U6 = Zero<Two<Term>>;
pub type U7 = One<Two<Term>>;
pub type U8 = Two<Two<Term>>;
pub type U9 = Zero<Zero<One<Term>>>;
pub type U10 = One<Zero<One<Term>>>;
pub type U11 = Two<Zero<One<Term>>>;
pub type U12 = Zero<One<One<Term>>>;
pub type U13 = One<One<One<Term>>>;
pub type U14 = Two<One<One<Term>>>;
pub type U15 = Zero<Two<One<Term>>>;
pub type U16 = One<Two<One<Term>>>;
pub type U17 = Two<Two<One<Term>>>;
pub type U18 = Zero<Zero<Two<Term>>>;
pub type U19 = One<Zero<Two<Term>>>;
pub type U20 = Two<Zero<Two<Term>>>;
pub type U21 = Zero<One<Two<Term>>>;
pub type U22 = One<One<Two<Term>>>;
pub type U23 = Two<One<Two<Term>>>;
pub type U24 = Zero<Two<Two<Term>>>;
pub type U25 = One<Two<Two<Term>>>;
pub type U26 = Two<Two<Two<Term>>>;
pub type U27 = Zero<Zero<Zero<One<Term>>>>;
pub type U81 = Zero<Zero<Zero<Zero<One<Term>>>>>;
pub type U243 = Zero<Zero<Zero<Zero<Zero<One<Term>>>>>>;


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ternary_constants() {
        assert_eq!(U0::reify(), 0);
        assert_eq!(U1::reify(), 1);
        assert_eq!(U2::reify(), 2);
        assert_eq!(U3::reify(), 3);
        assert_eq!(U4::reify(), 4);
        assert_eq!(U5::reify(), 5);
        assert_eq!(U6::reify(), 6);
        assert_eq!(U7::reify(), 7);
        assert_eq!(U8::reify(), 8);
        assert_eq!(U9::reify(), 9);
        assert_eq!(U10::reify(), 10);
        assert_eq!(U11::reify(), 11);
        assert_eq!(U12::reify(), 12);
        assert_eq!(U13::reify(), 13);
        assert_eq!(U14::reify(), 14);
        assert_eq!(U15::reify(), 15);
        assert_eq!(U16::reify(), 16);
        assert_eq!(U17::reify(), 17);
        assert_eq!(U18::reify(), 18);
        assert_eq!(U19::reify(), 19);
        assert_eq!(U20::reify(), 20);
        assert_eq!(U21::reify(), 21);
        assert_eq!(U22::reify(), 22);
        assert_eq!(U23::reify(), 23);
        assert_eq!(U24::reify(), 24);
        assert_eq!(U25::reify(), 25);
        assert_eq!(U26::reify(), 26);
        assert_eq!(U27::reify(), 27);
        assert_eq!(U81::reify(), 81);
        assert_eq!(U243::reify(), 243);
    }
}
