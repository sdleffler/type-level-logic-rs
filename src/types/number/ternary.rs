type_operators! {
    [A, B, C, D, E]

    concrete Nat: Default => usize where #[derive(Default)] {
        Term => 0,
        Zero(X: Nat = Term) => 3 * X,
        One(X: Nat = Term) => 3 * X + 1,
        Two(X: Nat = Term) => 3 * X + 2,
        Undefined => panic!("Error: This type-level Nat value is undefined, and cannot be reified!"),
        #[cfg(feature = "specialization")]
        Error => panic!("Error: An unexpected, non-Nat type has been introduced into type-level arithmetic!"),
        #[cfg(feature = "specialization")]
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
pub type U28 = One<Zero<Zero<One<Term>>>>;
pub type U29 = Two<Zero<Zero<One<Term>>>>;
pub type U30 = Zero<One<Zero<One<Term>>>>;
pub type U31 = One<One<Zero<One<Term>>>>;
pub type U32 = Two<One<Zero<One<Term>>>>;
pub type U33 = Zero<Two<Zero<One<Term>>>>;
pub type U34 = One<Two<Zero<One<Term>>>>;
pub type U35 = Two<Two<Zero<One<Term>>>>;
pub type U36 = Zero<Zero<One<One<Term>>>>;
pub type U37 = One<Zero<One<One<Term>>>>;
pub type U38 = Two<Zero<One<One<Term>>>>;
pub type U39 = Zero<One<One<One<Term>>>>;
pub type U40 = One<One<One<One<Term>>>>;
pub type U41 = Two<One<One<One<Term>>>>;
pub type U42 = Zero<Two<One<One<Term>>>>;
pub type U43 = One<Two<One<One<Term>>>>;
pub type U44 = Two<Two<One<One<Term>>>>;
pub type U45 = Zero<Zero<Two<One<Term>>>>;
pub type U46 = One<Zero<Two<One<Term>>>>;
pub type U47 = Two<Zero<Two<One<Term>>>>;
pub type U48 = Zero<One<Two<One<Term>>>>;
pub type U49 = One<One<Two<One<Term>>>>;
pub type U50 = Two<One<Two<One<Term>>>>;
pub type U51 = Zero<Two<Two<One<Term>>>>;
pub type U52 = One<Two<Two<One<Term>>>>;
pub type U53 = Two<Two<Two<One<Term>>>>;
pub type U54 = Zero<Zero<Zero<Two<Term>>>>;
pub type U55 = One<Zero<Zero<Two<Term>>>>;
pub type U56 = Two<Zero<Zero<Two<Term>>>>;
pub type U57 = Zero<One<Zero<Two<Term>>>>;
pub type U58 = One<One<Zero<Two<Term>>>>;
pub type U59 = Two<One<Zero<Two<Term>>>>;
pub type U60 = Zero<Two<Zero<Two<Term>>>>;
pub type U61 = One<Two<Zero<Two<Term>>>>;
pub type U62 = Two<Two<Zero<Two<Term>>>>;
pub type U63 = Zero<Zero<One<Two<Term>>>>;
pub type U64 = One<Zero<One<Two<Term>>>>;
pub type U65 = Two<Zero<One<Two<Term>>>>;
pub type U66 = Zero<One<One<Two<Term>>>>;
pub type U67 = One<One<One<Two<Term>>>>;
pub type U68 = Two<One<One<Two<Term>>>>;
pub type U69 = Zero<Two<One<Two<Term>>>>;
pub type U70 = One<Two<One<Two<Term>>>>;
pub type U71 = Two<Two<One<Two<Term>>>>;
pub type U72 = Zero<Zero<Two<Two<Term>>>>;
pub type U73 = One<Zero<Two<Two<Term>>>>;
pub type U74 = Two<Zero<Two<Two<Term>>>>;
pub type U75 = Zero<One<Two<Two<Term>>>>;
pub type U76 = One<One<Two<Two<Term>>>>;
pub type U77 = Two<One<Two<Two<Term>>>>;
pub type U78 = Zero<Two<Two<Two<Term>>>>;
pub type U79 = One<Two<Two<Two<Term>>>>;
pub type U80 = Two<Two<Two<Two<Term>>>>;
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
        assert_eq!(U28::reify(), 28);
        assert_eq!(U29::reify(), 29);
        assert_eq!(U30::reify(), 30);
        assert_eq!(U31::reify(), 31);
        assert_eq!(U32::reify(), 32);
        assert_eq!(U33::reify(), 33);
        assert_eq!(U34::reify(), 34);
        assert_eq!(U35::reify(), 35);
        assert_eq!(U36::reify(), 36);
        assert_eq!(U37::reify(), 37);
        assert_eq!(U38::reify(), 38);
        assert_eq!(U39::reify(), 39);
        assert_eq!(U40::reify(), 40);
        assert_eq!(U41::reify(), 41);
        assert_eq!(U42::reify(), 42);
        assert_eq!(U43::reify(), 43);
        assert_eq!(U44::reify(), 44);
        assert_eq!(U45::reify(), 45);
        assert_eq!(U46::reify(), 46);
        assert_eq!(U47::reify(), 47);
        assert_eq!(U48::reify(), 48);
        assert_eq!(U49::reify(), 49);
        assert_eq!(U50::reify(), 50);
        assert_eq!(U51::reify(), 51);
        assert_eq!(U52::reify(), 52);
        assert_eq!(U53::reify(), 53);
        assert_eq!(U54::reify(), 54);
        assert_eq!(U55::reify(), 55);
        assert_eq!(U56::reify(), 56);
        assert_eq!(U57::reify(), 57);
        assert_eq!(U58::reify(), 58);
        assert_eq!(U59::reify(), 59);
        assert_eq!(U60::reify(), 60);
        assert_eq!(U61::reify(), 61);
        assert_eq!(U62::reify(), 62);
        assert_eq!(U63::reify(), 63);
        assert_eq!(U64::reify(), 64);
        assert_eq!(U65::reify(), 65);
        assert_eq!(U66::reify(), 66);
        assert_eq!(U67::reify(), 67);
        assert_eq!(U68::reify(), 68);
        assert_eq!(U69::reify(), 69);
        assert_eq!(U70::reify(), 70);
        assert_eq!(U71::reify(), 71);
        assert_eq!(U72::reify(), 72);
        assert_eq!(U73::reify(), 73);
        assert_eq!(U74::reify(), 74);
        assert_eq!(U75::reify(), 75);
        assert_eq!(U76::reify(), 76);
        assert_eq!(U77::reify(), 77);
        assert_eq!(U78::reify(), 78);
        assert_eq!(U79::reify(), 79);
        assert_eq!(U80::reify(), 80);
        assert_eq!(U81::reify(), 81);
        assert_eq!(U243::reify(), 243);
    }
}
