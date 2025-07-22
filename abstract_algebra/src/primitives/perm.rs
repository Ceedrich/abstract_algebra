use crate::ops::{Associativity, BinOp, Identity, Invertible, Multiplication};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Permutation<const N: usize>([usize; N]);

impl<const N: usize> From<[usize; N]> for Permutation<N> {
    fn from(value: [usize; N]) -> Self {
        Self(value)
    }
}

impl<const N: usize> Associativity<Multiplication> for Permutation<N> {}

impl<const N: usize> BinOp<Multiplication> for Permutation<N> {
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
        let mut inverse = [0; N];
        for (i, &v) in self.0.iter().enumerate() {
            inverse[v - 1] = i + 1
        }
        Self(inverse)
    }
}

// impl<T: Default + Clone, const N: usize> Action<[T; N]> for Permutation<N> {
//     fn action(&self, set: &[T; N]) -> [T; N] {
//         let mut out: [T; N] = core::array::from_fn(|_| T::default());
//         for (i, x) in set.iter().enumerate() {
//             out[self.0[i] - 1] = x.clone();
//         }
//         out
//     }
// }

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
            y = $crate::ops::BinOp::<$crate::ops::Multiplication>::op(&y, &perm![@cycle; $tt]);
        )+
        y
    }}
}

#[cfg(test)]
mod test {
    use crate::{perm, structures::Group};

    use super::*;

    #[test]
    fn perm_macro() {
        let id = perm![5; ()];
        assert_eq!(id, Permutation::id());

        assert_eq!(perm![5; (1 2 3)(4 5)].inv(), perm![5; (1 3 2)(4 5)]);
        assert_eq!(perm![3; (1 2 3)], perm![3; (2 3 1)]);
        assert_eq!(perm![(1 2)(2 3)], perm![5; (1 2 3)]);

        assert_eq!(perm![5; (1 2 3 4 5)].inv(), perm![(1 5 4 3 2)]);
    }

    #[test]
    fn test() {
        let id = perm![5; ()];
        let a = perm![5; (1 2 3)(4 5)];
        let b = perm![5; (1 2)];
        let c = perm![5; (1 3 4)];

        assert_eq!(a.pow(6), id);
        assert_eq!(a.pow(2), perm![(1 3 2)]);
        assert_eq!(a.pow(3), perm![(4 5)]);

        // test_group_axioms(&[a, b, c]);
    }

    // #[test]
    // fn group_action() {
    //     let set = ['a', 'b', 'c', 'd', 'e'];
    //     let perm = perm![5; (1 3 5)(2 4)];
    //
    //     assert_eq!(perm.action(&set), ['e', 'd', 'a', 'b', 'c']);
    //     // test_actions(set, &[perm]);
    // }
}
