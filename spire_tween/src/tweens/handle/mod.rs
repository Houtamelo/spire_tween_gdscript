use std::{
    marker::PhantomData,
    ops::{Deref, DerefMut},
};

pub use untyped::*;

use super::*;

mod untyped;

#[derive(Debug, Clone)]
pub struct SpireHandle<T> {
    pub inner: UntypedHandle,
    _marker:   PhantomData<T>,
}

impl<T> Deref for SpireHandle<T> {
    type Target = UntypedHandle;

    fn deref(&self) -> &Self::Target { &self.inner }
}

impl<T> DerefMut for SpireHandle<T> {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.inner }
}

#[derive(Debug, Clone)]
pub enum FetchError {
    NotFound,
    TypeMismatch {
        expected: &'static str,
        found: &'static str,
    },
}

impl<T> SpireHandle<T> {
    pub fn new(id: TweenId) -> Self {
        Self {
            inner:   UntypedHandle::new(id),
            _marker: PhantomData,
        }
    }

    pub fn map<TMap>(&self, f: impl FnOnce(&SpireTween<T>) -> TMap) -> Result<TMap, FetchError>
    where SpireTween<T>: FromEnumRef<AnyTween> {
        TWEENS
            .inspect(self.id, |any_tween| {
                let found_type = any_tween.inner_type_name();
                let tween = SpireTween::<T>::from_enum_ref(any_tween).ok_or_else(|| {
                    FetchError::TypeMismatch {
                        expected: type_name::<T>(),
                        found: found_type,
                    }
                })?;

                Ok(f(tween))
            })
            .ok_or(FetchError::NotFound)?
    }

    pub fn map_mut<TMap>(
        &self,
        f: impl FnOnce(&mut SpireTween<T>) -> TMap,
    ) -> Result<TMap, FetchError>
    where
        SpireTween<T>: FromEnumMut<AnyTween>,
    {
        TWEENS
            .edit(self.id, |any_tween| {
                let found_type = any_tween.inner_type_name();
                let tween = SpireTween::<T>::from_enum_mut(any_tween).ok_or_else(|| {
                    FetchError::TypeMismatch {
                        expected: type_name::<T>(),
                        found: found_type,
                    }
                })?;

                Ok(f(tween))
            })
            .ok_or(FetchError::NotFound)?
    }
}

#[allow(unused)]
pub fn cast_handle<T, U>(handle: SpireHandle<T>) -> SpireHandle<U> {
    SpireHandle {
        inner:   handle.inner,
        _marker: PhantomData,
    }
}
