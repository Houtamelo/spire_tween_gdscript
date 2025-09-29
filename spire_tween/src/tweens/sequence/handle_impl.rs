use crate::internal_prelude::*;

impl SpireHandle<Sequence> {
    pub fn append(&mut self, tween: impl Into<AnyTween>) -> Result<(), FetchError> {
        self.map_mut(|seq| seq.append(tween))
    }

    pub fn append_call(&mut self, f: impl FnMut() + 'static) -> Result<(), FetchError> {
        self.map_mut(|seq| seq.append_call(f))
    }

    pub fn append_callable(&mut self, callable: Callable) -> Result<(), FetchError> {
        self.map_mut(|seq| seq.append_callable(callable))
    }

    pub fn append_interval(&mut self, duration: f64) -> Result<(), FetchError> {
        self.map_mut(|seq| seq.append_interval(duration))
    }

    pub fn join(&mut self, tween: impl Into<AnyTween>) -> Result<(), FetchError> {
        self.map_mut(|seq| seq.join(tween))
    }

    pub fn join_call(&mut self, f: impl FnMut() + 'static) -> Result<(), FetchError> {
        self.map_mut(|seq| seq.join_call(f))
    }

    pub fn join_callable(&mut self, callable: Callable) -> Result<(), FetchError> {
        self.map_mut(|seq| seq.join_callable(callable))
    }

    pub fn insert(&mut self, time: f64, tween: impl Into<AnyTween>) -> Result<(), FetchError> {
        self.map_mut(|seq| seq.insert(time, tween))
    }

    pub fn insert_call(&mut self, time: f64, f: impl FnMut() + 'static) -> Result<(), FetchError> {
        self.map_mut(|seq| seq.insert_call(time, f))
    }

    pub fn insert_callable(&mut self, time: f64, callable: Callable) -> Result<(), FetchError> {
        self.map_mut(|seq| seq.insert_callable(time, callable))
    }
}
