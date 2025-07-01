use abstract_algebra_macros::Operations;

use crate::ops::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Operations)]
#[operations(Multiplication)]
pub struct Permutation<const N: usize>([usize; N]);

impl<const N: usize> From<[usize; N]> for Permutation<N> {
    fn from(value: [usize; N]) -> Self {
        Self(value)
    }
}

impl<const N: usize> BinaryOperation<Multiplication> for Permutation<N> {
    fn op(&self, y: &Self) -> Self {
        Self(y.0.map(|i| self.0[i - 1]))
    }
}

impl<const N: usize> Identity<Multiplication> for Permutation<N> {
    fn id() -> Self {
        Self(::core::array::from_fn(|i| i + 1))
    }
}

impl<const N: usize> Invertible<Multiplication> for Permutation<N> {
    fn inv(&self) -> Self {
        Self(self.0.map(|i| self.0[i - 1]))
    }
}

#[macro_export]
macro_rules! perm {
    [@cycle; ()] => {{
        $crate::primitives::Permutation::id()
    }};
    [@cycle; ($($elems:literal)+)] => {{
        let cycle = [$($elems),+];
        let mut perm = ::core::array::from_fn(|i| i + 1);

        let m = cycle.len();
        for i in 0..m {
            let from = cycle[i % m] - 1;
            let to = cycle[(i + 1) % m];
            perm[from] = to;
        }
        $crate::primitives::Permutation::from(perm)
    }};
    [ $N:literal ; $( $tt:tt )+] => {{
        let out: $crate::primitives::Permutation::<$N> = perm![$($tt)+];
        out
    }};
    [$($tt:tt)+] => {{
        let mut y = $crate::primitives::Permutation::id();
        $(
            y = y * perm![@cycle; $tt];
        )+
        y
    }}
}

#[cfg(test)]
mod test {
    use crate::perm;

    use super::*;

    #[test]
    fn test() {
        let id: Permutation<4> = crate::primitives::Permutation::id();
        let x = perm![4; (1 2)(2 3)];
        assert_eq!(x * x * x, id)
    }
}
