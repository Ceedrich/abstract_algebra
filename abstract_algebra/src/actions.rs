pub trait Action<X> {
    fn action(&self, set: &X) -> X;
}

#[cfg(test)]
pub fn test_actions<X, G, K, C>(set: X, perms: &[G])
where
    X: std::fmt::Debug + PartialEq,
    G: crate::groups::GroupOperation<K, C> + Action<X>,
    K: crate::ops::OperationKind,
    C: crate::ops::OperationCommutativity,
{
    assert_eq!(G::id().action(&set), set, "e · x = x");

    for perm in perms {
        assert_eq!(
            perm.pow(2).action(&set),
            perm.action(&perm.action(&set)),
            "g · (g · x) = (gg) · x"
        );
        assert_eq!(
            perm.inv().action(&perm.action(&set)),
            set,
            "g^(-1) · (g · x) = x"
        );
        assert_eq!(
            perm.action(&perm.inv().action(&set)),
            set,
            "g · (g^(-1) · x) = x"
        );
    }

    for perms in perms.windows(2) {
        let g = &perms[0];
        let h = &perms[1];

        assert_eq!(
            g.op(h).action(&set),
            g.action(&h.action(&set)),
            "(gh) · x = g · (h · x)"
        );
    }
}
