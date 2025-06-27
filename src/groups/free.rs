use super::*;

pub type F = FreeGroupElement;

/// Macro used to create free groups
///
/// # Example
/// ```rust
/// use abstract_algebra::groups::{F, GroupElement};
/// use abstract_algebra::free;
///
/// let x = free!['a' 'b'- 'c'-];
/// let y = free!['c' 'b' 'a'-];
///
/// assert_eq!(free!('b' 'a' 'a'- 'b'-), F::identity());
/// assert_eq!(x.op(&y), F::identity())
/// ```
#[macro_export]
macro_rules! free {
    [@internal; ($l:literal-)] => {
        $crate::groups::Alphabet::Inv($l as usize)
    };
    [@internal; ($l:literal)] => {
        $crate::groups::Alphabet::Gen($l as usize)
    };
    [@internal; $($tt:tt)*] => {{
        F::from(&[
            $(
                free![@internal; $tt]
            ),*
        ])
    }};
    [( $($done:tt)+ ) $tt:tt-] => {
        free![@internal; $($done)+ ($tt-)]
    };
    [( $($done:tt)+ ) $tt:tt] => {
        free![@internal; $($done)+ ($tt)]
    };
    [( $($done:tt)* ) $tt:tt- $($rest:tt)*]=> {
        free![ ( $($done)+ ($tt-) ) $($rest)*]
    };
    [( $($done:tt)* ) $tt:tt $($rest:tt)*]=> {
        free![ ( $($done)+ ($tt) ) $($rest)*]
    };
    [$tt:tt- $($rest:tt)*] => {{
        free![ ( ($tt-) ) $($rest)*]
    }};
    [$tt:tt $($rest:tt)*] => {{
        free![ ( ($tt) ) $($rest)*]
    }}
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Alphabet {
    Gen(usize),
    Inv(usize),
}

impl Alphabet {
    fn inverse(&self) -> Self {
        match *self {
            Self::Gen(x) => Self::Inv(x),
            Self::Inv(x) => Self::Gen(x),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct FreeGroupElement {
    word: Vec<Alphabet>,
}

impl<const M: usize> From<&[Alphabet; M]> for F {
    fn from(value: &[Alphabet; M]) -> Self {
        let word = FreeGroup::reduce(value.into());
        Self { word }
    }
}

pub struct FreeGroup;

impl FreeGroup {
    fn reduce(v: Vec<Alphabet>) -> Vec<Alphabet> {
        let mut reduced = Vec::new();
        for sym in v.into_iter() {
            if let Some(last) = reduced.last() {
                if sym.inverse() == *last {
                    reduced.pop();
                    continue;
                }
            }
            reduced.push(sym);
        }
        reduced
    }
}

impl Group<F> for FreeGroup {
    fn identity() -> F {
        F { word: vec![] }
    }

    fn inverse(x: &F) -> F {
        F {
            word: x.word.iter().rev().map(Alphabet::inverse).collect(),
        }
    }

    fn op(x: &F, y: &F) -> F {
        let mut word = x.word.clone();
        word.extend(&y.word);
        F {
            word: Self::reduce(word),
        }
    }
}

impl GroupElement for F {
    type G = FreeGroup;
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn free_group() {
        let id = F::identity();
        let a: F = F::from(&[Alphabet::Gen(0), Alphabet::Gen(1)]);
        let b: F = F::from(&[Alphabet::Inv(1), Alphabet::Inv(0)]);

        assert_eq!(
            a.op(&a),
            F::from(&[
                Alphabet::Gen(0),
                Alphabet::Gen(1),
                Alphabet::Gen(0),
                Alphabet::Gen(1)
            ])
        );
        assert_eq!(a.inverse(), b);
        assert_eq!(id, a.op(&b))
    }

    #[test]
    fn free_macro() {}
}

