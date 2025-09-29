macro_rules! impl_from_enum {
	($outer_path: path {
		$($tween_ty: ty => $enum_var: path),+
		$(,)?
	}) => {
        $(
            impl FromEnum<AnyTween> for SpireTween<$tween_ty> {
                fn from_enum(tween: AnyTween) -> Result<Self, AnyTween> {
                    if let $outer_path($enum_var(t)) = tween {
                        Ok(t)
                    } else {
                        Err(tween)
                    }
                }

            }

            impl FromEnumRef<AnyTween> for SpireTween<$tween_ty> {
                fn from_enum_ref(tween: &AnyTween) -> Option<&Self> {
                    if let $outer_path($enum_var(t)) = tween {
                        Some(t)
                    } else {
                        None
                    }
                }
            }

            impl FromEnumMut<AnyTween> for SpireTween<$tween_ty> {
                fn from_enum_mut(tween: &mut AnyTween) -> Option<&mut Self> {
                    if let $outer_path($enum_var(t)) = tween {
                        Some(t)
                    } else {
                        None
                    }
                }
            }

            impl From<SpireTween<$tween_ty>> for AnyTween {
                fn from(tween: SpireTween<$tween_ty>) -> Self {
                    $outer_path($enum_var(tween))
                }
            }
        )+
    };
}

pub(crate) use impl_from_enum;

macro_rules! register_gd_handle {
    ($Tween:expr, $Handle:expr) => {{
        let handle = $Handle;
        TWEENS.register_with_id(gd_handle_id!(handle), $Tween.into());
        handle
    }};
}

pub(crate) use register_gd_handle;

macro_rules! define_base_methods {
    ($Ty:ty) => {
        #[delegate_impl]
        impl $Ty {
            pub(crate) fn bound_nodes_mut(&mut self) -> &mut SmolSet<[Gd<Node>; 1]>;

            pub fn get_bound_nodes(&self) -> SmolSetIter<[Gd<Node>; 1]>;
            pub fn clear_bound_nodes(&mut self);

            pub fn get_delay(&self) -> f64;
            pub fn set_delay(&mut self, delay: f64);

            pub fn get_speed_scale(&self) -> f64;
            pub fn set_speed_scale(&mut self, speed_scale: f64);

            pub fn get_elapsed_time(&self) -> f64;
            pub fn get_cycles_elapsed(&self) -> u32;

            pub fn get_pause_mode(&self) -> SpirePauseMode;
            pub fn set_pause_mode(&mut self, pause_mode: SpirePauseMode);

            pub fn get_process_mode(&self) -> SpireProcessMode;
            pub fn set_process_mode(&mut self, process_mode: SpireProcessMode);

            pub fn get_loop_mode(&self) -> LoopMode;
            pub fn set_loop_mode(&mut self, loop_mode: LoopMode);

            pub fn finished_connect(&mut self, f: impl FnMut() + 'static);
            pub fn finished_connect_callable(&mut self, callable: Callable);
            pub fn finished_clear_connections(&mut self);
        }
    };
}

pub(crate) use define_base_methods;

macro_rules! define_base_gd_methods {
    ($Ty:ty) => {
        #[godot_api(secondary)]
        impl $Ty {
            #[func(gd_self)]
            pub fn get_state(this: Gd<Self>) -> i64 {
                tween_any_ref(gd_handle_id!(this), |tween| tween.get_state() as i64).unwrap_or(-1)
            }

            #[func(gd_self)]
            pub fn set_state(this: Gd<Self>, state: TweenState) {
                tween_any_mut(gd_handle_id!(this), |tween| tween.set_state(state)).log_if_err();
            }

            #[func(gd_self)]
            pub fn get_bound_nodes(this: Gd<Self>) -> Array<Gd<Node>> {
                tween_any_ref(gd_handle_id!(this), |tween| {
                    tween.get_bound_nodes().cloned().collect()
                })
                .log_if_err()
                .unwrap_or_default()
            }

            #[func(gd_self)]
            pub fn clear_bound_nodes(this: Gd<Self>) {
                tween_any_mut(gd_handle_id!(this), |tween| {
                    tween.clear_bound_nodes();
                })
                .log_if_err();
            }

            #[func(gd_self)]
            pub fn bind_node(this: Gd<Self>, obj: Gd<Node>) {
                TWEENS.bind_bothways(obj, gd_handle_id!(this));
            }

            #[func(gd_self)]
            pub fn unbind_node(this: Gd<Self>, obj: Gd<Node>) {
                TWEENS.node_unbind(obj, gd_handle_id!(this));
            }

            #[func(gd_self)]
            pub fn get_delay(this: Gd<Self>) -> f64 {
                tween_any_ref(gd_handle_id!(this), |tween| tween.get_delay())
                    .log_if_err()
                    .unwrap_or(0.)
            }

            #[func(gd_self)]
            pub fn set_delay(this: Gd<Self>, delay: f64) {
                tween_any_mut(gd_handle_id!(this), |tween| tween.set_delay(delay)).log_if_err();
            }

            #[func(gd_self)]
            pub fn get_speed_scale(this: Gd<Self>) -> f64 {
                tween_any_ref(gd_handle_id!(this), |tween| tween.get_speed_scale())
                    .log_if_err()
                    .unwrap_or(1.)
            }

            #[func(gd_self)]
            pub fn set_speed_scale(this: Gd<Self>, speed_scale: f64) {
                tween_any_mut(gd_handle_id!(this), |tween| tween.set_speed_scale(speed_scale))
                    .log_if_err();
            }

            #[func(gd_self)]
            pub fn get_elapsed_time(this: Gd<Self>) -> f64 {
                tween_any_ref(gd_handle_id!(this), |tween| tween.get_elapsed_time())
                    .log_if_err()
                    .unwrap_or(0.)
            }

            #[func(gd_self)]
            pub fn get_cycles_elapsed(this: Gd<Self>) -> i64 {
                tween_any_ref(gd_handle_id!(this), |tween| tween.get_cycles_elapsed() as i64)
                    .log_if_err()
                    .unwrap_or(0)
            }

            #[func(gd_self)]
            pub fn get_pause_mode(this: Gd<Self>) -> SpirePauseMode {
                tween_any_ref(gd_handle_id!(this), |tween| tween.get_pause_mode())
                    .log_if_err()
                    .unwrap_or_default()
            }

            #[func(gd_self)]
            pub fn set_pause_mode(this: Gd<Self>, pause_mode: SpirePauseMode) {
                tween_any_mut(gd_handle_id!(this), |tween| tween.set_pause_mode(pause_mode))
                    .log_if_err();
            }

            #[func(gd_self)]
            pub fn get_process_mode(this: Gd<Self>) -> SpireProcessMode {
                tween_any_ref(gd_handle_id!(this), |tween| tween.get_process_mode())
                    .log_if_err()
                    .unwrap_or_default()
            }

            /// Returns the number of loops remaining, or -1 if infinite.
            #[func(gd_self)]
            pub fn get_loops(this: Gd<Self>) -> i64 {
                tween_any_ref(gd_handle_id!(this), |tween| {
                    match tween.get_loop_mode() {
                        LoopMode::Finite(cycles_remaining) => cycles_remaining as i64,
                        LoopMode::Infinite => -1,
                    }
                })
                .log_if_err()
                .unwrap_or_default()
            }

            /// Sets the number of loops remaining. Use -1 for infinite loops.
            #[func(gd_self)]
            pub fn set_loops(this: Gd<Self>, loops: i64) {
                let loop_mode = if loops < 0 {
                    LoopMode::Infinite
                } else {
                    LoopMode::Finite(loops as u32)
                };

                tween_any_mut(gd_handle_id!(this), |tween| tween.set_loop_mode(loop_mode))
                    .log_if_err();
            }

            #[func(gd_self)]
            pub fn set_process_mode(this: Gd<Self>, process_mode: SpireProcessMode) {
                tween_any_mut(gd_handle_id!(this), |tween| tween.set_process_mode(process_mode))
                    .log_if_err();
            }

            #[func(gd_self)]
            pub fn is_playing(this: Gd<Self>) -> bool {
                tween_any_ref(gd_handle_id!(this), |tween| tween.is_playing())
                    .log_if_err()
                    .unwrap_or(false)
            }

            #[func(gd_self)]
            pub fn play(this: Gd<Self>) -> bool {
                tween_any_mut(gd_handle_id!(this), |tween| tween.play())
                    .log_if_err()
                    .is_some()
            }

            #[func(gd_self)]
            pub fn is_paused(this: Gd<Self>) -> bool {
                tween_any_ref(gd_handle_id!(this), |tween| tween.is_paused())
                    .log_if_err()
                    .unwrap_or(false)
            }

            #[func(gd_self)]
            pub fn pause(this: Gd<Self>) -> bool {
                tween_any_mut(gd_handle_id!(this), |tween| tween.pause())
                    .log_if_err()
                    .is_some()
            }

            #[func(gd_self)]
            pub fn is_stopped(this: Gd<Self>) -> bool {
                tween_any_ref(gd_handle_id!(this), |tween| tween.is_stopped())
                    .log_if_err()
                    .unwrap_or(true)
            }

            #[func(gd_self)]
            pub fn stop(this: Gd<Self>) -> bool {
                tween_any_mut(gd_handle_id!(this), |tween| tween.stop())
                    .log_if_err()
                    .is_some()
            }

            #[func(gd_self)]
            pub fn finished_connect(this: Gd<Self>, call: Callable) {
                tween_any_mut(gd_handle_id!(this), |tween| {
                    tween.finished_connect_callable(call);
                })
                .log_if_err();
            }

            #[func(gd_self)]
            pub fn finished_clear_connections(this: Gd<Self>) {
                tween_any_mut(gd_handle_id!(this), |tween| {
                    tween.finished_clear_connections();
                })
                .log_if_err();
            }

            #[func(gd_self)]
            pub fn advance_time(this: Gd<Self>, delta_time: f64) -> f64 {
                tween_any_mut(gd_handle_id!(this), |tween| tween.advance_time(delta_time))
                    .log_if_err()
                    .unwrap_or(-1.0)
            }

            #[func(gd_self)]
            pub fn complete(this: Gd<Self>) {
                if let Some(mut tween) = TWEENS.remove(gd_handle_id!(this)) {
                    tween.complete();
                }
            }
        }
    };
}

pub(crate) use define_base_gd_methods;
