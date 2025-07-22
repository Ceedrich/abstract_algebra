use crate::{ops::OperationKind, primitives::Permutation, structures::Group};

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        let g = 32;
        let x = 48;

        let y = IdentityAction::action(&g, &x);

        assert_eq!(x, y)
    }
}
