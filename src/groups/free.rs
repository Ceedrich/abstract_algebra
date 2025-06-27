use super::*;

pub type F = Group<FreeGroupElement>;

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
/// assert_eq!(free!('b' 'a' 'a'- 'b'-), F::id());
/// assert_eq!(x.op(&y), F::id())
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
    fn inv(&self) -> Self {
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
        let word = FreeGroupElement::reduce(value.into());
        Self(FreeGroupElement { word })
    }
}

impl FreeGroupElement {
    fn reduce(v: Vec<Alphabet>) -> Vec<Alphabet> {
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

impl GroupElement for FreeGroupElement {
    fn id() -> Self {
        Self { word: vec![] }
    }

    fn inv(&self) -> Self {
        Self {
            word: self.word.iter().rev().map(Alphabet::inv).collect(),
        }
    }

    fn op(&self, y: &Self) -> Self {
        let mut word = self.word.clone();
        word.extend(&y.word);
        FreeGroupElement {
            word: Self::reduce(word),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn free_group() {
        let id = F::id();
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
        assert_eq!(a.inv(), b);
        assert_eq!(id, a * b);
    }

    #[test]
    fn free_macro() {}
}
