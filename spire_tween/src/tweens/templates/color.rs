use super::*;

macro_rules! color_templates {
    ($trait:ident, $class:ty, $getter:ident, $setter:ident, $property:ident, $blend:ident, $gradient:ident) => {
        /// Color convenience templates. Blendable color captures its baseline at
        /// construction and adds `target - baseline` over time. Gradient sampling
        /// includes alpha unless disabled; disabling it preserves current alpha on
        /// each write. Gradients are sampled live, including their interpolation
        /// mode/color space. Constructors return unregistered builders.
        pub trait $trait {
            fn $blend(&self, to: Color, duration: f64) -> SpireTween<LerpPropertyData<Color>>;
            fn $gradient(
                &self,
                gradient: Gd<Gradient>,
                duration: f64,
                include_alpha: bool,
            ) -> SpireTween<LerpMethodData<f64>>;
        }
        impl<C: Inherits<$class> + Inherits<Object>> $trait for Gd<C> {
            fn $blend(&self, to: Color, duration: f64) -> SpireTween<LerpPropertyData<Color>> {
                let node = self.clone().upcast::<$class>();
                self.$property(to, duration).as_relative(node.$getter())
            }
            fn $gradient(
                &self,
                mut gradient: Gd<Gradient>,
                duration: f64,
                include_alpha: bool,
            ) -> SpireTween<LerpMethodData<f64>> {
                let mut node = self.clone().upcast::<$class>();
                owned_progress(node.clone().upcast(), duration, move |p| {
                    let mut color = gradient.sample(p as f32);
                    if !include_alpha {
                        color.a = node.$getter().a;
                    }
                    node.$setter(color);
                })
            }
        }
    };
}

color_templates!(
    DoCanvasItemColors,
    CanvasItem,
    get_modulate,
    set_modulate,
    do_modulate,
    do_blendable_modulate,
    do_gradient_modulate
);
color_templates!(
    DoSpriteBase3DColors,
    SpriteBase3D,
    get_modulate,
    set_modulate,
    do_modulate,
    do_blendable_modulate,
    do_gradient_modulate
);
color_templates!(
    DoLabel3DColors,
    Label3D,
    get_modulate,
    set_modulate,
    do_modulate,
    do_blendable_modulate,
    do_gradient_modulate
);
color_templates!(
    DoLight3DColors,
    Light3D,
    get_color,
    set_color,
    do_light_color,
    do_blendable_light_color,
    do_gradient_light_color
);
color_templates!(
    DoMaterialColors,
    BaseMaterial3D,
    get_albedo,
    set_albedo,
    do_albedo_color,
    do_blendable_albedo_color,
    do_gradient_albedo_color
);

/// Animates Line2D's gradient endpoints while preserving interior stops and
/// interpolation settings. A private copy is assigned on first processing, so
/// shared gradients remain unchanged. A missing gradient starts from default_color.
/// The copy remains assigned on completion/cancellation. Returns an unregistered
/// progress tween; supports Restart/Yoyo, not Incremental.
pub trait DoLineColors {
    fn do_endpoint_colors(&self, start: Color, end: Color, duration: f64) -> SpireTween<LerpMethodData<f64>>;
}
impl<C: Inherits<Line2D> + Inherits<Object>> DoLineColors for Gd<C> {
    fn do_endpoint_colors(&self, start: Color, end: Color, duration: f64) -> SpireTween<LerpMethodData<f64>> {
        let mut line = self.clone().upcast::<Line2D>();
        let mut state: Option<(Gd<Gradient>, Color, Color)> = None;
        owned_progress(line.clone().upcast(), duration, move |p| {
            let (gradient, from_start, from_end) = state.get_or_insert_with(|| {
                let mut copy = Gradient::new_gd();
                if let Some(source) = line.get_gradient() {
                    copy.set_offsets(&source.get_offsets());
                    copy.set_colors(&source.get_colors());
                    copy.set_interpolation_mode(source.get_interpolation_mode());
                    copy.set_interpolation_color_space(source.get_interpolation_color_space());
                }
                if line.get_gradient().is_none() || copy.get_point_count() < 2 {
                    let color = if line.get_gradient().is_some() && copy.get_point_count() == 1 {
                        copy.get_color(0)
                    } else {
                        line.get_default_color()
                    };
                    copy.set_offsets(&PackedFloat32Array::from(&[0.0, 1.0]));
                    copy.set_colors(&PackedColorArray::from(&[color; 2]));
                }
                let first = copy.get_color(0);
                let last = copy.get_color(copy.get_point_count() - 1);
                line.set_gradient(&copy);
                (copy, first, last)
            });
            gradient.set_color(0, from_start.lerp(start, p));
            let last_index = gradient.get_point_count() - 1;
            gradient.set_color(last_index, from_end.lerp(end, p));
        })
    }
}
