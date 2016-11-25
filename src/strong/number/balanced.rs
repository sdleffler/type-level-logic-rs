use types::number::balanced::{Int, Zero, Plus, Minus};

#[cfg(features = "specialization")]
use types::number::balanced::Error;


type_operators! {
    [A, B, C, D, E]

    (IntSum) IntAdd(Int, Int): Int {
        [Zero, Zero] => Zero
        forall (N: Int) {
            [Zero, (Plus N)] => (Plus N)
            [Zero, (Minus N)] => (Minus N)
            [(Plus N), Zero] => (Plus N)
            [(Minus N), Zero] => (Minus N)
        }
        forall (M: Int, N: Int) {
            [(Plus M), (Plus N)] => (Minus (# (# M N) Plus))
            [(Plus M), (Minus N)] => (Zero (# M N))
            [(Minus M), (Plus N)] => (Zero (# M N))
            [(Minus M), (Minus N)] => (Plus (# (# M N) Minus))
        }
    }
}
