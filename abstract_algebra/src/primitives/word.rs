use crate::ops::{Associativity, BinOp, Identity, Invertible, Multiplication};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Alphabet<T: Copy + Eq> {
    Fwd(T),
    Bwd(T),
}

impl<T: Copy + Eq> Alphabet<T> {
    pub fn inv(&self) -> Self {
        match self {
            Self::Fwd(x) => Self::Bwd(*x),
            Self::Bwd(x) => Self::Fwd(*x),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Word<T: Copy + Eq>(Vec<Alphabet<T>>);

impl<T: Copy + Eq> Word<T> {
    fn reduce(v: Vec<Alphabet<T>>) -> Vec<Alphabet<T>> {
        let mut reduced = Vec::new();
        for sym in v.into_iter() {
            if let Some(last) = reduced.last() {
                if sym.inv() == *last {
                    reduced.pop();
                    continue;
                }
            }
            reduced.push(sym);
        }
        reduced
    }
}

impl<T: Copy + Eq, const N: usize> From<&[Alphabet<T>; N]> for Word<T> {
    fn from(value: &[Alphabet<T>; N]) -> Self {
        let word = Self::reduce(value.into());
        Self(word)
    }
}

impl<T: Copy + Eq> Associativity<Multiplication> for Word<T> {}

impl<T: Copy + Eq> BinOp<Multiplication> for Word<T> {
    fn op(&self, y: &Self) -> Self {
        let mut word = self.0.clone();
        word.extend(y.0.clone());
        Self(Self::reduce(word))
    }
}

impl<T: Copy + Eq> Identity<Multiplication> for Word<T> {
    fn id() -> Self {
        Self(vec![])
    }
}

impl<T: Copy + Eq> Invertible<Multiplication> for Word<T> {
    fn inv(&self) -> Self {
        Self(self.0.iter().rev().map(Alphabet::inv).collect())
    }
}

/// Macro used to create words groups
///
/// # Example
/// ```rust
/// use abstract_algebra::ops::Identity;
/// use abstract_algebra::primitives::Word;
/// use abstract_algebra::word;
///
/// let x = word!['a' 'b'- 'c'-];
/// let y = word!['c' 'b' 'a'-];
///
/// assert_eq!(word!('b' 'a' 'a'- 'b'-), Word::id());
/// assert_eq!(x * y, Word::id())
/// ```
#[macro_export]
macro_rules! word {
    [@internal; ($l:literal-)] => {
        $crate::primitives::Alphabet::Bwd($l)
    };
    [@internal; ($l:literal)] => {
        $crate::primitives::Alphabet::Fwd($l)
    };
    [@internal; $($tt:tt)*] => {{
        $crate::primitives::Word::from(&[
            $(
                word![@internal; $tt]
            ),*
        ])
    }};
    [( $($done:tt)+ ) $tt:tt-] => {
        word![@internal; $($done)+ ($tt-)]
    };
    [( $($done:tt)+ ) $tt:tt] => {
        word![@internal; $($done)+ ($tt)]
    };
    [( $($done:tt)* ) $tt:tt- $($rest:tt)*]=> {
        word![ ( $($done)+ ($tt-) ) $($rest)*]
    };
    [( $($done:tt)* ) $tt:tt $($rest:tt)*]=> {
        word![ ( $($done)+ ($tt) ) $($rest)*]
    };
    [$tt:tt- $($rest:tt)*] => {{
        word![ ( ($tt-) ) $($rest)*]
    }};
    [$tt:tt $($rest:tt)*] => {{
        word![ ( ($tt) ) $($rest)*]
    }}
}
