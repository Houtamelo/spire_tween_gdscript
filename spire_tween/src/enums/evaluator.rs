use super::*;

pub enum Evaluator<T> {
    Static(T),
    Dynamic(Box<dyn FnMut() -> T>),
    Callable(Callable),
}

impl<T: Clone + FromGodot + Default> Evaluator<T> {
    pub fn eval(&mut self) -> T {
        match self {
            Evaluator::Static(t) => t.clone(),
            Evaluator::Dynamic(f) => f(),
            Evaluator::Callable(call) => {
                let result = call.call(&[]);
                result.try_to::<T>().unwrap_or_else(|err| {
                    godot_error!("{err:?}");
                    T::default()
                })
            }
        }
    }
}

impl<T: GodotConvert> GodotConvert for Evaluator<T> {
    type Via = Variant;
}

impl<T: FromGodot> FromGodot for Evaluator<T> {
    fn try_from_godot(via: Self::Via) -> Result<Self, ConvertError> {
        if let Ok(call) = via.try_to::<Callable>() {
            Ok(Evaluator::Callable(call))
        } else if let Ok(t) = via.try_to::<T>() {
            Ok(Evaluator::Static(t))
        } else {
            Err(ConvertError::new("Cannot convert to Evaluator"))
        }
    }
}
