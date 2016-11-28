type_operators! {
    [A, B, C, D, E]

    concrete Int => isize {
        Term => 0,
        Zero(X: Int = Term) => 3 * X,
        Plus(X: Int = Term) => 3 * X + 1,
        Minus(X: Int = Term) => 3 * X - 1,
        Undefined => panic!("Error: This type-level Int value is undefined, and cannot be reified!"),
        #[cfg(feature = "specialization")]
        Error => panic!("Error: An unexpected, non-Int type has been introduced into type-level arithmetic!"),
        #[cfg(feature = "specialization")]
        DEFAULT => panic!("Error: This is not an Int!"),
    }

    concrete IntPair => (isize, isize) {
        Int2(X: Int, Y: Int) => (X, Y),
    }

    (Int2First) Int2P1(IntPair): Int {
        forall (A: Int, B: Int) {
            [(Int2 A B)] => A
        }
    }

    (Int2Second) Int2P2(IntPair): Int {
        forall (A: Int, B: Int) {
            [(Int2 A B)] => B
        }
    }
}


pub type SN243 = Zero<Zero<Zero<Zero<Zero<Minus<Term>>>>>>;
pub type SN81 = Zero<Zero<Zero<Zero<Minus<Term>>>>>;
pub type SN27 = Zero<Zero<Zero<Minus<Term>>>>;
pub type SN26 = Plus<Zero<Zero<Minus<Term>>>>;
pub type SN25 = Minus<Plus<Zero<Minus<Term>>>>;
pub type SN24 = Zero<Plus<Zero<Minus<Term>>>>;
pub type SN23 = Plus<Plus<Zero<Minus<Term>>>>;
pub type SN22 = Minus<Minus<Plus<Minus<Term>>>>;
pub type SN21 = Zero<Minus<Plus<Minus<Term>>>>;
pub type SN20 = Plus<Minus<Plus<Minus<Term>>>>;
pub type SN19 = Minus<Zero<Plus<Minus<Term>>>>;
pub type SN18 = Zero<Zero<Plus<Minus<Term>>>>;
pub type SN17 = Plus<Zero<Plus<Minus<Term>>>>;
pub type SN16 = Minus<Plus<Plus<Minus<Term>>>>;
pub type SN15 = Zero<Plus<Plus<Minus<Term>>>>;
pub type SN14 = Plus<Plus<Plus<Minus<Term>>>>;
pub type SN13 = Minus<Minus<Minus<Term>>>;
pub type SN12 = Zero<Minus<Minus<Term>>>;
pub type SN11 = Plus<Minus<Minus<Term>>>;
pub type SN10 = Minus<Zero<Minus<Term>>>;
pub type SN9 = Zero<Zero<Minus<Term>>>;
pub type SN8 = Plus<Zero<Minus<Term>>>;
pub type SN7 = Minus<Plus<Minus<Term>>>;
pub type SN6 = Zero<Plus<Minus<Term>>>;
pub type SN5 = Plus<Plus<Minus<Term>>>;
pub type SN4 = Minus<Minus<Term>>;
pub type SN3 = Zero<Minus<Term>>;
pub type SN2 = Plus<Minus<Term>>;
pub type SN1 = Minus<Term>;
pub type S0 = Term;
pub type SP1 = Plus<Term>;
pub type SP2 = Minus<Plus<Term>>;
pub type SP3 = Zero<Plus<Term>>;
pub type SP4 = Plus<Plus<Term>>;
pub type SP5 = Minus<Minus<Plus<Term>>>;
pub type SP6 = Zero<Minus<Plus<Term>>>;
pub type SP7 = Plus<Minus<Plus<Term>>>;
pub type SP8 = Minus<Zero<Plus<Term>>>;
pub type SP9 = Zero<Zero<Plus<Term>>>;
pub type SP10 = Plus<Zero<Plus<Term>>>;
pub type SP11 = Minus<Plus<Plus<Term>>>;
pub type SP12 = Zero<Plus<Plus<Term>>>;
pub type SP13 = Plus<Plus<Plus<Term>>>;
pub type SP14 = Minus<Minus<Minus<Plus<Term>>>>;
pub type SP15 = Zero<Minus<Minus<Plus<Term>>>>;
pub type SP16 = Plus<Minus<Minus<Plus<Term>>>>;
pub type SP17 = Minus<Zero<Minus<Plus<Term>>>>;
pub type SP18 = Zero<Zero<Minus<Plus<Term>>>>;
pub type SP19 = Plus<Zero<Minus<Plus<Term>>>>;
pub type SP20 = Minus<Plus<Minus<Plus<Term>>>>;
pub type SP21 = Zero<Plus<Minus<Plus<Term>>>>;
pub type SP22 = Plus<Plus<Minus<Plus<Term>>>>;
pub type SP23 = Minus<Minus<Zero<Plus<Term>>>>;
pub type SP24 = Zero<Minus<Zero<Plus<Term>>>>;
pub type SP25 = Plus<Minus<Zero<Plus<Term>>>>;
pub type SP26 = Minus<Zero<Zero<Plus<Term>>>>;
pub type SP27 = Zero<Zero<Zero<Plus<Term>>>>;
pub type SP81 = Zero<Zero<Zero<Zero<Plus<Term>>>>>;
pub type SP243 = Zero<Zero<Zero<Zero<Zero<Plus<Term>>>>>>;


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn balanced_ternary_constants() {
        assert_eq!(SN243::reify(), -243);
        assert_eq!(SN81::reify(), -81);
        assert_eq!(SN27::reify(), -27);
        assert_eq!(SN26::reify(), -26);
        assert_eq!(SN25::reify(), -25);
        assert_eq!(SN24::reify(), -24);
        assert_eq!(SN23::reify(), -23);
        assert_eq!(SN22::reify(), -22);
        assert_eq!(SN21::reify(), -21);
        assert_eq!(SN20::reify(), -20);
        assert_eq!(SN19::reify(), -19);
        assert_eq!(SN18::reify(), -18);
        assert_eq!(SN17::reify(), -17);
        assert_eq!(SN16::reify(), -16);
        assert_eq!(SN15::reify(), -15);
        assert_eq!(SN14::reify(), -14);
        assert_eq!(SN13::reify(), -13);
        assert_eq!(SN12::reify(), -12);
        assert_eq!(SN11::reify(), -11);
        assert_eq!(SN10::reify(), -10);
        assert_eq!(SN9::reify(), -9);
        assert_eq!(SN8::reify(), -8);
        assert_eq!(SN7::reify(), -7);
        assert_eq!(SN6::reify(), -6);
        assert_eq!(SN5::reify(), -5);
        assert_eq!(SN4::reify(), -4);
        assert_eq!(SN3::reify(), -3);
        assert_eq!(SN2::reify(), -2);
        assert_eq!(SN1::reify(), -1);
        assert_eq!(S0::reify(), 0);
        assert_eq!(SP1::reify(), 1);
        assert_eq!(SP2::reify(), 2);
        assert_eq!(SP3::reify(), 3);
        assert_eq!(SP4::reify(), 4);
        assert_eq!(SP5::reify(), 5);
        assert_eq!(SP6::reify(), 6);
        assert_eq!(SP7::reify(), 7);
        assert_eq!(SP8::reify(), 8);
        assert_eq!(SP9::reify(), 9);
        assert_eq!(SP10::reify(), 10);
        assert_eq!(SP11::reify(), 11);
        assert_eq!(SP12::reify(), 12);
        assert_eq!(SP13::reify(), 13);
        assert_eq!(SP14::reify(), 14);
        assert_eq!(SP15::reify(), 15);
        assert_eq!(SP16::reify(), 16);
        assert_eq!(SP17::reify(), 17);
        assert_eq!(SP18::reify(), 18);
        assert_eq!(SP19::reify(), 19);
        assert_eq!(SP20::reify(), 20);
        assert_eq!(SP21::reify(), 21);
        assert_eq!(SP22::reify(), 22);
        assert_eq!(SP23::reify(), 23);
        assert_eq!(SP24::reify(), 24);
        assert_eq!(SP25::reify(), 25);
        assert_eq!(SP26::reify(), 26);
        assert_eq!(SP27::reify(), 27);
        assert_eq!(SP81::reify(), 81);
        assert_eq!(SP243::reify(), 243);
    }
}
