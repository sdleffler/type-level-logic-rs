//! Signed, unbounded type-level integers represented in balanced ternary form. `Int` constants
//! are provided, from `SN81` (signed, negative 81) to `S0` (signed zero) to `SP81` (signed,
//! positive 81). `SN243` and `SP243` are also provided.

type_operators! {
    [A, B, C, D, E]

    /// The `Int` kind is for signed, type-level integers. They are represented through balanced
    /// ternary in order to avoid a potentially non-unique representation of zero. [Balanced
    /// ternary](https://en.wikipedia.org/wiki/Balanced_ternary) is a *signed digit
    /// representation*. A signed digit representation is two things:
    ///
    /// 1. *Really cool.*
    /// 2. A number system in which digits can be multiples of negative one. If you want to go even
    ///    further down the Cool Spectrum, Donald Knuth suggested a *complex-base* number system
    ///    which has digits that are multiples of powers of `i`. I'm honestly not sure how useful
    ///    that is but *it sure is really cool!*
    ///
    /// In balanced ternary, we have three digits: zero, plus, and minus. So, here are the integers
    /// from negative five to positive five, in balanced ternary:
    ///
    /// Decimal | Balanced ternary (0+-) | Balanced ternary (01T) | Expression
    /// --------|------------------------|------------------------|-----------
    /// -5      | `-++`                  | `T11`                  | `3 * (3 * (3 * 0 - 1) + 1) + 1`
    /// -4      | `--`                   | `TT`                   | `3 * (3 * 0 - 1) - 1`
    /// -3      | `-0`                   | `T0`                   | `3 * (3 * 0 - 1)`
    /// -2      | `-+`                   | `T1`                   | `3 * (3 * 0 - 1) + 1`
    /// -1      | `-`                    | `T`                    | `3 * 0 - 1`
    /// 0       | `0`                    | `0`                    | `0`
    /// 1       | `+`                    | `1`                    | `3 * 0 + 1`
    /// 2       | `+-`                   | `1T`                   | `3 * (3 * 0 + 1) - 1`
    /// 3       | `+0`                   | `10`                   | `3 * (3 * 0 + 1)`
    /// 4       | `++`                   | `11`                   | `3 * (3 * 0 + 1) + 1`
    /// 5       | `+--`                  | `1TT`                  | `3 * (3 * (3 * 0 + 1) - 1) - 1`
    ///
    /// Like the ternary representation in this library, we use a type-level linked list to
    /// represent our balanced ternary numbers. Our digits are:
    ///
    /// - `Term` represents the end of our linked list - our "nil" - and also "zero".
    /// - `Zero<X>`, `Plus<X>`, and `Minus<X>` represent `3 * X`, `3 * X + 1`, and `3 * X - 1`,
    ///   respectively.
    ///
    /// Like the ternary representation, this linked list is stored with the least-significant
    /// digit at the head of the list. Precautions are taken to avoid issues with non-unique
    /// representations with redundant zeroes such as `Zero<Zero<Term>>` and if one *ever crops up
    /// in your code*, please lodge an issue! It is definitely caused by either a user error or a
    /// bug.
    ///
    /// `Int` can be reified to `isize`. Like `Nat`s, `Int`s cannot be reified to a constant
    /// expression due to limitations with Rust's associated const fns and constants. If and when
    /// these features are added to Rust in the future they will be added here too.
    ///
    /// `Int`s are always zero-sized, so you can store them directly in your `struct`s instead of
    /// using `PhantomData`. They implement `Default`.
    concrete Int: Default => isize where #[derive(Default)] {
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

    /// The `IntPair` trait and `Int2` struct represent 2-tuples of `Int`s. They are used
    /// internally for defining type-level logic.
    concrete IntPair: Default => (isize, isize) where #[derive(Default)] {
        Int2(X: Int, Y: Int) => (X, Y),
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
