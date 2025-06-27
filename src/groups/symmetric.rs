use super::*;

#[macro_export]
macro_rules! sym {
    [@cycle; ()] => {{ $crate::groups::S::id() }};
    [@cycle; ($($elems:literal)+)] => {{
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
    [ $N:literal ; $( $tt:tt )+] => {{
        let out: $crate::groups::S::<$N> = sym![$($tt)+];
        out
    }};
    [$($tt:tt)+] => {{
        let mut y = $crate::groups::S::id();
        $(
            y = y * sym![@cycle; $tt];
        )+
        y
    }}
}

pub type S<const N: usize> = Group<SymmetricGroupElement<N>>;

impl<const N: usize> From<[usize; N]> for S<N> {
    fn from(perm: [usize; N]) -> Self {
        Self(SymmetricGroupElement { perm })
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct SymmetricGroupElement<const N: usize> {
    perm: [usize; N],
}

impl<const N: usize> GroupElement for SymmetricGroupElement<N> {
    fn id() -> Self {
        Self {
            perm: core::array::from_fn(|i| i + 1),
        }
    }
    fn op(&self, y: &Self) -> Self {
        Self {
            perm: y.perm.map(|i| self.perm[i - 1]),
        }
    }
    fn inv(&self) -> Self {
        Self {
            perm: self.perm.map(|i| self.perm[i - 1]),
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

        assert_eq!(id, S::id());
        assert_eq!(x * x, S::id());
        assert_eq!(z.pow(3), S::id());
        assert_eq!(x * y, [2, 3, 1].into()); // (1 2)(2 3) = (1 2 3)
        assert_eq!(y * x, [3, 1, 2].into()); // (2 3)(1 2) = (1 3 2)
    }

    #[test]
    fn sym_macro() {
        let id = sym![5; ()];
        let x: S<3> = sym![(1 2 3)];
        let y = sym![(1 2)(2 3)];

        assert_eq!(id, S::id());
        assert_eq!(x, y);
        assert_eq!(x, [2, 3, 1].into());

        assert_eq!(sym![3; (1 2 3)], sym![(1 2)(2 3)]);

        assert_eq!(sym![(1 2 3 4 5 6 7 8)], [2, 3, 4, 5, 6, 7, 8, 1].into());

        assert_eq!(sym!((1 2)(1 2)), S::<10>::id());
        assert_eq!(sym!((1)), S::<3>::id())
    }
}
