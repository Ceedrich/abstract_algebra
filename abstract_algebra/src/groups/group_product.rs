use abstract_algebra_macros::Blub;

use super::GroupOperation;
use crate::ops::{OperationCommutativity, OperationKind};

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Blub)]
#[blub(
    accessor(.e : (A, B)),
)]
pub struct GroupProduct<A, B, KA, KB, CA, CB>
where
    A: GroupOperation<KA, CA>,
    B: GroupOperation<KB, CB>,
    KA: OperationKind,
    KB: OperationKind,
    CA: OperationCommutativity,
    CB: OperationCommutativity,
{
    e: (A, B),
    _kind_a: KA,
    _kind_b: KB,
    _commutativity_a: CA,
    _commutativity_b: CB,
}
