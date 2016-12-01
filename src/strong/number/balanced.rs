use types::number::balanced::{IntPair, Int2, Int2P1, Int2P2, Int, Undefined, Term, Zero, Plus,
                              Minus};

type_operators! {
    [A, B, C, D, E, F, G, H]

    /// Project the first value in an `IntPair`.
    (Int2First) Int2P1(IntPair): Int {
        forall (A: Int, B: Int) {
            [(Int2 A B)] => A
        }
    }

    /// Project the second value in an `IntPair`.
    (Int2Second) Int2P2(IntPair): Int {
        forall (A: Int, B: Int) {
            [(Int2 A B)] => B
        }
    }

    /// The `Succ` operator adds one to an integer.
    (Succ) IntSucc(Int): Int {
        [Undefined] => Undefined
        [Term] => Plus
        forall (N: Int) {
            [(Zero N)] => (Plus N)
            [(Plus N)] => (Minus (# N))
            [(Minus N)] => (@Unique (Zero N))
        }
    }

    /// The `Pred` operator subtracts one to an integer.
    (Pred) IntPred(Int): Int {
        [Undefined] => Undefined
        [Term] => Minus
        forall (N: Int) {
            [(Zero N)] => (Minus N)
            [(Plus N)] => (@Unique (Zero N))
            [(Minus N)] => (Plus (# N))
        }
    }

    /// The `Triple` operator triples an integer, and avoids redundant zeroes (and thus preserves
    /// unique representations.) It is equivalent to `Zero<N>` unless `N` is `Term`, in which case
    /// `Triple<Term> = Term`. Its collapsing acts at only one level, which should be sufficient
    /// unless multiple levels of redundant zeroes are introduced by user error.
    (Triple) IntTriple(Int): Int {
        [Undefined] => Undefined
        [Term] => Term
        forall (N: Int) {
            [(Zero N)] => (Zero (Zero N))
            [(Plus N)] => (Zero (Plus N))
            [(Minus N)] => (Zero (Minus N))
        }
    }

    /// The `Unique` operator collapses one level of redundant zeroes in a balanced ternary
    /// representation. Using `Unique` every time we might get a redundant zero *should* get rid of
    /// any problems with non-unique representations of zero.
    (IntUnique) Unique(Int): Int {
        [Undefined] => Undefined
        [Term] => Term
        [(Zero Term)] => Term
        forall (N: Int) {
            [(Zero (Zero N))] => (Zero (# (Zero N)))
            [(Zero (Plus N))] => (Zero (Plus (# N)))
            [(Zero (Minus N))] => (Zero (Minus (# N)))
            [(Plus N)] => (Plus N)
            [(Minus N)] => (Minus N)
        }
    }

    /// Integer negation. In balanced ternary, this is just taking every "minus" and "plus" and
    /// switching them. Used as `Neg<X>` or `<X as IntNeg>::Output`.
    ///
    /// Here, we also combine it with the functionality of the `Unique` operator such that they
    /// don't need to be nested (for example in our implementation of subtraction.)
    (Neg) IntNeg(Int): Int {
        [Undefined] => Undefined
        [Term] => Term
        [(Zero Term)] => Term
        forall (N: Int) {
            [(Zero (Zero N))] => (Zero (# (Zero N)))
            [(Zero (Plus N))] => (Zero (Minus (# N)))
            [(Zero (Minus N))] => (Zero (Plus (# N)))
            [(Plus N)] => (Minus (# N))
            [(Minus N)] => (Plus (# N))
        }
    }

    /// Integer addition. Used as `Add<X, Y>` or `<X as IntAdd<Y>>::Output`.
    (Add) IntAdd(Int, Int): Int {
        [Term, Term] => Term
        [Undefined, Term] => Undefined
        [Term, Undefined] => Undefined
        [Undefined, Undefined] => Undefined
        forall (N: Int) {
            [Term, (Zero N)] => (@Unique (Zero N))
            [Term, (Plus N)] => (Plus N)
            [Term, (Minus N)] => (Minus N)
            [(Zero N), Term] => (@Unique (Zero N))
            [(Plus N), Term] => (Plus N)
            [(Minus N), Term] => (Minus N)
            [Undefined, (Zero N)] => Undefined
            [Undefined, (Plus N)] => Undefined
            [Undefined, (Minus N)] => Undefined
            [(Zero N), Undefined] => Undefined
            [(Plus N), Undefined] => Undefined
            [(Minus N), Undefined] => Undefined
        }
        forall (M: Int, N: Int) {
            [(Zero M), (Zero N)] => (@Unique (Zero (# M N)))
            [(Zero M), (Plus N)] => (Plus (# M N))
            [(Zero M), (Minus N)] => (Minus (# M N))
            [(Plus M), (Zero N)] => (Plus (# M N))
            [(Plus M), (Plus N)] => (Minus (@IntSucc (# M N)))
            [(Plus M), (Minus N)] => (@Unique (Zero (# M N)))
            [(Minus M), (Zero N)] => (Minus (# M N))
            [(Minus M), (Plus N)] => (@Unique (Zero (# M N)))
            [(Minus M), (Minus N)] => (Plus (# (# M N) Minus))
        }
    }

    /// Integer subtraction. Used as `Sub<X, Y>` or `<X as IntSub<Y>>::Output`.
    (Sub) IntSub(Int, Int): Int {
        [Term, Term] => Term
        [Undefined, Term] => Undefined
        [Term, Undefined] => Undefined
        [Undefined, Undefined] => Undefined
        forall (N: Int) {
            [Term, (Zero N)] => (@IntNeg (Zero N))
            [Term, (Plus N)] => (Minus (@IntNeg N))
            [Term, (Minus N)] => (Plus (@IntNeg N))
            [(Zero N), Term] => (@Unique (Zero N))
            [(Plus N), Term] => (Plus N)
            [(Minus N), Term] => (Minus N)
            [Undefined, (Zero N)] => Undefined
            [Undefined, (Plus N)] => Undefined
            [Undefined, (Minus N)] => Undefined
            [(Zero N), Undefined] => Undefined
            [(Plus N), Undefined] => Undefined
            [(Minus N), Undefined] => Undefined
        }
        forall (M: Int, N: Int) {
            [(Zero M), (Zero N)] => (@Unique (Zero (# M N)))
            [(Zero M), (Plus N)] => (Minus (# M N))
            [(Zero M), (Minus N)] => (Plus (# M N))
            [(Plus M), (Zero N)] => (Plus (# M N))
            [(Plus M), (Plus N)] => (@Unique (Zero (# M N)))
            [(Plus M), (Minus N)] => (Minus (# (# M N) Minus))
            [(Minus M), (Zero N)] => (Minus (# M N))
            [(Minus M), (Plus N)] => (Plus (# (# M N) Plus))
            [(Minus M), (Minus N)] => (@Unique (Zero (# M N)))
        }
    }

    /// Integer multiplication. Used as `Mul<X, Y>` or `<X as IntMul<Y>>::Output`.
    (Mul) IntMul(Int, Int): Int {
        [Term, Term] => Term
        [Undefined, Term] => Undefined
        [Term, Undefined] => Undefined
        [Undefined, Undefined] => Undefined
        forall (N: Int) {
            [Term, (Zero N)] => Term
            [Term, (Plus N)] => Term
            [Term, (Minus N)] => Term
            [(Zero N), Term] => Term
            [(Plus N), Term] => Term
            [(Minus N), Term] => Term
            [Undefined, (Zero N)] => Undefined
            [Undefined, (Plus N)] => Undefined
            [Undefined, (Minus N)] => Undefined
            [(Zero N), Undefined] => Undefined
            [(Plus N), Undefined] => Undefined
            [(Minus N), Undefined] => Undefined
        }
        forall (M: Int, N: Int) {
            [(Zero M), (Zero N)] => (# M (Zero (Zero N))) // m0 * n0 => 3 * (m * n0) + 0
            [(Zero M), (Plus N)] => (# M (Zero (Plus M))) // m0 * n1 => 3 * (m * n1) + 0
            [(Zero M), (Minus N)] => (# M (Zero (Minus N))) // m0 * nT => 3 * (m * nT) + 0
            [(Plus M), (Zero N)] => (@IntAdd (# M (Zero (Zero N))) (Zero N)) // m1 * n0 => 3 * (m * n0) + 0
            [(Plus M), (Plus N)] => (@IntAdd (# M (Zero (Plus N))) (Plus N)) // m1 * n1 => 3 * (m * n1) + n1
            [(Plus M), (Minus N)] => (@IntAdd (# M (Zero (Minus N))) (Minus N)) // m1 * nT => 3 * (m * nT) + nT
            [(Minus M), (Zero N)] => (@IntSub (# M (Zero (Zero N))) (Zero N)) // mT * n0 => 3 * (m * n0) + 0
            [(Minus M), (Plus N)] => (@IntSub (# M (Zero (Plus N))) (Plus N)) // mT * n1 => 3 * (m * n1) - n1
            [(Minus M), (Minus N)] => (@IntSub (# M (Zero (Minus N))) (Minus N)) // mT * nT => 3 * (m * nT) - nT
        }
    }

    /// Integer comparison. Takes five integers - the first two are to be compared. If the result is
    /// that the first integer is greater than the second, the fifth integer is returned; if the
    /// result is that the two integers are equal, the fourth integer is returned; and if the result
    /// is that the first integer is less than the second, the third integer is returned.
    (Cmp) IntCmp(Int, Int, Int, Int, Int): Int {
        forall (L: Int, E: Int, G: Int) {
            [Term, Term, L, E, G] => E
        }
        forall (N: Int, L: Int, E: Int, G: Int) {
            [Term, (Zero N), L, E, G] => (# Term N L E G)
            [Term, (Plus N), L, E, G] => (# Term N L L G)
            [Term, (Minus N), L, E, G] => (# Term N L G G)
            [(Zero N), Term, L, E, G] => (# N Term L E G)
            [(Plus N), Term, L, E, G] => (# N Term L G G)
            [(Minus N), Term, L, E, G] => (# N Term L L G)
        }
        forall (M: Int, N: Int, L: Int, E: Int, G: Int) {
            [(Zero M), (Zero N), L, E, G] => (# M N L E G)
            [(Zero M), (Plus N), L, E, G] => (# M N L L G)
            [(Zero M), (Minus N), L, E, G] => (# M N L G G)
            [(Plus M), (Zero N), L, E, G] => (# M N L G G)
            [(Plus M), (Plus N), L, E, G] => (# M N L E G)
            [(Plus M), (Minus N), L, E, G] => (# M N L G G)
            [(Minus M), (Zero N), L, E, G] => (# M N L L G)
            [(Minus M), (Plus N), L, E, G] => (# M N L L G)
            [(Minus M), (Minus N), L, E, G] => (# M N L E G)
        }
    }

    /// Two-way integer comparison, but with the output trait bounds as an
    /// `IntPair` instead of an `Int`.
    (Cmp2) IntCmp2(Int, Int, IntPair, IntPair, IntPair): IntPair {
        forall (L: IntPair, E: IntPair, G: IntPair) {
            [Term, Term, L, E, G] => E
        }
        forall (N: Int, L: IntPair, E: IntPair, G: IntPair) {
            [Term, (Zero N), L, E, G] => (# Term N L E G)
            [Term, (Plus N), L, E, G] => (# Term N L L G)
            [Term, (Minus N), L, E, G] => (# Term N L G G)
            [(Zero N), Term, L, E, G] => (# N Term L E G)
            [(Plus N), Term, L, E, G] => (# N Term L G G)
            [(Minus N), Term, L, E, G] => (# N Term L L G)
        }
        forall (M: Int, N: Int, L: IntPair, E: IntPair, G: IntPair) {
            [(Zero M), (Zero N), L, E, G] => (# M N L E G)
            [(Zero M), (Plus N), L, E, G] => (# M N L L G)
            [(Zero M), (Minus N), L, E, G] => (# M N L G G)
            [(Plus M), (Zero N), L, E, G] => (# M N L G G)
            [(Plus M), (Plus N), L, E, G] => (# M N L E G)
            [(Plus M), (Minus N), L, E, G] => (# M N L G G)
            [(Minus M), (Zero N), L, E, G] => (# M N L L G)
            [(Minus M), (Plus N), L, E, G] => (# M N L L G)
            [(Minus M), (Minus N), L, E, G] => (# M N L E G)
        }
    }

    (LteCmp) IntLteCmp(Int, Int, Int, Int): Int {
        forall (X: Int, Y: Int, LE: Int, G: Int) {
            [X, Y, LE, G] => (@IntCmp X Y LE LE G)
        }
    }

    (LteCmp2) IntLteCmp2(Int, Int, IntPair, IntPair): IntPair {
        forall (X: Int, Y: Int, LE: IntPair, G: IntPair) {
            [X, Y, LE, G] => (@IntCmp2 X Y LE LE G)
        }
    }

    /// Integer absolute value. Used as `Abs<X>` or `<X as IntAbs>::Output`.
    (Abs) IntAbs(Int): Int {
        [Term] => Term
        [Undefined] => Undefined
        forall (X: Int) {
            [(Zero X)] => (@IntLteCmp Term (Zero X) (Zero X) (@IntNeg (Zero X)))
            [(Plus X)] => (@IntLteCmp Term (Plus X) (Plus X) (@IntNeg (Plus X)))
            [(Minus X)] => (@IntLteCmp Term (Minus X) (Minus X) (@IntNeg (Minus X)))
        }
    }

    /// Three-way integer absolute-value minimum comparison. Takes three integers, and returns a pair,
    /// the corresponding argument paired with the integer whose absolute value was smallest.
    (AbsMinThreeCmp) IntAbsMinThreeCmp(Int, Int, Int, Int, Int, Int): IntPair {
        forall (X: Int, Y: Int, Z: Int, A: Int, B: Int, C: Int) {
            [X, Y, Z, A, B, C] => (@IntLteCmp2 (@IntAbs Y) (@IntAbs Z)
                (@IntLteCmp2 (@IntAbs X) (@IntAbs Y) (Int2 X A) (Int2 Y B))
                (@IntLteCmp2 (@IntAbs X) (@IntAbs Z) (Int2 X A) (Int2 Z C)))
        }
    }

    /// Integer *reversal.* This is probably something you *never* want to do! However, it's useful
    /// in implementing division, in which we wish to access trits with the most-significant trits
    /// first. This reverses all the trits of the integer.
    (Rev) IntRev(Int): Int {
        forall (N: Int) {
            [N] => (@IntRevInternal N Term)
        }
    }

    (RevInternal) IntRevInternal(Int, Int): Int {
        forall (N: Int) {
            [Term, N] => N
        }
        forall (M: Int, N: Int) {
            [(Zero M), N] => (# M (Zero N))
            [(Plus M), N] => (# M (Plus N))
            [(Minus M), N] => (# M (Minus N))
        }
    }

    /// Integer division. Used as `Div<X, Y>` or `<X as IntDiv<Y>>::Output`.
    (Div) IntDiv(Int, Int): Int {
        [Term, Term] => Undefined
        [Term, Undefined] => Undefined
        [Undefined, Term] => Undefined
        [Undefined, Undefined] => Undefined
        forall (D: Int) {
            [Term, (Zero D)] => Term
            [Term, (Plus D)] => Term
            [Term, (Minus D)] => Term
        }
        forall (N: Int, D: Int) {
            [(Zero N), (Zero D)] => (@Int2P2 (@IntDivInternal (@IntRev (Zero N)) (Zero D) (Int2 Term Term)))
            [(Zero N), (Plus D)] => (@Int2P2 (@IntDivInternal (@IntRev (Zero N)) (Plus D) (Int2 Term Term)))
            [(Zero N), (Minus D)] => (@Int2P2 (@IntDivInternal (@IntRev (Zero N)) (Minus D) (Int2 Term Term)))
            [(Plus N), (Zero D)] => (@Int2P2 (@IntDivInternal (@IntRev (Plus N)) (Zero D) (Int2 Term Term)))
            [(Plus N), (Plus D)] => (@Int2P2 (@IntDivInternal (@IntRev (Plus N)) (Plus D) (Int2 Term Term)))
            [(Plus N), (Minus D)] => (@Int2P2 (@IntDivInternal (@IntRev (Plus N)) (Minus D) (Int2 Term Term)))
            [(Minus N), (Zero D)] => (@Int2P2 (@IntDivInternal (@IntRev (Minus N)) (Zero D) (Int2 Term Term)))
            [(Minus N), (Plus D)] => (@Int2P2 (@IntDivInternal (@IntRev (Minus N)) (Plus D) (Int2 Term Term)))
            [(Minus N), (Minus D)] => (@Int2P2 (@IntDivInternal (@IntRev (Minus N)) (Minus D) (Int2 Term Term)))
        }
    }

    /// Integer remainder. Used as `Rem<X, Y>` or `<X as IntRem<Y>>::Output`.
    (Rem) IntRem(Int, Int): Int {
        [Term, Term] => Undefined
        [Term, Undefined] => Undefined
        [Undefined, Term] => Undefined
        [Undefined, Undefined] => Undefined
        forall (D: Int) {
            [Term, (Zero D)] => Term
            [Term, (Plus D)] => Term
            [Term, (Minus D)] => Term
        }
        forall (N: Int, D: Int) {
            [(Zero N), (Zero D)] => (@Int2P1 (@IntDivInternal (@IntRev (Zero N)) (Zero D) (Int2 Term Term)))
            [(Zero N), (Plus D)] => (@Int2P1 (@IntDivInternal (@IntRev (Zero N)) (Plus D) (Int2 Term Term)))
            [(Zero N), (Minus D)] => (@Int2P1 (@IntDivInternal (@IntRev (Zero N)) (Minus D) (Int2 Term Term)))
            [(Plus N), (Zero D)] => (@Int2P1 (@IntDivInternal (@IntRev (Plus N)) (Zero D) (Int2 Term Term)))
            [(Plus N), (Plus D)] => (@Int2P1 (@IntDivInternal (@IntRev (Plus N)) (Plus D) (Int2 Term Term)))
            [(Plus N), (Minus D)] => (@Int2P1 (@IntDivInternal (@IntRev (Plus N)) (Minus D) (Int2 Term Term)))
            [(Minus N), (Zero D)] => (@Int2P1 (@IntDivInternal (@IntRev (Minus N)) (Zero D) (Int2 Term Term)))
            [(Minus N), (Plus D)] => (@Int2P1 (@IntDivInternal (@IntRev (Minus N)) (Plus D) (Int2 Term Term)))
            [(Minus N), (Minus D)] => (@Int2P1 (@IntDivInternal (@IntRev (Minus N)) (Minus D) (Int2 Term Term)))
        }
    }

    (DivInternal) IntDivInternal(Int, Int, IntPair): IntPair {
        forall (D: Int, R: Int, Q: Int) {
            [Term, D, (Int2 R Q)] => (Int2 R Q)
        }
        forall (N: Int, D: Int, R: Int, Q: Int) {
            [(Zero N), D, (Int2 R Q)] => (# N D
                (@IntAbsMinThreeCmp
                    (@IntTriple R)
                    (@IntSub (@IntTriple R) D)
                    (@IntAdd (@IntTriple R) D)
                        (@IntTriple Q) (Plus Q) (Minus Q)))
            [(Plus N), D, (Int2 R Q)] => (# N D
                (@IntAbsMinThreeCmp
                    (Plus R)
                    (@IntSub (Plus R) D)
                    (@IntAdd (Plus R) D)
                        (@IntTriple Q) (Plus Q) (Minus Q)))
            [(Minus N), D, (Int2 R Q)] => (# N D
                (@IntAbsMinThreeCmp
                    (Minus R)
                    (@IntSub (Minus R) D)
                    (@IntAdd (Minus R) D)
                        (@IntTriple Q) (Plus Q) (Minus Q)))
        }
    }
}


#[allow(non_snake_case)]
#[cfg(test)]
mod tests {
    use super::*;
    use types::number::balanced::*;

    #[test]
    fn balanced_ternary_cmp() {
        assert_eq!(<Cmp<S0, S0, SN1, S0, SP1> as Int>::reify(), 0);
        assert_eq!(<Cmp<SN7, SP9, SN1, S0, SP1> as Int>::reify(), -1);
        assert_eq!(<Cmp<SP2, SP2, SN1, S0, SP1> as Int>::reify(), 0);
        assert_eq!(<Cmp<SN2, SN1, SN1, S0, SP1> as Int>::reify(), -1);
        assert_eq!(<Cmp<SN9, SP5, SN1, S0, SP1> as Int>::reify(), -1);
        assert_eq!(<Cmp<SP5, SN8, SN1, S0, SP1> as Int>::reify(), 1);
        assert_eq!(<Cmp<SN8, SP2, SN1, S0, SP1> as Int>::reify(), -1);
        assert_eq!(<Cmp<SP3, SP2, SN1, S0, SP1> as Int>::reify(), 1);
        assert_eq!(<Cmp<S0, SN5, SN1, S0, SP1> as Int>::reify(), 1);
        assert_eq!(<Cmp<S0, SN4, SN1, S0, SP1> as Int>::reify(), 1);
        assert_eq!(<Cmp<S0, SN3, SN1, S0, SP1> as Int>::reify(), 1);
        assert_eq!(<Cmp<S0, SN2, SN1, S0, SP1> as Int>::reify(), 1);
        assert_eq!(<Cmp<S0, SN1, SN1, S0, SP1> as Int>::reify(), 1);
        assert_eq!(<Cmp<S0, S0, SN1, S0, SP1> as Int>::reify(), 0);
        assert_eq!(<Cmp<S0, SP1, SN1, S0, SP1> as Int>::reify(), -1);
        assert_eq!(<Cmp<S0, SP2, SN1, S0, SP1> as Int>::reify(), -1);
        assert_eq!(<Cmp<S0, SP3, SN1, S0, SP1> as Int>::reify(), -1);
        assert_eq!(<Cmp<S0, SP4, SN1, S0, SP1> as Int>::reify(), -1);
        assert_eq!(<Cmp<S0, SP5, SN1, S0, SP1> as Int>::reify(), -1);
    }

    #[test]
    fn balanced_ternary_lte_cmp() {
        assert_eq!(<LteCmp<SN7, SP9, S0, SP1> as Int>::reify(), 0);
        assert_eq!(<LteCmp<SP2, SP2, S0, SP1> as Int>::reify(), 0);
        assert_eq!(<LteCmp<SN2, SN1, S0, SP1> as Int>::reify(), 0);
        assert_eq!(<LteCmp<SN9, SP5, S0, SP1> as Int>::reify(), 0);
        assert_eq!(<LteCmp<SP5, SN8, S0, SP1> as Int>::reify(), 1);
        assert_eq!(<LteCmp<SN8, SP2, S0, SP1> as Int>::reify(), 0);
        assert_eq!(<LteCmp<SP3, SP2, S0, SP1> as Int>::reify(), 1);
        assert_eq!(<LteCmp<S0, SN5, S0, SP1> as Int>::reify(), 1);
        assert_eq!(<LteCmp<S0, SN4, S0, SP1> as Int>::reify(), 1);
        assert_eq!(<LteCmp<S0, SN3, S0, SP1> as Int>::reify(), 1);
        assert_eq!(<LteCmp<S0, SN2, S0, SP1> as Int>::reify(), 1);
        assert_eq!(<LteCmp<S0, SN1, S0, SP1> as Int>::reify(), 1);
        assert_eq!(<LteCmp<S0, S0, S0, SP1> as Int>::reify(), 0);
        assert_eq!(<LteCmp<S0, SP1, S0, SP1> as Int>::reify(), 0);
        assert_eq!(<LteCmp<S0, SP2, S0, SP1> as Int>::reify(), 0);
        assert_eq!(<LteCmp<S0, SP3, S0, SP1> as Int>::reify(), 0);
        assert_eq!(<LteCmp<S0, SP4, S0, SP1> as Int>::reify(), 0);
        assert_eq!(<LteCmp<S0, SP5, S0, SP1> as Int>::reify(), 0);
        assert_eq!(<LteCmp<S0, SP2, SP2, SN2> as Int>::reify(), 2);
    }

    #[test]
    fn balanced_ternary_abs_min_three_cmp() {
        assert_eq!(<AbsMinThreeCmp<SP5, SN7, SN8, S0, SP1, SP2> as IntPair>::reify(), (5, 0));
        assert_eq!(<AbsMinThreeCmp<SP9, SN4, SN1, S0, SP1, SP2> as IntPair>::reify(), (-1, 2));
        assert_eq!(<AbsMinThreeCmp<SN4, SN1, SN5, S0, SP1, SP2> as IntPair>::reify(), (-1, 1));
        assert_eq!(<AbsMinThreeCmp<SP2, S0, SP5, S0, SP1, SP2> as IntPair>::reify(), (0, 1));
        assert_eq!(<AbsMinThreeCmp<SN6, SP8, SP7, S0, SP1, SP2> as IntPair>::reify(), (-6, 0));
        assert_eq!(<AbsMinThreeCmp<SN7, SN3, SP6, S0, SP1, SP2> as IntPair>::reify(), (-3, 1));
        assert_eq!(<AbsMinThreeCmp<SP2, SN8, SP3, S0, SP1, SP2> as IntPair>::reify(), (2, 0));
        assert_eq!(<AbsMinThreeCmp<S0, SP9, SN2, S0, SP1, SP2> as IntPair>::reify(), (0, 0));
        assert_eq!(<AbsMinThreeCmp<SP7, SN2, SP1, S0, SP1, SP2> as IntPair>::reify(), (1, 2));
    }

    #[test]
    fn balanced_ternary_succ() {
        assert_eq!(<Succ<S0> as Int>::reify(), 1);
        assert_eq!(<Succ<SN7> as Int>::reify(), -6);
        assert_eq!(<Succ<SP2> as Int>::reify(), 3);
        assert_eq!(<Succ<SN2> as Int>::reify(), -1);
        assert_eq!(<Succ<SN9> as Int>::reify(), -8);
        assert_eq!(<Succ<SP5> as Int>::reify(), 6);
        assert_eq!(<Succ<SN8> as Int>::reify(), -7);
        assert_eq!(<Succ<SP3> as Int>::reify(), 4);
    }

    #[test]
    fn balanced_ternary_pred() {
        assert_eq!(<Pred<S0> as Int>::reify(), -1);
        assert_eq!(<Pred<SN7> as Int>::reify(), -8);
        assert_eq!(<Pred<SP2> as Int>::reify(), 1);
        assert_eq!(<Pred<SN2> as Int>::reify(), -3);
        assert_eq!(<Pred<SN9> as Int>::reify(), -10);
        assert_eq!(<Pred<SP5> as Int>::reify(), 4);
        assert_eq!(<Pred<SN8> as Int>::reify(), -9);
        assert_eq!(<Pred<SP3> as Int>::reify(), 2);
    }

    #[test]
    fn balanced_ternary_abs() {
        assert_eq!(<Abs<S0> as Int>::reify(), 0);
        assert_eq!(<Abs<SN7> as Int>::reify(), 7);
        assert_eq!(<Abs<SP2> as Int>::reify(), 2);
        assert_eq!(<Abs<SN2> as Int>::reify(), 2);
        assert_eq!(<Abs<SN9> as Int>::reify(), 9);
        assert_eq!(<Abs<SP5> as Int>::reify(), 5);
        assert_eq!(<Abs<SN8> as Int>::reify(), 8);
        assert_eq!(<Abs<SP3> as Int>::reify(), 3);
    }

    #[test]
    fn balanced_ternary_add() {
        assert_eq!(<Add<S0, S0> as Int>::reify(), 0);
        assert_eq!(<Add<SN7, SP9> as Int>::reify(), 2);
        assert_eq!(<Add<SP2, SP2> as Int>::reify(), 4);
        assert_eq!(<Add<SN2, SN1> as Int>::reify(), -3);
        assert_eq!(<Add<SN9, SP5> as Int>::reify(), -4);
        assert_eq!(<Add<SP5, SN8> as Int>::reify(), -3);
        assert_eq!(<Add<SN8, SP2> as Int>::reify(), -6);
        assert_eq!(<Add<SP3, SP2> as Int>::reify(), 5);
    }

    #[test]
    fn balanced_ternary_sub() {
        assert_eq!(<Sub<S0, S0> as Int>::reify(), 0);
        assert_eq!(<Sub<SP7, SP9> as Int>::reify(), -2);
        assert_eq!(<Sub<SP2, SP2> as Int>::reify(), 0);
        assert_eq!(<Sub<SN2, SN1> as Int>::reify(), -1);
        assert_eq!(<Sub<SP9, SP5> as Int>::reify(), 4);
        assert_eq!(<Sub<SP5, SN8> as Int>::reify(), 13);
        assert_eq!(<Sub<SN8, SP2> as Int>::reify(), -10);
        assert_eq!(<Sub<SP3, SP2> as Int>::reify(), 1);
    }

    #[test]
    fn balanced_ternary_mul() {
        assert_eq!(<Mul<S0, S0> as Int>::reify(), 0);
        assert_eq!(<Mul<SP2, SP2> as Int>::reify(), 4);
        assert_eq!(<Mul<SN7, SP9> as Int>::reify(), -63);
        assert_eq!(<Mul<SN2, SN1> as Int>::reify(), 2);
        assert_eq!(<Mul<SN9, SP5> as Int>::reify(), -45);
        assert_eq!(<Mul<SP5, SN8> as Int>::reify(), -40);
        assert_eq!(<Mul<SN8, SP2> as Int>::reify(), -16);
        assert_eq!(<Mul<SP3, SP2> as Int>::reify(), 6);
    }

    #[test]
    fn balanced_ternary_div() {
        assert_eq!(<Div<S0, SP1> as Int>::reify(), 0);
        assert_eq!(<Div<SP2, SP2> as Int>::reify(), 1);
        assert_eq!(<Div<SN8, SP2> as Int>::reify(), -4);
        assert_eq!(<Div<SN7, SP9> as Int>::reify(), -1);
        assert_eq!(<Div<SN2, SN1> as Int>::reify(), 2);
        assert_eq!(<Div<SN9, SP5> as Int>::reify(), -2);
        assert_eq!(<Div<SP5, SN8> as Int>::reify(), -1);
        assert_eq!(<Div<SP3, SP2> as Int>::reify(), 1);
    }

    #[test]
    fn balanced_ternary_rem() {
        assert_eq!(<Rem<S0, SP1> as Int>::reify(), 0);
        assert_eq!(<Rem<SP2, SP2> as Int>::reify(), 0);
        assert_eq!(<Rem<SN8, SP2> as Int>::reify(), 0);
        assert_eq!(<Rem<SN7, SP9> as Int>::reify(), 2);
        assert_eq!(<Rem<SN2, SN1> as Int>::reify(), 0);
        assert_eq!(<Rem<SN9, SP5> as Int>::reify(), 1);
        assert_eq!(<Rem<SP5, SN8> as Int>::reify(), -3);
        assert_eq!(<Rem<SP3, SP2> as Int>::reify(), 1);
    }

    #[test]
    fn balanced_ternary_rev() {
        assert_eq!(<Rev<SN9> as Int>::reify(), -1);
        assert_eq!(<Rev<SN8> as Int>::reify(), 8);
        assert_eq!(<Rev<SN7> as Int>::reify(), -7);
        assert_eq!(<Rev<SN6> as Int>::reify(), 2);
        assert_eq!(<Rev<SN5> as Int>::reify(), 11);
        assert_eq!(<Rev<SN4> as Int>::reify(), -4);
        assert_eq!(<Rev<SN3> as Int>::reify(), -1);
        assert_eq!(<Rev<SN2> as Int>::reify(), 2);
        assert_eq!(<Rev<SN1> as Int>::reify(), -1);
        assert_eq!(<Rev<S0> as Int>::reify(), 0);
        assert_eq!(<Rev<SP1> as Int>::reify(), 1);
        assert_eq!(<Rev<SP2> as Int>::reify(), -2);
        assert_eq!(<Rev<SP3> as Int>::reify(), 1);
    }
}
