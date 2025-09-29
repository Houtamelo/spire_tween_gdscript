#[allow(unused_imports)]
use super::*;

macro_rules! cow_fn {
    ($EnumName: ident ($($arg_name: ident : $arg_ty: ty),*) -> $Out: ty) => {
        pub enum $EnumName<T> {
            Static(fn($($arg_ty),*) -> $Out),
            Boxed(Box<dyn Fn($($arg_ty),*) -> $Out>),
        }

        impl<T> $EnumName<T> {
            #[inline]
            pub fn from_static(f: fn($($arg_ty),*) -> $Out) -> Self { Self::Static(f) }

            #[inline]
            pub fn from_boxed(f: impl 'static + Fn($($arg_ty),*) -> $Out) -> Self { Self::Boxed(Box::new(f)) }
        }

        impl<T> FnOnce<($($arg_ty),*)> for $EnumName<T> {
            type Output = $Out;

            #[inline]
            extern "rust-call" fn call_once(self, args: ($($arg_ty),*)) -> Self::Output {
                let ($($arg_name),*) = args;

                match self {
                    Self::Static(f) => f($($arg_name),*),
                    Self::Boxed(f) => f($($arg_name),*),
                }
            }
        }

        impl<T> FnMut<($($arg_ty),*)> for $EnumName<T> {
            #[inline]
            extern "rust-call" fn call_mut(&mut self, args: ($($arg_ty),*)) -> Self::Output {
                let ($($arg_name),*) = args;

                match self {
                    Self::Static(f) => f($($arg_name),*),
                    Self::Boxed(f) => f($($arg_name),*),
                }
            }
        }

        impl<T> Fn<($($arg_ty),*)> for $EnumName<T> {
            #[inline]
            extern "rust-call" fn call(&self, args: ($($arg_ty),*)) -> Self::Output {
                let ($($arg_name),*) = args;

                match self {
                    Self::Static(f) => f($($arg_name),*),
                    Self::Boxed(f) => f($($arg_name),*),
                }
            }
        }
    };
}

cow_fn! { CowLerpFn (from: &T, to: &T, t: f64) -> T }
cow_fn! { CowRelativeFn (current_at_obj: &T, previous_relative: &T, new_relative: &T) -> T }
cow_fn! { CowStepFn (from: &T, to: &T, speed: f64, t: f64) -> (T, StepResult) }
cow_fn! { CowDistanceFn (from: &T, to: &T) -> f64 }
