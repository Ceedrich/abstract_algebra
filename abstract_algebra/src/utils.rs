pub const fn is_prime(n: usize) -> bool {
    if n < 2 {
        return false;
    }
    let mut i = 2;
    while i * i <= n {
        if n.is_multiple_of(i) {
            return false;
        }
        i += 1
    }
    true
}

/// Given  `&T op &T -> T` implements
/// - T op T -> T
/// - &T op T -> T
/// - T op &T -> T
#[macro_export]
macro_rules! impl_op {
    (
        impl<$(const $gen:ident: $gen_t:ty),*> $op:ident ; $fn:ident : $ty:ty $(; $($wc:tt)*)?
    ) => {

        impl<$(const $gen: $gen_t),*> $op< $ty > for $ty
            $( $( $wc )* )?
            {
                type Output =  $ty ;
                fn $fn(self, rhs:  $ty ) -> Self::Output {
                    self.$fn(&rhs)
                }
            }

        impl<$(const $gen: $gen_t),*> $op<& $ty > for $ty
            $( $( $wc )* )?
            {
                type Output =  $ty ;
                fn $fn(self, rhs: & $ty ) -> Self::Output {
                    (&self).$fn(rhs)
                }
            }

        impl<$(const $gen: $gen_t),*> $op< $ty > for & $ty
            $( $( $wc )* )?
            {
                type Output =  $ty ;
                fn $fn(self, rhs:  $ty ) -> Self::Output {
                    self.$fn(&rhs)
                }
            }
    };
    (
        impl<$($gen:tt),*> $op:ident ; $fn:ident : $ty:ty $(; $($wc:tt)*)?
    ) => {

        impl<$($gen),*> $op< $ty > for $ty
            $( $( $wc )* )?
            {
                type Output =  $ty ;
                fn $fn(self, rhs:  $ty ) -> Self::Output {
                    self.$fn(&rhs)
                }
            }

        impl<$($gen),*> $op<& $ty > for $ty
            $( $( $wc )* )?
            {
                type Output =  $ty ;
                fn $fn(self, rhs: & $ty ) -> Self::Output {
                    (&self).$fn(rhs)
                }
            }

        impl<$($gen),*> $op< $ty > for & $ty
            $( $( $wc )* )?
            {
                type Output =  $ty ;
                fn $fn(self, rhs:  $ty ) -> Self::Output {
                    self.$fn(&rhs)
                }
            }
    };
}

#[macro_export]
macro_rules! impl_op_assign {
    (
        impl<$($gen:tt),*> $op:ident ; $fn:ident ; $fn_ass:ident : $ty:ty $(; $($wc:tt)*)?
    ) => {
        impl<$($gen),*> $op<&Self> for $ty $( $($wc)*)?
        {
            fn $fn_ass(&mut self, rhs: &Self) {
                *self = (&*self).$fn(rhs)
            }
        }

        impl<$($gen),*> $op<Self> for $ty $( $($wc)*)?
        {
            fn $fn_ass(&mut self, rhs: Self) {
                *self = (&*self).$fn(&rhs)
            }
        }
    };
    (
        impl<$(const $gen:ident: $gen_t:ty),*> $op:ident ; $fn:ident ; $fn_ass:ident : $ty:ty $(; $($wc:tt)*)?
    ) => {
        impl<$(const $gen: $gen_t),*> $op<&Self> for $ty $( $($wc)*)?
        {
            fn $fn_ass(&mut self, rhs: &Self) {
                *self = (&*self).$fn(rhs)
            }
        }

        impl<$(const $gen: $gen_t),*> $op<Self> for $ty $( $($wc)*)?
        {
            fn $fn_ass(&mut self, rhs: Self) {
                *self = (&*self).$fn(&rhs)
            }
        }
    };
}
