use std::ops::Mul;

use super::*;

#[macro_export]
macro_rules! sym {
    ( ($( $elems:literal )+) ) => {{
        let cycle = [$($elems),+];
        let mut perm = ::core::array::from_fn(|i| i + 1);

        let m = cycle.len();
        for i in 0..m {
            let from = cycle[i % m] - 1;
            let to = cycle[(i + 1) % m];
            perm[from] = to;
        }
        $crate::groups::S::from(perm)
    }};
    ($( $inner:tt )+) => {{
        let mut y = $crate::groups::S::identity();
        $(
            y = y * sym![$inner];
        )+
        y
    }};

}

pub type S<const N: usize> = SymmetricGroupElement<N>;

impl<const N: usize> Mul for S<N> {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        self.op(&rhs)
    }
}

impl<const N: usize> From<[usize; N]> for S<N> {
    fn from(perm: [usize; N]) -> Self {
        Self { perm }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct SymmetricGroupElement<const N: usize> {
    perm: [usize; N],
}
pub struct SymmetricGroup<const N: usize>;

impl<const N: usize> GroupElement for S<N> {
    type G = SymmetricGroup<N>;
}

impl<const N: usize> Group<S<N>> for SymmetricGroup<N> {
    fn identity() -> S<N> {
        S {
            perm: core::array::from_fn(|i| i + 1),
        }
    }
    fn op(x: &S<N>, y: &S<N>) -> S<N> {
        S {
            perm: y.perm.map(|i| x.perm[i - 1]),
        }
    }
    fn inverse(x: &S<N>) -> S<N> {
        S {
            perm: x.perm.map(|i| x.perm[i - 1]),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn symmetric_group() {
        let id: S<3> = [1, 2, 3].into();
        let x: S<3> = [2, 1, 3].into(); // (1 2)
        let y: S<3> = [1, 3, 2].into(); // (2 3)
        let z = x * y; // (1 2)(2 3) = (1 2 3)

        assert_eq!(id, S::identity());
        assert_eq!(x * x, S::identity());
        assert_eq!(z.pow(3), S::identity());
        assert_eq!(x * y, [2, 3, 1].into()); // (1 2)(2 3) = (1 2 3)
        assert_eq!(y * x, [3, 1, 2].into()); // (2 3)(1 2) = (1 3 2)
    }

    #[test]
    fn sym_macro() {
        let x: S<3> = sym![(1 2 3)];
        let y = sym![(1 2)(2 3)];

        assert_eq!(x, y);
        assert_eq!(x, [2, 3, 1].into());

        // assert_eq!(sym![(1 2)(2 3)], sym![(1 2 3)]);

        assert_eq!(sym![(1 2 3 4 5 6 7 8)], [2, 3, 4, 5, 6, 7, 8, 1].into());

        assert_eq!(sym!((1 2)(1 2)), S::<10>::identity());
        assert_eq!(sym!((1)), S::<3>::identity())
    }
}
