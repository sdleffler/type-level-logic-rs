//! (Unbalanced) type-level ternary operations, with "strongly" enforced validity. In this module
//! documentation, unsigned ternary numbers will be referred to as `Nat`s (because they're a
//! representation of the natural numbers), "natural numbers" (see preceding), or "unsigned
//! numbers".

use types::number::ternary::{NatPair, Nat2, Nat, Undefined, Term, Zero, One, Two};

type_operators! {
    [A, B, C, D, E, F, G, H]

    /// Project the first value in a `NatPair`.
    (Nat2First) Nat2P1(NatPair): Nat {
        forall (A: Nat, B: Nat) {
            [(Nat2 A B)] => A
        }
    }

    /// Project the second value in a `NatPair`.
    (Nat2Second) Nat2P2(NatPair): Nat {
        forall (A: Nat, B: Nat) {
            [(Nat2 A B)] => B
        }
    }

    /// The `Succ` operator adds one to a `Nat`. It is always defined. It can be used as `Succ<X>`
    /// or `<X as NatSucc>::Output`.
    (Succ) NatSucc(Nat): Nat {
        [Undefined] => Undefined
        [Term] => One
        forall (X: Nat) {
            [(Zero X)] => (One X)
            [(One X)] => (Two X)
            [(Two X)] => (Zero (# X))
        }
    }

    /// The `Pred` operator subtracts one from a `Nat`. It is defined for non-zero `Nat`s, and can
    /// be used as `Pred<X>` or `<X as NatPred>::Output`.
    (Pred) NatPred(Nat): Nat {
        [Undefined] => Undefined
        [Term] => Undefined
        forall (X: Nat) {
            [(Zero X)] => (Two (# X))
            [(One X)] => (@NatTriple X)
            [(Two X)] => (One X)
        }
    }

    /// The `DoublePred` operator subtracts two from a `Nat`. It is defined for `Nat`s greater
    /// than one, and can be used as `DoublePred<X>` or `<X as NatDoublePred>::Output`.
    (DoublePred) NatDoublePred(Nat): Nat {
        [Undefined] => Undefined
        [Term] => Undefined
        forall (X: Nat) {
            [(Zero X)] => (One (@NatPred X))
            [(One X)] => (Two (@NatPred X))
            [(Two X)] => (@NatTriple X)
        }
    }

    /// The `Triple` operator triples a `Nat`, and avoids a single level of redundant zeroes
    /// (and thus helps to preserve unique representations.) It is equivalent to `Zero<N>` unless
    /// `N` is `Term`, in which case `Triple<Term> = Term`. Its collapsing acts at only one level,
    /// which should be sufficient unless multiple levels of redundant zeroes are introduced by
    /// user error. It also propagates undefined values: `Triple<Undefined> = Undefined`.
    (Triple) NatTriple(Nat): Nat {
        [Undefined] => Undefined
        [Term] => Term
        forall (N: Nat) {
            [(Zero N)] => (Zero (Zero N))
            [(One N)] => (Zero (One N))
            [(Two N)] => (Zero (Two N))
        }
    }

    /// The `Unique` operator collapses one level of redundant zeroes in a ternary representation.
    /// Using `Unique` every time we might get a redundant zero *should* get rid of any potential
    /// problems with non-unique representations of zero.
    (Unique) NatUnique(Nat): Nat {
        [Undefined] => Undefined
        [Term] => Term
        [(Zero Term)] => Term
        forall (N: Nat) {
            [(Zero (Zero N))] => (Zero (# (Zero N)))
            [(Zero (One N))] => (Zero (One (# N)))
            [(Zero (Two N))] => (Zero (Two (# N)))
            [(One N)] => (One N)
            [(Two N)] => (Two N)
        }
    }

    /// `Nat` addition. Used as `Add<X, Y>` or `<X as NatAdd<Y>>::Output`.
    (Add) NatAdd(Nat, Nat): Nat {
        [Term, Term] => Term
        [Undefined, Term] => Undefined
        [Term, Undefined] => Undefined
        [Undefined, Undefined] => Undefined
        forall (X: Nat) {
            [Term, (Zero X)] => (@NatTriple X)
            [Term, (One X)] => (One X)
            [Term, (Two X)] => (Two X)
            [(Zero X), Term] => (@NatTriple X)
            [(One X), Term] => (One X)
            [(Two X), Term] => (Two X)
            [Undefined, (Zero X)] => Undefined
            [Undefined, (One X)] => Undefined
            [Undefined, (Two X)] => Undefined
            [(Zero X), Undefined] => Undefined
            [(One X), Undefined] => Undefined
            [(Two X), Undefined] => Undefined
        }
        forall (X: Nat, Y: Nat) {
            [(Zero X), (Zero Y)] => (@NatTriple (# X Y))
            [(Zero X), (One Y)] => (One (# X Y))
            [(Zero X), (Two Y)] => (Two (# X Y))
            [(One X), (Zero Y)] => (One (# X Y))
            [(One X), (One Y)] => (Two (# X Y))
            [(One X), (Two Y)] => (Zero (@NatSucc (# X Y)))
            [(Two X), (Zero Y)] => (Two (# X Y))
            [(Two X), (One Y)] => (Zero (@NatSucc (# X Y)))
            [(Two X), (Two Y)] => (One (@NatSucc (# X Y)))
        }
    }

    /// A convenience operator for propagating undefined values.
    (TriplePlusOne) NatTriplePlusOne(Nat): Nat {
        [Undefined] => Undefined
        [Term] => One
        forall (X: Nat) {
            [(Zero X)] => (One (Zero X))
            [(One X)] => (One (One X))
            [(Two X)] => (One (Two X))
        }
    }

    /// A convenience operator for propagating undefined values.
    (TriplePlusTwo) NatTriplePlusTwo(Nat): Nat {
        [Undefined] => Undefined
        [Term] => Two
        forall (X: Nat) {
            [(Zero X)] => (Two (Zero X))
            [(One X)] => (Two (One X))
            [(Two X)] => (Two (Two X))
        }
    }

    /// `Nat` subtraction. Used as `Sub<X, Y>` or `<X as NatSub<Y>>::Output`. Undefined for any
    /// inputs which would result in an output less than zero. If you need signed numbers, you
    /// should probably be using a signed representation, such as balanced ternary.
    (Sub) NatSub(Nat, Nat): Nat {
        [Term, Term] => Term
        [Undefined, Term] => Undefined
        [Term, Undefined] => Undefined
        [Undefined, Undefined] => Undefined
        forall (X: Nat) {
            [Term, (Zero X)] => (# Term X)
            [Term, (One X)] => Undefined
            [Term, (Two X)] => Undefined
            [(Zero X), Term] => (@NatTriple X)
            [(One X), Term] => (One X)
            [(Two X), Term] => (Two X)
            [Undefined, (Zero X)] => Undefined
            [Undefined, (One X)] => Undefined
            [Undefined, (Two X)] => Undefined
            [(Zero X), Undefined] => Undefined
            [(One X), Undefined] => Undefined
            [(Two X), Undefined] => Undefined
        }
        forall (X: Nat, Y: Nat) {
            [(Zero X), (Zero Y)] => (@NatTriple (# X Y))
            [(Zero X), (One Y)] => (@NatTriplePlusTwo (@NatPred (# X Y)))
            [(Zero X), (Two Y)] => (@NatTriplePlusOne (@NatPred (# X Y)))
            [(One X), (Zero Y)] => (@NatTriplePlusOne (# X Y))
            [(One X), (One Y)] => (@NatTriple (# X Y))
            [(One X), (Two Y)] => (@NatTriplePlusTwo (@NatPred (# X Y)))
            [(Two X), (Zero Y)] => (@NatTriplePlusTwo (# X Y))
            [(Two X), (One Y)] => (@NatTriplePlusOne (# X Y))
            [(Two X), (Two Y)] => (@NatTriple (# X Y))
        }
    }

    /// `Nat` multiplication. Used as `Mul<X, Y>` or `<X as NatMul<Y>>::Output`.
    (Mul) NatMul(Nat, Nat): Nat {
        [Term, Term] => Term
        [Undefined, Term] => Undefined
        [Term, Undefined] => Undefined
        [Undefined, Undefined] => Undefined
        forall (X: Nat) {
            [Term, (Zero X)] => Term
            [Term, (One X)] => Term
            [Term, (Two X)] => Term
            [(Zero X), Term] => Term
            [(One X), Term] => Term
            [(Two X), Term] => Term
            [Undefined, (Zero X)] => Undefined
            [Undefined, (One X)] => Undefined
            [Undefined, (Two X)] => Undefined
            [(Zero X), Undefined] => Undefined
            [(One X), Undefined] => Undefined
            [(Two X), Undefined] => Undefined
        }
        forall (X: Nat, Y: Nat) {
            [(Zero X), (Zero Y)] => (# X (Zero (Zero Y)))
            [(Zero X), (One Y)] => (# X (Zero (One Y)))
            [(Zero X), (Two Y)] => (# X (Zero (Two Y)))
            [(One X), (Zero Y)] => (@NatAdd (# X (Zero (Zero Y))) (Zero Y))
            [(One X), (One Y)] => (@NatAdd (# X (Zero (One Y))) (One Y))
            [(One X), (Two Y)] => (@NatAdd (# X (Zero (Two Y))) (Two Y))
            [(Two X), (Zero Y)] => (@NatAdd (# (One X) (Zero Y)) (Zero Y))
            [(Two X), (One Y)] => (@NatAdd (# (One X) (One Y)) (One Y))
            [(Two X), (Two Y)] => (@NatAdd (# (One X) (Two Y)) (Two Y))
        }
    }

    /// `Nat` comparison. If the first argument is less than the second, return the third argument;
    /// else if the first argument is equal to the second, then return the fourth argument; else,
    /// return the fifth argument.
    (Cmp) NatCmp(Nat, Nat, Nat, Nat, Nat): Nat {
        forall (L: Nat, E: Nat, G: Nat) {
            [Term, Term, L, E, G] => E
        }
        forall (X: Nat, L: Nat, E: Nat, G: Nat) {
            [Term, (Zero X), L, E, G] => (# Term X L E G)
            [Term, (One X), L, E, G] => L
            [Term, (Two X), L, E, G] => L
            [(Zero X), Term, L, E, G] => (# X Term L E G)
            [(One X), Term, L, E, G] => G
            [(Two X), Term, L, E, G] => G
        }
        forall (X: Nat, Y: Nat, L: Nat, E: Nat, G: Nat) {
            [(Zero X), (Zero Y), L, E, G] => (# X Y L E G)
            [(Zero X), (One Y), L, E, G] => (# X Y L L G)
            [(Zero X), (Two Y), L, E, G] => (# X Y L L G)
            [(One X), (Zero Y), L, E, G] => (# X Y L G G)
            [(One X), (One Y), L, E, G] => (# X Y L E G)
            [(One X), (Two Y), L, E, G] => (# X Y L L G)
            [(Two X), (Zero Y), L, E, G] => (# X Y L G G)
            [(Two X), (One Y), L, E, G] => (# X Y L G G)
            [(Two X), (Two Y), L, E, G] => (# X Y L E G)
        }
    }

    /// `Nat` undefined-or with pair - if the first argument is undefined, return the pair passed
    /// in as the third argument; else, return the first paired with the second argument.
    (UndefOr2) NatUndefOr2(Nat, Nat, NatPair): NatPair {
        forall (A: Nat, BC: NatPair) {
            [Undefined, A, BC] => BC
            [Term, A, BC] => (Nat2 Term A)
        }
        forall (X: Nat, A: Nat, BC: NatPair) {
            [(Zero X), A, BC] => (Nat2 (Zero X) A)
            [(One X), A, BC] => (Nat2 (One X) A)
            [(Two X), A, BC] => (Nat2 (Two X) A)
        }
    }

    /// `Nat` *reversal.* This is probably something you *never* want to do! However, it's useful
    /// in implementing division, in which we wish to access trits with the most-significant trits
    /// first. This reverses all the trits of the integer.
    (Rev) NatRev(Nat): Nat {
        forall (N: Nat) {
            [N] => (@NatRevInternal N Term)
        }
    }

    (RevInternal) NatRevInternal(Nat, Nat): Nat {
        forall (N: Nat) {
            [Term, N] => N
        }
        forall (M: Nat, N: Nat) {
            [(Zero M), N] => (# M (Zero N))
            [(One M), N] => (# M (One N))
            [(Two M), N] => (# M (Two N))
        }
    }

    /// `Nat` truncating division. Used as `Div<X, Y>` or `<X as NatDiv<Y>>::Output`. Returns
    /// `Undefined` in the case of a division by zero.
    (Div) NatDiv(Nat, Nat): Nat {
        [Term, Term] => Undefined
        [Term, Undefined] => Undefined
        [Undefined, Term] => Undefined
        [Undefined, Undefined] => Undefined
        forall (D: Nat) {
            [Term, (Zero D)] => Term
            [Term, (One D)] => Term
            [Term, (Two D)] => Term
        }
        forall (N: Nat, D: Nat) {
            [(Zero N), (Zero D)] => (@Nat2P2 (@NatDivInternal (@NatRev (Zero N)) (Zero D) (Nat2 Term Term)))
            [(Zero N), (One D)] => (@Nat2P2 (@NatDivInternal (@NatRev (Zero N)) (One D) (Nat2 Term Term)))
            [(Zero N), (Two D)] => (@Nat2P2 (@NatDivInternal (@NatRev (Zero N)) (Two D) (Nat2 Term Term)))
            [(One N), (Zero D)] => (@Nat2P2 (@NatDivInternal (@NatRev (One N)) (Zero D) (Nat2 Term Term)))
            [(One N), (One D)] => (@Nat2P2 (@NatDivInternal (@NatRev (One N)) (One D) (Nat2 Term Term)))
            [(One N), (Two D)] => (@Nat2P2 (@NatDivInternal (@NatRev (One N)) (Two D) (Nat2 Term Term)))
            [(Two N), (Zero D)] => (@Nat2P2 (@NatDivInternal (@NatRev (Two N)) (Zero D) (Nat2 Term Term)))
            [(Two N), (One D)] => (@Nat2P2 (@NatDivInternal (@NatRev (Two N)) (One D) (Nat2 Term Term)))
            [(Two N), (Two D)] => (@Nat2P2 (@NatDivInternal (@NatRev (Two N)) (Two D) (Nat2 Term Term)))
        }
    }

    /// `Nat` remainder. Used as `Rem<X, Y>` or `<X as NatRem<Y>>::Output`. Returns
    /// `Undefined` in the case of a division by zero.
    (Rem) NatRem(Nat, Nat): Nat {
        [Term, Term] => Undefined
        [Term, Undefined] => Undefined
        [Undefined, Term] => Undefined
        [Undefined, Undefined] => Undefined
        forall (D: Nat) {
            [Term, (Zero D)] => Term
            [Term, (One D)] => Term
            [Term, (Two D)] => Term
        }
        forall (N: Nat, D: Nat) {
            [(Zero N), (Zero D)] => (@Nat2P1 (@NatDivInternal (@NatRev (Zero N)) (Zero D) (Nat2 Term Term)))
            [(Zero N), (One D)] => (@Nat2P1 (@NatDivInternal (@NatRev (Zero N)) (One D) (Nat2 Term Term)))
            [(Zero N), (Two D)] => (@Nat2P1 (@NatDivInternal (@NatRev (Zero N)) (Two D) (Nat2 Term Term)))
            [(One N), (Zero D)] => (@Nat2P1 (@NatDivInternal (@NatRev (One N)) (Zero D) (Nat2 Term Term)))
            [(One N), (One D)] => (@Nat2P1 (@NatDivInternal (@NatRev (One N)) (One D) (Nat2 Term Term)))
            [(One N), (Two D)] => (@Nat2P1 (@NatDivInternal (@NatRev (One N)) (Two D) (Nat2 Term Term)))
            [(Two N), (Zero D)] => (@Nat2P1 (@NatDivInternal (@NatRev (Two N)) (Zero D) (Nat2 Term Term)))
            [(Two N), (One D)] => (@Nat2P1 (@NatDivInternal (@NatRev (Two N)) (One D) (Nat2 Term Term)))
            [(Two N), (Two D)] => (@Nat2P1 (@NatDivInternal (@NatRev (Two N)) (Two D) (Nat2 Term Term)))
        }
    }

    (DivInternal) NatDivInternal(Nat, Nat, NatPair): NatPair {
        forall (D: Nat, RQ: NatPair) {
            [Term, D, RQ] => RQ
        }
        forall (N: Nat, D: Nat, R: Nat, Q: Nat) {
            [(Zero N), D, (Nat2 R Q)] => (# N D
                (@NatUndefOr2
                    (@NatSub (@NatSub (@NatTriple R) D) D) (Two Q)
                    (@NatUndefOr2
                        (@NatSub (@NatTriple R) D) (One Q)
                        (Nat2 (@NatTriple R) (@NatTriple Q)))))
            [(One N), D, (Nat2 R Q)] => (# N D
                (@NatUndefOr2
                    (@NatSub (@NatSub (One R) D) D) (Two Q)
                    (@NatUndefOr2
                        (@NatSub (One R) D) (One Q)
                        (Nat2 (One R) (@NatTriple Q)))))
            [(Two N), D, (Nat2 R Q)] => (# N D
                (@NatUndefOr2
                    (@NatSub (@NatSub (Two R) D) D) (Two Q)
                    (@NatUndefOr2
                        (@NatSub (Two R) D) (One Q)
                        (Nat2 (Two R) (@NatTriple Q)))))
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use types::number::ternary::*;

    #[test]
    fn ternary_succ() {
        assert_eq!(<Succ<U0> as Nat>::reify(), 1);
        assert_eq!(<Succ<U7> as Nat>::reify(), 8);
        assert_eq!(<Succ<U2> as Nat>::reify(), 3);
        assert_eq!(<Succ<U9> as Nat>::reify(), 10);
        assert_eq!(<Succ<U5> as Nat>::reify(), 6);
        assert_eq!(<Succ<U8> as Nat>::reify(), 9);
        assert_eq!(<Succ<U3> as Nat>::reify(), 4);
    }

    #[test]
    fn ternary_pred() {
        assert_eq!(<Pred<U7> as Nat>::reify(), 6);
        assert_eq!(<Pred<U2> as Nat>::reify(), 1);
        assert_eq!(<Pred<U9> as Nat>::reify(), 8);
        assert_eq!(<Pred<U5> as Nat>::reify(), 4);
        assert_eq!(<Pred<U8> as Nat>::reify(), 7);
        assert_eq!(<Pred<U3> as Nat>::reify(), 2);
    }

    #[test]
    fn ternary_double_pred() {
        assert_eq!(<DoublePred<U7> as Nat>::reify(), 5);
        assert_eq!(<DoublePred<U2> as Nat>::reify(), 0);
        assert_eq!(<DoublePred<U9> as Nat>::reify(), 7);
        assert_eq!(<DoublePred<U5> as Nat>::reify(), 3);
        assert_eq!(<DoublePred<U8> as Nat>::reify(), 6);
        assert_eq!(<DoublePred<U3> as Nat>::reify(), 1);
    }

    #[test]
    fn ternary_add() {
        assert_eq!(<Add<U0, U10> as Nat>::reify(), 10);
        assert_eq!(<Add<U2, U11> as Nat>::reify(), 13);
        assert_eq!(<Add<U25, U12> as Nat>::reify(), 37);
        assert_eq!(<Add<U10, U18> as Nat>::reify(), 28);
        assert_eq!(<Add<U15, U2> as Nat>::reify(), 17);
        assert_eq!(<Add<U24, U5> as Nat>::reify(), 29);
        assert_eq!(<Add<U13, U1> as Nat>::reify(), 14);
        assert_eq!(<Add<U6, U23> as Nat>::reify(), 29);
        assert_eq!(<Add<U2, U17> as Nat>::reify(), 19);
        assert_eq!(<Add<U26, U25> as Nat>::reify(), 51);
    }

    #[test]
    fn ternary_sub() {
        assert_eq!(<Sub<U25, U12> as Nat>::reify(), 13);
        assert_eq!(<Sub<U15, U2> as Nat>::reify(), 13);
        assert_eq!(<Sub<U24, U5> as Nat>::reify(), 19);
        assert_eq!(<Sub<U13, U1> as Nat>::reify(), 12);
        assert_eq!(<Sub<U26, U25> as Nat>::reify(), 1);
    }

    #[test]
    #[should_panic]
    fn ternary_sub_undefined() {
        let _: Undefined = <Sub<U1, U9>>::default();
        let _: Undefined = <Sub<Sub<U1, U9>, U9>>::default();
        let _ = <Sub<Sub<U1, U9>, U9> as Nat>::reify();
    }

    #[test]
    fn ternary_undef_or_2() {
        assert_eq!(<UndefOr2<Sub<Sub<U1, U9>, U9>, U2, Nat2<U1, U0>> as NatPair>::reify(), (1, 0));
        assert_eq!(<UndefOr2<Sub<Sub<U2, U1>, U1>, U2, Nat2<U0, U1>> as NatPair>::reify(), (0, 2));
    }

    #[test]
    fn ternary_mul() {
        assert_eq!(<Mul<U0, U0> as Nat>::reify(), 0);
        assert_eq!(<Mul<U2, U2> as Nat>::reify(), 4);
        assert_eq!(<Mul<U7, U9> as Nat>::reify(), 63);
        assert_eq!(<Mul<U2, U1> as Nat>::reify(), 2);
        assert_eq!(<Mul<U9, U5> as Nat>::reify(), 45);
        assert_eq!(<Mul<U5, U8> as Nat>::reify(), 40);
        assert_eq!(<Mul<U8, U2> as Nat>::reify(), 16);
        assert_eq!(<Mul<U3, U2> as Nat>::reify(), 6);
    }

    #[test]
    fn ternary_div() {
        assert_eq!(<Div<U0, U1> as Nat>::reify(), 0);
        assert_eq!(<Div<U2, U2> as Nat>::reify(), 1);
        assert_eq!(<Div<U8, U2> as Nat>::reify(), 4);
        assert_eq!(<Div<U7, U9> as Nat>::reify(), 0);
        assert_eq!(<Div<U2, U1> as Nat>::reify(), 2);
        assert_eq!(<Div<U9, U5> as Nat>::reify(), 1);
        assert_eq!(<Div<U5, U8> as Nat>::reify(), 0);
        assert_eq!(<Div<U3, U2> as Nat>::reify(), 1);
    }

    #[test]
    fn ternary_rem() {
        assert_eq!(<Rem<U0, U1> as Nat>::reify(), 0);
        assert_eq!(<Rem<U2, U2> as Nat>::reify(), 0);
        assert_eq!(<Rem<U8, U2> as Nat>::reify(), 0);
        assert_eq!(<Rem<U7, U9> as Nat>::reify(), 7);
        assert_eq!(<Rem<U2, U1> as Nat>::reify(), 0);
        assert_eq!(<Rem<U9, U5> as Nat>::reify(), 4);
        assert_eq!(<Rem<U5, U8> as Nat>::reify(), 5);
        assert_eq!(<Rem<U3, U2> as Nat>::reify(), 1);
    }
}
