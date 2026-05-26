mod add {
    macro_rules! impl_add_assign {
        (
            <$($generic:ident),*> $for:ty, $with:ty,
            with $func:expr,
            $(where $($rules:tt)+)?
        ) => {
            impl<$($generic),*> std::ops::AddAssign<$with> for $for
            $(where $($rules)+)?
            {
                fn add_assign(&mut self, other: $with) {
                    let new_self: $for = $func(self, &other);
                    *self = new_self;
                }
            }

            impl<$($generic),*> std::ops::AddAssign<&$with> for $for
            $(where $($rules)+)?
            {
                fn add_assign(&mut self, other: &$with) {
                    let new_self: $for = $func(self, other);
                    *self = new_self;
                }
            }
        };

    }
    pub(crate) use impl_add_assign;

    macro_rules! impl_add {
        (
            <$($generic:ident),*> $for:ty, $with:ty,
            $(where $($rules:tt)+)?
        ) => {
            $crate::macros::arithmetics::impl_add!(
                <$($generic),*> $for, $with,
                Output = $for,
                $(where $($rules)+)?
            );
        };

        (
            <$($generic:ident),*> $for:ty, $with:ty,
            Output = $output:ty,
            $(where $($rules:tt)+)?
        ) => {
            impl<$($generic),*> std::ops::Add<$with> for $for
            $(where $($rules)+)?
            {
                type Output = $output;

                fn add(mut self, other: $with) -> Self::Output {
                    self += other;
                    self
                }
            }

            impl<$($generic),*> std::ops::Add<&$with> for $for
            $(where $($rules)+)?
            {
                type Output = $output;

                fn add(mut self, other: &$with) -> Self::Output {
                    self += other;
                    self
                }
            }

            impl<$($generic),*> std::ops::Add<$with> for &$for
            where
                Self: Clone,
                $($($rules)+)?
            {
                type Output = $output;

                fn add(self, other: $with) -> Self::Output {
                    let mut new = self.clone();
                    new += other;
                    new
                }
            }

            impl<$($generic),*> std::ops::Add<&$with> for &$for
            where
                Self: Clone,
                $($($rules)+)?
            {
                type Output = $output;

                fn add(self, other: &$with) -> Self::Output {
                    let mut new = self.clone();
                    new += other;
                    new
                }
            }
        };
    }
    pub(crate) use impl_add;

    macro_rules! impl_add_ops {
        (
            <$($generic:ident),*> $for:ty, $with:ty,
            with $func:expr,
            $(where $($rules:tt)+)?
        ) => {
            $crate::macros::arithmetics::impl_add_assign!(
                <$($generic),*> $for, $with,
                with $func,
                $(where $($rules)+)?
            );

            $crate::macros::arithmetics::impl_add!(
                <$($generic),*> $for, $with,
                $(where $($rules)+)?
            );
        };
    }
    pub(crate) use impl_add_ops;
}

mod sub {
    macro_rules! impl_sub_assign {
        (
            <$($generic:ident),*> $for:ty, $with:ty,
            with $func:expr,
            $(where $($rules:tt)+)?
        ) => {
            impl<$($generic),*> std::ops::SubAssign<$with> for $for
            where
                $($( $rules )*)?
            {
                fn sub_assign(&mut self, other: $with) {
                    let new_self: $for = $func(self, &other);
                    *self = new_self;
                }
            }

            impl<$($generic),*> std::ops::SubAssign<&$with> for $for
            where
                $($( $rules )*)?
            {
                fn sub_assign(&mut self, other: &$with) {
                    let new_self: $for = $func(self, other);
                    *self = new_self;
                }
            }
        };
    }
    pub(crate) use impl_sub_assign;

    macro_rules! impl_sub {
        (
            <$($generic:ident),*> $for:ty, $with:ty,
            $(where $($rules:tt)+)?
        ) => {
            $crate::macros::arithmetics::impl_sub!(
                <$($generic),*> $for, $with,
                Output = $for,
                $(where $($rules)+)?
            );
        };

        (
            <$($generic:ident),*> $for:ty, $with:ty,
            Output = $output:ty,
            $(where $($rules:tt)+)?
        ) => {
            impl<$($generic),*> std::ops::Sub<$with> for $for
            where
                $($( $rules )+)?
            {
                type Output = $output;

                fn sub(mut self, other: $with) -> Self::Output {
                    self -= other;
                    self
                }
            }

            impl<$($generic),*> std::ops::Sub<&$with> for $for
            where
                $($( $rules )+)?
            {
                type Output = $output;

                fn sub(mut self, other: &$with) -> Self::Output {
                    self -= other;
                    self
                }
            }

            impl<$($generic),*> std::ops::Sub<$with> for &$for
            where
                Self: Clone,
                $($( $rules )+)?
            {
                type Output = $output;

                fn sub(self, other: $with) -> Self::Output {
                    let mut new = self.clone();
                    new -= other;
                    new
                }
            }

            impl<$($generic),*> std::ops::Sub<&$with> for &$for
            where
                Self: Clone,
                $($( $rules )+)?
            {
                type Output = $output;

                fn sub(self, other: &$with) -> Self::Output {
                    let mut new = self.clone();
                    new -= other;
                    new
                }
            }
        };
    }
    pub(crate) use impl_sub;

    macro_rules! impl_sub_ops {
        (
            <$($generic:ident),*> $for:ty, $with:ty,
            with $func:expr,
            $(where $($rules:tt)+)?
        ) => {
            $crate::macros::arithmetics::impl_sub_assign!(
                <$($generic),*> $for, $with,
                with $func,
                $(where $($rules)+)?
            );

            $crate::macros::arithmetics::impl_sub!(
                <$($generic),*> $for, $with,
                $(where $($rules)+)?
            );
        };
    }
    pub(crate) use impl_sub_ops;
}

mod mul {
    macro_rules! impl_mul_assign {
        (
            <$($generic:ident),*> $for:ty, $with:ty,
            with $func:expr,
            $(where $($rules:tt)+)?
        ) => {
            impl<$($generic),*> std::ops::MulAssign<$with> for $for
            where
                $($( $rules )+)?
            {
                fn mul_assign(&mut self, other: $with) {
                    let new_self: $for = $func(self, &other);
                    *self = new_self;
                }
            }

            impl<$($generic),*> std::ops::MulAssign<&$with> for $for
            where
                $($( $rules )+)?
            {
                fn mul_assign(&mut self, other: &$with) {
                    let new_self: $for = $func(self, other);
                    *self = new_self;
                }
            }
        };
    }
    pub(crate) use impl_mul_assign;

    macro_rules! impl_mul {
        (
            <$($generic:ident),*> $for:ty, $with:ty,
            $(where $($rules:tt)+)?
        ) => {
            $crate::macros::arithmetics::impl_mul!(
                <$($generic),*> $for, $with,
                Output = $for,
                $(where $($rules)+)?
            );
        };

        (
            <$($generic:ident),*> $for:ty, $with:ty,
            Output = $output:ty,
            $(where $($rules:tt)+)?
        ) => {
            impl<$($generic),*> std::ops::Mul<$with> for $for
            where
                $($( $rules )+)?
            {
                type Output = $output;

                fn mul(mut self, other: $with) -> Self::Output {
                    self *= other;
                    self
                }
            }

            impl<$($generic),*> std::ops::Mul<&$with> for $for
            where
                $($( $rules )+)?
            {
                type Output = $output;

                fn mul(mut self, other: &$with) -> Self::Output {
                    self *= other;
                    self
                }
            }

            impl<$($generic),*> std::ops::Mul<$with> for &$for
            where
                Self: Clone,
                $($( $rules )+)?
            {
                type Output = $output;

                fn mul(self, other: $with) -> Self::Output {
                    let mut new = self.clone();
                    new *= other;
                    new
                }
            }

            impl<$($generic),*> std::ops::Mul<&$with> for &$for
            where
                Self: Clone,
                $($( $rules )+)?
            {
                type Output = $output;

                fn mul(self, other: &$with) -> Self::Output {
                    let mut new = self.clone();
                    new *= other;
                    new
                }
            }
        };
    }
    pub(crate) use impl_mul;

    macro_rules! impl_mul_reverse {
        (
            <$($generic:ident),*> $for:ty, $with:ty,
            $(where $($rules:tt)+)?
        ) => {
            impl<$($generic),*> std::ops::Mul<$for> for $with
            where
                $($( $rules )+)?
            {
                type Output = <$for as std::ops::Mul<Self>>::Output;

                fn mul(self, other: $for) -> Self::Output {
                    other * self
                }
            }

            impl<$($generic),*> std::ops::Mul<&$for> for $with
            where
                $($( $rules )+)?
            {
                type Output = <$for as std::ops::Mul<Self>>::Output;

                fn mul(self, other: &$for) -> Self::Output {
                    other * self
                }
            }

            impl<$($generic),*> std::ops::Mul<$for> for &$with
            where
                Self: Clone,
                $($( $rules )+)?
            {
                type Output = <$for as std::ops::Mul<Self>>::Output;

                fn mul(self, other: $for) -> Self::Output {
                    other * self
                }
            }

            impl<$($generic),*> std::ops::Mul<&$for> for &$with
            where
                Self: Clone,
                $($( $rules )+)?
            {
                type Output = <$for as std::ops::Mul<Self>>::Output;

                fn mul(self, other: &$for) -> Self::Output {
                    other * self
                }
            }
        };
    }
    pub(crate) use impl_mul_reverse;

    macro_rules! impl_mul_ops {
        (
            <$($generic:ident),*> $for:ty, $with:ty,
            with $func:expr,
            $(where $($rules:tt)+)?
        ) => {
            $crate::macros::arithmetics::impl_mul_assign!(
                <$($generic),*> $for, $with,
                with $func,
                $(where $($rules)+)?
            );

            $crate::macros::arithmetics::impl_mul!(
                <$($generic),*> $for, $with,
                $(where $($rules)+)?
            );
        };
    }
    pub(crate) use impl_mul_ops;
}

mod div {
    macro_rules! impl_div_assign {
        (
            <$($generic:ident),*> $for:ty, $with:ty,
            with $func:expr,
            $(where $($rules:tt)+)?
        ) => {
            impl<$($generic),*> std::ops::DivAssign<$with> for $for
            where
                $($( $rules )+)?
            {
                fn div_assign(&mut self, other: $with) {
                    let new_self: $for = $func(self, &other);
                    *self = new_self;
                }
            }

            impl<$($generic),*> std::ops::DivAssign<&$with> for $for
            where
                $($( $rules )+)?
            {
                fn div_assign(&mut self, other: &$with) {
                    let new_self: $for = $func(self, other);
                    *self = new_self;
                }
            }
        };
    }
    pub(crate) use impl_div_assign;

    macro_rules! impl_div {
        (
            <$($generic:ident),*> $for:ty, $with:ty,
            $(where $($rules:tt)+)?
        ) => {
            $crate::macros::arithmetics::impl_div!(
                <$($generic),*> $for, $with,
                Output = $for,
                $(where $($rules)+)?
            );
        };

        (
            <$($generic:ident),*> $for:ty, $with:ty,
            Output = $output:ty,
            $(where $($rules:tt)+)?
        ) => {
            impl<$($generic),*> std::ops::Div<$with> for $for
            where
                $($( $rules )+)?
            {
                type Output = $output;

                fn div(mut self, other: $with) -> Self::Output {
                    self /= other;
                    self
                }
            }

            impl<$($generic),*> std::ops::Div<&$with> for $for
            where
                $($( $rules )+)?
            {
                type Output = $output;

                fn div(mut self, other: &$with) -> Self::Output {
                    self /= other;
                    self
                }
            }

            impl<$($generic),*> std::ops::Div<$with> for &$for
            where
                Self: Clone,
                $($( $rules )+)?
            {
                type Output = $output;

                fn div(self, other: $with) -> Self::Output {
                    let mut new = self.clone();
                    new /= other;
                    new
                }
            }

            impl<$($generic),*> std::ops::Div<&$with> for &$for
            where
                Self: Clone,
                $($( $rules )+)?
            {
                type Output = $output;

                fn div(self, other: &$with) -> Self::Output {
                    let mut new = self.clone();
                    new /= other;
                    new
                }
            }
        };
    }
    pub(crate) use impl_div;

    macro_rules! impl_div_ops {
        (
            <$($generic:ident),*> $for:ty, $with:ty,
            with $func:expr,
            $(where $($rules:tt)+)?
        ) => {
            $crate::macros::arithmetics::impl_div_assign!(
                <$($generic),*> $for, $with,
                with $func,
                $(where $($rules)+)?
            );

            $crate::macros::arithmetics::impl_div!(
                <$($generic),*> $for, $with,
                $(where $($rules)+)?
            );
        };
    }
    pub(crate) use impl_div_ops;
}

pub(crate) use add::impl_add;
pub(crate) use add::impl_add_assign;
pub(crate) use add::impl_add_ops;

pub(crate) use sub::impl_sub;
pub(crate) use sub::impl_sub_assign;
pub(crate) use sub::impl_sub_ops;

pub(crate) use mul::impl_mul;
pub(crate) use mul::impl_mul_assign;
pub(crate) use mul::impl_mul_ops;
pub(crate) use mul::impl_mul_reverse;

pub(crate) use div::impl_div;
pub(crate) use div::impl_div_assign;
pub(crate) use div::impl_div_ops;
