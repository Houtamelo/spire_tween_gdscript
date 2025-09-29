use std::{fmt::Formatter, rc::Rc};

use keyframe::EasingFunction;

use crate::internal_prelude::*;

#[derive(Clone, Copy, Debug, GodotConvert)]
#[godot(via = i64)]
pub enum BasicEase {
    In   = 0,
    InCubic = 1,
    InOut = 2,
    InOutCubic = 3,
    InOutQuad = 4,
    InOutQuart = 5,
    InOutQuint = 6,
    InQuad = 7,
    InQuart = 8,
    InQuint = 9,
    Out  = 10,
    OutCubic = 11,
    OutQuad = 12,
    OutQuart = 13,
    OutQuint = 14,
    Hold = 15,
    Linear = 16,
    Step = 17,
}

#[derive(Clone)]
pub enum Ease {
    Basic(BasicEase),
    GodotCurve(Gd<Curve>),
    Callable(Callable),
    Custom(Rc<dyn EasingFunction>),
}

impl GodotConvert for Ease {
    type Via = Variant;
}

impl FromGodot for Ease {
    fn try_from_godot(via: Self::Via) -> Result<Self, ConvertError> {
        if let Ok(curve) = via.clone().try_to::<Gd<Curve>>() {
            Ok(Ease::GodotCurve(curve))
        } else if let Ok(callable) = via.clone().try_to::<Callable>() {
            Ok(Ease::Callable(callable))
        } else if let Ok(basic) = via.try_to::<BasicEase>() {
            Ok(Ease::Basic(basic))
        } else {
            Err(ConvertError::with_error_value("Could not convert Variant to `Ease`", via))
        }
    }
}

impl ToGodot for Ease {
    type ToVia<'v> = Self::Via;

    fn to_godot(&self) -> Self::ToVia<'_> {
        match self {
            Ease::Basic(basic) => basic.to_variant(),
            Ease::GodotCurve(curve) => curve.to_variant(),
            Ease::Callable(callable) => callable.to_variant(),
            Ease::Custom(f) => {
                let f = Rc::clone(f);
                Callable::from_local_fn("anonymous_easing_fn", move |args| {
                    let x = args.first().ok_or(())?;
                    let x = x.try_to::<f64>().map_err(|_| ())?;
                    Ok(f.y(x).to_variant())
                })
                .to_variant()
            }
        }
    }
}

impl Default for Ease {
    fn default() -> Self { Self::Basic(BasicEase::Linear) }
}

impl Ease {
    pub fn sample(&self, x: f64) -> f64 {
        match self {
            Ease::Basic(basic) => {
                match basic {
                    BasicEase::In => keyframe::functions::EaseIn.y(x),
                    BasicEase::InCubic => keyframe::functions::EaseInCubic.y(x),
                    BasicEase::InOut => keyframe::functions::EaseInOut.y(x),
                    BasicEase::InOutCubic => keyframe::functions::EaseInOutCubic.y(x),
                    BasicEase::InOutQuad => keyframe::functions::EaseInOutQuad.y(x),
                    BasicEase::InOutQuart => keyframe::functions::EaseInOutQuart.y(x),
                    BasicEase::InOutQuint => keyframe::functions::EaseInOutQuint.y(x),
                    BasicEase::InQuad => keyframe::functions::EaseInQuad.y(x),
                    BasicEase::InQuart => keyframe::functions::EaseInQuart.y(x),
                    BasicEase::InQuint => keyframe::functions::EaseInQuint.y(x),
                    BasicEase::Out => keyframe::functions::EaseOut.y(x),
                    BasicEase::OutCubic => keyframe::functions::EaseOutCubic.y(x),
                    BasicEase::OutQuad => keyframe::functions::EaseOutQuad.y(x),
                    BasicEase::OutQuart => keyframe::functions::EaseOutQuart.y(x),
                    BasicEase::OutQuint => keyframe::functions::EaseOutQuint.y(x),
                    BasicEase::Hold => keyframe::functions::Hold.y(x),
                    BasicEase::Linear => keyframe::functions::Linear.y(x),
                    BasicEase::Step => keyframe::functions::Step.y(x),
                }
            }
            Ease::GodotCurve(curve) => curve.sample(x as f32) as f64,
            Ease::Custom(custom) => custom.y(x),
            Ease::Callable(callable) => {
                let result = callable.call(&[x.to_variant()]);
                match result.try_to::<f64>() {
                    Ok(variant) => variant,
                    Err(err) => {
                        godot_error!("Error calling easing function: {err:?}");
                        x
                    }
                }
            }
        }
    }
}

impl Debug for Ease {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Ease::Basic(basic) => {
                match basic {
                    BasicEase::In => f.write_str("In"),
                    BasicEase::InCubic => f.write_str("InCubic"),
                    BasicEase::InOut => f.write_str("InOut"),
                    BasicEase::InOutCubic => f.write_str("InOutCubic"),
                    BasicEase::InOutQuad => f.write_str("InOutQuad"),
                    BasicEase::InOutQuart => f.write_str("InOutQuart"),
                    BasicEase::InOutQuint => f.write_str("InOutQuint"),
                    BasicEase::InQuad => f.write_str("InQuad"),
                    BasicEase::InQuart => f.write_str("InQuart"),
                    BasicEase::InQuint => f.write_str("InQuint"),
                    BasicEase::Out => f.write_str("Out"),
                    BasicEase::OutCubic => f.write_str("OutCubic"),
                    BasicEase::OutQuad => f.write_str("OutQuad"),
                    BasicEase::OutQuart => f.write_str("OutQuart"),
                    BasicEase::OutQuint => f.write_str("OutQuint"),
                    BasicEase::Hold => f.write_str("Hold"),
                    BasicEase::Linear => f.write_str("Linear"),
                    BasicEase::Step => f.write_str("Step"),
                }
            }
            Ease::GodotCurve(curve) => f.debug_tuple("GodotCurve").field(curve).finish(),
            Ease::Custom(_) => {
                f.debug_tuple("Custom")
                    .field(&"..anonymous dyn object..")
                    .finish()
            }
            Ease::Callable(callable) => f.debug_tuple("Callable").field(callable).finish(),
        }
    }
}
