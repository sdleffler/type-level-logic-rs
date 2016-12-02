//! Type-level boolean operations.

pub use types::boolean::*;

type_operators! {
    [A, B, C, D, E]

    /// Boolean `And`. Use as `And<X, Y>` or `<X as BoolAnd<Y>>::Output`.
    (And) BoolAnd(Bool, Bool): Bool {
        [False, False] => False
        [False, True] => False
        [True, False] => False
        [True, True] => True
    }

    /// Boolean `Or`. Use as `Or<X, Y>` or `<X as BoolOr<Y>>::Output`.
    (Or) BoolOr(Bool, Bool): Bool {
        [False, False] => False
        [False, True] => True
        [True, False] => True
        [True, True] => True
    }

    /// Boolean `IfThen`. Use as `IfThen<X, Y>` or `<X as BoolIfThen<Y>>::Output`. As a logical
    /// proposition, this is equivalent to "**if** `X`, **then** `Y`".
    (IfThen) BoolIfThen(Bool, Bool): Bool {
        [False, False] => True
        [False, True] => True
        [True, False] => False
        [True, True] => True
    }

    /// Boolean `OnlyIf`. Use as `OnlyIf<X, Y>` or `<X as BoolOnlyIf<Y>>::Output`. As a logical
    /// proposition, this is equivalent to "`Y` **only if** `X`".
    (OnlyIf) BoolOnlyIf(Bool, Bool): Bool {
        [False, False] => True
        [False, True] => False
        [True, False] => True
        [True, True] => True
    }

    /// Boolean `Xor`. Use as `Xor<X, Y>` or `<X as BoolXor<Y>>::Output`.
    (Xor) BoolXor(Bool, Bool): Bool {
        [False, False] => False
        [False, True] => True
        [True, False] => True
        [True, True] => False
    }
}
