use types::boolean::{Bool, True, False};

type_operators! {
    [A, B, C, D, E]

    (And) BoolAnd(Bool, Bool): Bool {
        [False, False] => False
        [False, True] => False
        [True, False] => False
        [True, True] => True
    }

    (Or) BoolOr(Bool, Bool): Bool {
        [False, False] => False
        [False, True] => True
        [True, False] => True
        [True, True] => True
    }

    (IfThen) BoolIfThen(Bool, Bool): Bool {
        [False, False] => True
        [False, True] => True
        [True, False] => False
        [True, True] => True
    }

    (OnlyIf) BoolOnlyIf(Bool, Bool): Bool {
        [False, False] => True
        [False, True] => False
        [True, False] => True
        [True, True] => True
    }

    (Xor) BoolXor(Bool, Bool): Bool {
        [False, False] => False
        [False, True] => True
        [True, False] => True
        [True, True] => False
    }
}
