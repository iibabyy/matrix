macro_rules! impl_add_assign {
    (
        <$($generic:ident),+> $for:ty, $with:ty,
        with $func:expr,
        where $($rules:tt)+
    ) => {
        impl<$($generic),*> AddAssign<$with> for $for
        where
            $($rules)+
        {
            fn add_assign(&mut self, other: $with) {
                let new_self: $for = $func(self, &other);
                *self = new_self;
            }
        }

        impl<$($generic),*> AddAssign<&$with> for $for
        where
            $($rules)+
        {
            fn add_assign(&mut self, other: &$with) {
                let new_self: $for = $func(self, other);
                *self = new_self;
            }
        }
    };

}

macro_rules! impl_add {
    (
        <$($generic:ident),+> $for:ty, $with:ty,
        where $($rules:tt)+
    ) => {
        impl<$($generic),*> Add<$with> for $for
        where
            $($rules)+
        {
            type Output = $for;

            fn add(mut self, other: $with) -> Self::Output {
                self += other;
                self
            }
        }

        impl<$($generic),*> Add<&$with> for $for
        where
            $($rules)+
        {
            type Output = $for;

            fn add(mut self, other: &$with) -> Self::Output {
                self += other;
                self
            }
        }

        impl<$($generic),*> Add<$with> for &$for
        where
            Self: Clone,
            $($rules)+
        {
            type Output = $for;

            fn add(self, other: $with) -> Self::Output {
                let mut new = self.clone();
                new += other;
                new
            }
        }

        impl<$($generic),*> Add<&$with> for &$for
        where
            Self: Clone,
            $($rules)+
        {
            type Output = $for;

            fn add(self, other: &$with) -> Self::Output {
                let mut new = self.clone();
                new += other;
                new
            }
        }
    };
}

macro_rules! impl_add_ops {
    (
        <$($generic:ident),+> $for:ty, $with:ty,
        with $func:expr,
        where $($rules:tt)+
    ) => {
        impl_add_assign!(
            <$($generic),+> $for, $with,
            with $func,
            where $($rules)+
        );

        impl_add!(
            <$($generic),+> $for, $with,
            where $($rules)+
        );
    };
}

macro_rules! impl_sub_assign {
    (
        <$($generic:ident),+> $for:ty, $with:ty,
        with $func:expr,
        where $($rules:tt)+
    ) => {
        impl<$($generic),*> SubAssign<$with> for $for
        where
            $( $rules )*
        {
            fn sub_assign(&mut self, other: $with) {
                let new_self: $for = $func(self, &other);
                *self = new_self;
            }
        }

        impl<$($generic),*> SubAssign<&$with> for $for
        where
            $( $rules )*
        {
            fn sub_assign(&mut self, other: &$with) {
                let new_self: $for = $func(self, other);
                *self = new_self;
            }
        }
    };
}

macro_rules! impl_sub {
    (
        <$($generic:ident),+> $for:ty, $with:ty,
        where $($rules:tt)+
    ) => {
        impl<$($generic),*> Sub<$with> for $for
        where
            $( $rules )+
        {
            type Output = $for;

            fn sub(mut self, other: $with) -> Self::Output {
                self -= other;
                self
            }
        }

        impl<$($generic),*> Sub<&$with> for $for
        where
            $( $rules )+
        {
            type Output = $for;

            fn sub(mut self, other: &$with) -> Self::Output {
                self -= other;
                self
            }
        }

        impl<$($generic),*> Sub<$with> for &$for
        where
            Self: Clone,
            $( $rules )+
        {
            type Output = $for;

            fn sub(self, other: $with) -> Self::Output {
                let mut new = self.clone();
                new -= other;
                new
            }
        }

        impl<$($generic),*> Sub<&$with> for &$for
        where
            Self: Clone,
            $( $rules )+
        {
            type Output = $for;

            fn sub(self, other: &$with) -> Self::Output {
                let mut new = self.clone();
                new -= other;
                new
            }
        }
    };
}

macro_rules! impl_sub_ops {
    (
        <$($generic:ident),+> $for:ty, $with:ty,
        with $func:expr,
        where $($rules:tt)+
    ) => {
        impl_sub_assign!(
            <$($generic),+> $for, $with,
            with $func,
            where $($rules)+
        );

        impl_sub!(
            <$($generic),+> $for, $with,
            where $($rules)+
        );
    };
}

macro_rules! impl_mul_assign {
    (
        <$($generic:ident),+> $for:ty, $with:ty,
        with $func:expr,
        where $($rules:tt)+
    ) => {
        impl<$($generic),*> MulAssign<$with> for $for
        where
            $( $rules )+
        {
            fn mul_assign(&mut self, other: $with) {
                let new_self: $for = $func(self, &other);
                *self = new_self;
            }
        }

        impl<$($generic),*> MulAssign<&$with> for $for
        where
            $( $rules )+
        {
            fn mul_assign(&mut self, other: &$with) {
                let new_self: $for = $func(self, other);
                *self = new_self;
            }
        }
    };
}

macro_rules! impl_mul {
    (
        <$($generic:ident),+> $for:ty, $with:ty,
        where $($rules:tt)+
    ) => {
        impl<$($generic),*> Mul<$with> for $for
        where
            $( $rules )+
        {
            type Output = $for;

            fn mul(mut self, other: $with) -> Self::Output {
                self *= other;
                self
            }
        }

        impl<$($generic),*> Mul<&$with> for $for
        where
            $( $rules )+
        {
            type Output = $for;

            fn mul(mut self, other: &$with) -> Self::Output {
                self *= other;
                self
            }
        }

        impl<$($generic),*> Mul<$with> for &$for
        where
            Self: Clone,
            $( $rules )+
        {
            type Output = $for;

            fn mul(self, other: $with) -> Self::Output {
                let mut new = self.clone();
                new *= other;
                new
            }
        }

        impl<$($generic),*> Mul<&$with> for &$for
        where
            Self: Clone,
            $( $rules )+
        {
            type Output = $for;

            fn mul(self, other: &$with) -> Self::Output {
                let mut new = self.clone();
                new *= other;
                new
            }
        }
    };
}

macro_rules! impl_mul_ops {
    (
        <$($generic:ident),+> $for:ty, $with:ty,
        with $func:expr,
        where $($rules:tt)+
    ) => {
        impl_mul_assign!(
            <$($generic),+> $for, $with,
            with $func,
            where $($rules)+
        );

        impl_mul!(
            <$($generic),+> $for, $with,
            where $($rules)+
        );
    };
}

macro_rules! use_arithmetic_macros {
    () => {
        use crate::macros::arithmetics::{
            impl_add, impl_add_assign, impl_add_ops,
            impl_mul, impl_mul_assign, impl_mul_ops,
            impl_sub, impl_sub_assign, impl_sub_ops,
        };
    };
}

pub(crate) use use_arithmetic_macros;

pub(crate) use impl_add;
pub(crate) use impl_add_assign;
pub(crate) use impl_add_ops;
pub(crate) use impl_sub;
pub(crate) use impl_sub_assign;
pub(crate) use impl_sub_ops;
pub(crate) use impl_mul;
pub(crate) use impl_mul_assign;
pub(crate) use impl_mul_ops;