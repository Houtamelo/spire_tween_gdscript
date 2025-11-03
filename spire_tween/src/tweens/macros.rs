#[allow(unused)] // this is here for intellisense on the macros.
use super::*;

macro_rules! define_tween_ptr_methods {
    (Strong = $Strong:ty; Weak = $Weak:ty;) => {
        #[delegate_impl]
        impl Address for $Strong {
            #[inline]
            fn address(&self) -> *const ();
        }

        #[delegate_impl]
        impl $Strong {
            #[inline]
            pub fn downgrade(&self) -> $Weak;

            #[inline]
            pub fn strong_count(&self) -> usize;
        }

        impl PartialEq for $Strong {
            #[inline]
            fn eq(&self, other: &Self) -> bool {
                std::ptr::addr_eq(self.address(), other.address())
            }
        }

        impl PartialEq<$Weak> for $Strong {
            #[inline]
            fn eq(&self, other: &$Weak) -> bool {
                std::ptr::addr_eq(self.address(), other.address())
            }
        }

        impl Eq for $Strong {}

        impl<T> PartialEq<RcPtr<T>> for $Strong {
            #[inline]
            fn eq(&self, other: &RcPtr<T>) -> bool {
                std::ptr::addr_eq(self.address(), other.address())
            }
        }

        impl<T> PartialEq<$Strong> for RcPtr<T> {
            #[inline]
            fn eq(&self, other: &$Strong) -> bool {
                std::ptr::addr_eq(self.address(), other.address())
            }
        }

        impl<T> PartialEq<WeakPtr<T>> for $Strong {
            #[inline]
            fn eq(&self, other: &WeakPtr<T>) -> bool {
                std::ptr::addr_eq(self.address(), other.address())
            }
        }

        impl<T> PartialEq<$Strong> for WeakPtr<T> {
            #[inline]
            fn eq(&self, other: &$Strong) -> bool {
                std::ptr::addr_eq(self.address(), other.address())
            }
        }

        #[delegate_impl]
        impl Hash for $Strong {
            #[inline]
            fn hash<H: Hasher>(&self, state: &mut H);
        }

        impl Equivalent<*const ()> for $Strong {
            #[inline]
            fn equivalent(&self, key: &*const ()) -> bool {
                std::ptr::addr_eq(self.address(), *key)
            }
        }

        impl Equivalent<$Strong> for *const () {
            #[inline]
            fn equivalent(&self, key: &$Strong) -> bool { std::ptr::addr_eq(*self, key.address()) }
        }

        #[delegate_impl]
        impl Address for $Weak {
            #[inline]
            fn address(&self) -> *const ();
        }

        impl PartialEq for $Weak {
            #[inline]
            fn eq(&self, other: &Self) -> bool {
                std::ptr::addr_eq(self.address(), other.address())
            }
        }

        impl PartialEq<$Strong> for $Weak {
            #[inline]
            fn eq(&self, other: &$Strong) -> bool {
                std::ptr::addr_eq(self.address(), other.address())
            }
        }

        impl Eq for $Weak {}

        #[delegate_impl]
        impl Hash for $Weak {
            #[inline]
            fn hash<H: Hasher>(&self, state: &mut H);
        }

        impl From<&$Strong> for $Weak {
            #[inline]
            fn from(tween: &$Strong) -> Self { tween.downgrade() }
        }

        impl Equivalent<*const ()> for $Weak {
            #[inline]
            fn equivalent(&self, key: &*const ()) -> bool {
                std::ptr::addr_eq(self.address(), *key)
            }
        }

        impl Equivalent<$Weak> for *const () {
            #[inline]
            fn equivalent(&self, key: &$Weak) -> bool { std::ptr::addr_eq(*self, key.address()) }
        }

        impl<T> PartialEq<RcPtr<T>> for $Weak {
            #[inline]
            fn eq(&self, other: &RcPtr<T>) -> bool {
                std::ptr::addr_eq(self.address(), other.address())
            }
        }

        impl<T> PartialEq<$Weak> for RcPtr<T> {
            #[inline]
            fn eq(&self, other: &$Weak) -> bool {
                std::ptr::addr_eq(self.address(), other.address())
            }
        }

        impl<T> PartialEq<WeakPtr<T>> for $Weak {
            #[inline]
            fn eq(&self, other: &WeakPtr<T>) -> bool {
                std::ptr::addr_eq(self.address(), other.address())
            }
        }

        impl<T> PartialEq<$Weak> for WeakPtr<T> {
            #[inline]
            fn eq(&self, other: &$Weak) -> bool {
                std::ptr::addr_eq(self.address(), other.address())
            }
        }
    };
}

pub(crate) use define_tween_ptr_methods;

macro_rules! impl_from_enum {
	($outer_path: path {
		$($tween_ty: ty => $enum_var: path),+
		$(,)?
	}) => {
        $(
            impl FromEnum<AnyTween> for RcPtr<SpireTween<$tween_ty>> {
                fn from_enum(tween: AnyTween) -> Result<Self, AnyTween> {
                    if let $outer_path($enum_var(t)) = tween {
                        Ok(t)
                    } else {
                        Err(tween)
                    }
                }
            }

            impl FromEnumRef<AnyTween> for RcPtr<SpireTween<$tween_ty>> {
                fn from_enum_ref(tween: &AnyTween) -> Option<&Self> {
                    if let $outer_path($enum_var(t)) = tween {
                        Some(t)
                    } else {
                        None
                    }
                }
            }

            impl FromEnumMut<AnyTween> for RcPtr<SpireTween<$tween_ty>> {
                fn from_enum_mut(tween: &mut AnyTween) -> Option<&mut Self> {
                    if let $outer_path($enum_var(t)) = tween {
                        Some(t)
                    } else {
                        None
                    }
                }
            }

            impl From<RcPtr<SpireTween<$tween_ty>>> for AnyTween {
                fn from(tween: RcPtr<SpireTween<$tween_ty>>) -> Self {
                    $outer_path($enum_var(tween))
                }
            }
        )+
    };
}

pub(crate) use impl_from_enum;

macro_rules! impl_from_enum_weak {
	($outer_path: path {
		$($tween_ty: ty => $enum_var: path),+
		$(,)?
	}) => {
        $(
            impl FromEnum<WeakAnyTween> for WeakPtr<SpireTween<$tween_ty>> {
                fn from_enum(tween: WeakAnyTween) -> Result<Self, WeakAnyTween> {
                    if let $outer_path($enum_var(t)) = tween {
                        Ok(t)
                    } else {
                        Err(tween)
                    }
                }
            }

            impl FromEnumRef<WeakAnyTween> for WeakPtr<SpireTween<$tween_ty>> {
                fn from_enum_ref(tween: &WeakAnyTween) -> Option<&Self> {
                    if let $outer_path($enum_var(t)) = tween {
                        Some(t)
                    } else {
                        None
                    }
                }
            }

            impl FromEnumMut<WeakAnyTween> for WeakPtr<SpireTween<$tween_ty>> {
                fn from_enum_mut(tween: &mut WeakAnyTween) -> Option<&mut Self> {
                    if let $outer_path($enum_var(t)) = tween {
                        Some(t)
                    } else {
                        None
                    }
                }
            }

            impl From<WeakPtr<SpireTween<$tween_ty>>> for WeakAnyTween {
                fn from(tween: WeakPtr<SpireTween<$tween_ty>>) -> Self {
                    $outer_path($enum_var(tween))
                }
            }
        )+
    };
}

pub(crate) use impl_from_enum_weak;

macro_rules! define_base_methods {
    ($Ty:ty) => {
        #[delegate_impl]
        impl $Ty {
            #[inline]
            pub fn get_state(&self) -> State;
            #[inline]
            pub fn set_state(&mut self, state: State);

            #[inline]
            pub fn is_playing(&self) -> bool;
            #[inline]
            pub fn is_paused(&self) -> bool;
            #[inline]
            pub fn is_stopped(&self) -> bool;

            #[allow(unused)]
            #[inline]
            pub(crate) fn bound_nodes_mut(&mut self) -> &mut SmolSet<[Gd<Node>; 1]>;

            #[inline]
            pub fn get_bound_nodes(&self) -> &SmolSet<[Gd<Node>; 1]>;
            #[inline]
            pub fn clear_bound_nodes(&mut self);

            #[inline]
            pub fn get_delay(&self) -> f64;
            #[inline]
            pub fn set_delay(&mut self, delay: f64);

            #[inline]
            pub fn get_speed_scale(&self) -> f64;
            #[inline]
            pub fn set_speed_scale(&mut self, speed_scale: f64);

            #[inline]
            pub fn get_ignore_time_scale(&self) -> bool;
            #[inline]
            pub fn set_ignore_time_scale(&mut self, ignore_time_scale: bool);

            #[inline]
            pub fn get_pause_mode(&self) -> PauseMode;
            #[inline]
            pub fn set_pause_mode(&mut self, pause_mode: PauseMode);

            #[inline]
            pub fn get_process_mode(&self) -> ProcessMode;
            #[inline]
            pub fn set_process_mode(&mut self, process_mode: ProcessMode);

            #[inline]
            pub fn get_animation_position(&self) -> f64;
            #[inline]
            pub fn get_total_elapsed_time(&self) -> f64;
            #[inline]
            pub fn get_loops_finished(&self) -> i64;

            #[inline]
            pub fn get_loops(&self) -> i64;
            #[inline]
            pub fn set_loops(&mut self, loops: i64, loop_mode: LoopMode);

            #[inline]
            pub fn get_loop_mode(&self) -> LoopMode;
            #[inline]
            pub fn set_loop_mode(&mut self, loop_mode: LoopMode);

            /*
            #[inline]
            pub fn loop_finished_connect(&mut self, callable: Callable, flags: Flags);
            #[inline]
            pub fn loop_finished_disconnect(&mut self, callable: &Callable);
            #[inline]
            pub fn loop_finished_clear_connections(&mut self);

            #[inline]
            pub fn finished_connect(&mut self, callable: Callable, flags: Flags);
            #[inline]
            pub fn finished_disconnect(&mut self, callable: &Callable);
            #[inline]
            pub fn finished_clear_connections(&mut self);
            */

            #[allow(unused)]
            #[inline]
            pub(crate) fn handle_bound_nodes_validity(
                &mut self,
                is_tree_paused: bool,
            ) -> ObjectValidityResult;
        }
    };
}

pub(crate) use define_base_methods;

macro_rules! define_base_gd_methods {
    ($Ty:ident => $InnerTy:ty) => {
        #[godot_api(secondary)]
        impl $Ty {
            #[inline]
            pub(crate) fn to_ref(&self) -> &RcPtr<$InnerTy> { unsafe { & *self.inner.get() } }

            #[allow(clippy::wrong_self_convention)]
            #[allow(clippy::mut_from_ref)]
            #[inline]
            pub(crate) fn to_mut(&self) -> &mut RcPtr<$InnerTy> { unsafe { &mut *self.inner.get() } }

            /// Returns the current state of the tween. See [enum Spire.State] for details on each state.
            #[func]
            pub fn get_state(&self) -> State {
                self.to_ref().get_state()
            }

            /// Sets the current state of the tween.
            ///
            /// If [param state] is:
            /// - [constant Spire.STATE_PLAYING]: this is the same as calling [method play].
            /// - [constant Spire.STATE_PAUSED]: this is the same as calling [method pause].
            /// - [constant Spire.STATE_STOPPED]: this is the same as calling [method stop].
            #[func]
            pub fn set_state(&self, state: State) {
                self.to_mut().set_state(state);
            }

            /// Returns an array with all nodes this tween is currently bound to.
            /// See [method bind_node] for details on what binding to a node does.
            ///
            /// [b]Note:[/b] Property/Method tweeners are forcibly bound to the node they are animating,
            /// that node is not included in the array returned by this method.
            #[func]
            pub fn get_bound_nodes(&self) -> Array<Gd<Node>> {
                self.to_ref().get_bound_nodes().iter().cloned().collect()
            }

            /// Unbinds this tween from all nodes it is currently bound to.
            /// See [method bind_node] for details on what binding to a node does.
            ///
            /// [b]Note:[/b] Property/Method tweeners are forcibly bound to the node they are animating,
            /// this method won't unbind them from that node, there is no way of doing so.
            #[func]
            pub fn clear_bound_nodes(&self) {
                self.to_mut().clear_bound_nodes();
            }

            /// Binds this tween to the given node.
            /// Unlike the built-in [Tween], Spire tweens can be bound to any number of nodes.
            ///
            /// This influences the tween in several ways:
            /// - It will automatically be deleted when any of the bound nodes is freed.
            /// - If [enum Spire.PauseMode] is set to [constant Spire.PAUSE_MODE_BOUND],
            /// the tween will only process when all bound nodes are also processing
            /// (this is checked by calling [method Node.can_process] on each).
            ///
            /// [b]Note:[/b] Property/Method tweeners are automatically bound to the node they are animating,
            /// manually attempting to bind them won't do anything, and there is no harm in doing so.
            #[func(gd_self)]
            pub fn bind_node(this: Gd<Self>, obj: Option<Gd<Node>>) -> Gd<Self> {
                let Some(obj) = obj else {
                    godot_error!("Parameter `obj`: expected `Node`, got: `null`.");
                    return this;
                };

                let gd_ref = this.bind();
                let tween = gd_ref.to_mut();

                tween.bound_nodes_mut().insert(obj);
                TM.node_bind(obj, tween.downgrade());
                drop(gd_ref);
                this
            }

            /// Unbinds this tween from the given node.
            /// See [method bind_node] for details on what binding to a node does.
            ///
            /// Calling this method with a node that isn't bound to this tween is harmless.
            #[func]
            pub fn unbind_node(&self, obj: Option<Gd<Node>>) {
                let Some(obj) = obj else {
                    godot_error!("Parameter `obj`: expected `Node`, got: `null`.");
                    return;
                };

                let tween = self.to_mut();
                tween.bound_nodes_mut().remove(&obj);
                TM.node_unbind(obj, tween);
            }

            /// Returns the total delay (in seconds) that must elapse before the tween starts playing.
            /// For more details on how delay works, see [method set_delay].
            ///
            /// [b]Note:[/b] This returns the initial delay set with [method set_delay], not the remaining delay.
            /// The remaining delay can be trivially calculated with: [method get_delay] - [method get_total_elapsed_time].
            #[func]
            pub fn get_delay(&self) -> f64 {
                self.to_ref().get_delay()
            }

            /// Sets the delay (in seconds) before the tween starts playing.
            /// This effectively pauses the tween until the delay time has elapsed.
            ///
            /// Default value is 0.0 (no delay).
            ///
            /// [b]Note:[/b] Delay is affected by speed scale (see [method set_speed_scale]),
            /// a speed scale of 3.0 will make delay pass three times faster.
            ///
            /// [b]Note:[/b] If the tween is configured to loop (see [method set_loops]),
            /// delay only applies before the first loop, subsequent loops start immediately.
            ///
            /// [b]Note:[/b] `animation_position` (see [method get_animation_position])
            /// only starts increasing once the delay has elapsed.
            ///
            /// [b]Note:[/b] Delay does not elapse if:
            /// - The tween isn't processing (due to properties like [enum Spire.PauseMode]).
            /// - The tween is paused or stopped.
            #[func(gd_self)]
            pub fn set_delay(this: Gd<Self>, delay: f64) -> Gd<Self> {
                this.bind().to_mut().set_delay(delay);
                this
            }

            /// Returns the speed scale assigned to this tween.
            /// For more details on how speed scale works, see [method set_speed_scale].
            #[func]
            pub fn get_speed_scale(&self) -> f64 {
                self.to_ref().get_speed_scale()
            }

            /// Speed scale is a multiplier applied to `delta_time` when processing the tween.
            ///
            /// A value of 0.0 effectively pauses the tween, while a value of 2.0 makes it run twice as fast.
            ///
            /// Default value is 1.0.
            #[func(gd_self)]
            pub fn set_speed_scale(this: Gd<Self>, speed_scale: f64) -> Gd<Self> {
                this.bind().to_mut().set_speed_scale(speed_scale);
                this
            }

            /// Returns whether this tween ignores the global timescale (see [method set_ignore_time_scale]).
            #[func]
            pub fn get_ignore_time_scale(&self) -> bool {
                self.to_ref().get_ignore_time_scale()
            }

            /// Returns the [enum Spire.PauseMode] assigned to this tween.
            /// For more details on how each mode works, see [method set_pause_mode].
            #[func]
            pub fn get_pause_mode(&self) -> PauseMode {
                self.to_ref().get_pause_mode()
            }

            /// Determines the behavior of the tween when the [SceneTree] is paused.
            ///
            /// Works slight differently than the built-in [Tween]:
            /// - [constant Spire.PAUSE_MODE_BOUND]: The tween will only process when all its bound nodes
            /// are processing (this is checked with [method Node.can_process]). If there aren't any bound nodes,
            /// this falls back to [constant Spire.PAUSE_MODE_STOP].
            /// - [constant Spire.PAUSE_MODE_STOP]: The tween will only process when the SceneTree is not paused.
            /// - [constant Spire.PAUSE_MODE_PROCESS]: The tween will process regardless of the SceneTree's pause state.
            ///
            /// Default value is [constant Spire.PAUSE_MODE_BOUND].
            ///
            /// [b]Note:[/b] Almost all tweens created by Spire's `do_*` functions are automatically bound to the node they are animating.
            #[func(gd_self)]
            pub fn set_pause_mode(this: Gd<Self>, pause_mode: PauseMode) -> Gd<Self> {
                this.bind().to_mut().set_pause_mode(pause_mode);
                this
            }

            /// Returns the [enum Spire.ProcessMode] assigned to this tween.
            /// For more details on how each mode works, see [method set_process_mode].
            #[func]
            pub fn get_process_mode(&self) -> ProcessMode {
                self.to_ref().get_process_mode()
            }

            /// Determines whether the tween should update in [method Node._process],
            /// [method Node._physics_process], or not at all.
            ///
            /// Default value is [constant Spire.PROCESS_MODE_IDLE].
            ///
            /// [b]Note:[/b] Spire supports the additional mode [constant Spire.PROCESS_MODE_MANUAL],
            /// which makes the tween never process automatically, requiring manual advancement with [method custom_step].
            ///
            /// ### Behavior inside sequences
            /// The [enum Spire.ProcessMode] of tweens inside sequences is ignored. Only the root sequence's process mode matters.
            #[func(gd_self)]
            pub fn set_process_mode(this: Gd<Self>, process_mode: ProcessMode) -> Gd<Self> {
                this.bind().to_mut().set_process_mode(process_mode);
                this
            }

            /// Processes the tween by the given delta time, in seconds.
            /// This is mostly useful for manual control when the tween is paused.
            /// Setting delta longer than `duration` can be used to immediately apply the final "frame" of the animation
            /// (this may also be accomplished by calling [method force_complete]).
            ///
            /// If you don't want the tween to process automatically,
            /// set its [enum Spire.ProcessMode] to [constant Spire.PROCESS_MODE_MANUAL].
            /// This can be done with [method set_process_mode].
            ///
            /// [b]Note:[/b] This takes `speed_scale` (see [method set_speed_scale]) into account,
            /// so the effective delta time will be `delta_time * speed_scale`.
            ///
            /// [b]Note:[/b] This does not bypass the tween's [enum Spire.PauseMode].
            /// However, for the purposes of [constant Spire.PAUSE_MODE_STOP] the SceneTree is considered unpaused.
            /// This method won't do anything if the tween can't process
            /// (such as if pause mode is [constant Spire.PAUSE_MODE_BOUND] and any of the bound nodes can't process).
            /// If this behavior isn't desired, consider setting `pause_mode` to [constant Spire.PAUSE_MODE_PROCESS].
            #[func]
            pub fn custom_step(&self, delta_time: f64) {
                let _ = self.to_mut().process(delta_time, true);
            }

            /// [b]Returns:[/b] The current position of the tween's animation, in seconds. This is the time elapsed
            /// since the start of the current loop.
            ///
            /// [b]Note:[/b] This value does not include any initial delay set with [method set_delay].
            /// Which means that during the delay period, this will return 0.0.
            ///
            /// [b]Note:[/b] This value does not take easing into account.
            ///
            /// [b]Note:[/b] This value does not represent "real time", it is affected by factors like `speed_scale`
            /// (see [method set_speed_scale]), and it does not increase if the tween is paused or isn't processing
            /// due to [enum Spire.PauseMode] or [enum Spire.ProcessMode] configurations.
            #[func]
            pub fn get_animation_position(&self) -> f64 {
                self.to_ref().get_animation_position()
            }

            /// Works exactly like the built-in's [method Tween.get_total_elapsed_time].
            ///
            /// From the built-in's docs:
            /// "
            /// Returns the total time in seconds the Tween has been animating (i.e. the time since it started, not counting pauses etc.).
            /// The time is affected by [method set_speed_scale], and [method stop] will reset it to 0.
            /// "
            #[func]
            pub fn get_total_elapsed_time(&self) -> f64 {
                self.to_ref().get_total_elapsed_time()
            }

            /// [b]Returns:[b/] The number of loops configured for this tween (see [method set_loops]).
            /// A return value of -1 indicates an infinitely looping Tween.
            #[func]
            pub fn get_loops(&self) -> i64 {
                match self.to_ref().get_loops() {
                    0 => 1,
                    ..0 => -1,
                    max_loops => max_loops,
                }
            }

            /// [b]Returns:[b/] the [enum Spire.LoopMode] assigned to this tween (see [method set_loops]).
            /// The default value is [constant Spire.LOOP_MODE_RESTART].
            #[func]
            pub fn get_loop_mode(&self) -> LoopMode {
                self.to_ref().get_loop_mode()
            }

            /// Returns the number of completed loops since this tween began playing.
            ///
            /// This resets to 0 when the tween is stopped.
            #[func]
            pub fn get_loops_finished(&self) -> i64 {
                self.to_ref().get_loops_finished()
            }

            /// Returns the number of remaining loops for this tween (see [method set_loops]).
            /// A return value of -1 indicates an infinitely looping Tween.
            ///
            /// [b]Note:[/b] Unlike the built-in's [method Tween.get_loops_left], here a return value of 0 does **NOT** mean
            /// that the tween has already completed. To check if a tween has completed, use [method is_stopped].
            #[func]
            pub fn get_loops_left(&self) -> i64 {
                let tween = self.to_ref();
                match tween.get_loops() {
                    0 => 1,
                    ..0 => -1,
                    max_loops => max_loops - tween.get_loops_finished(),
                }
            }

            /*
            /// [b]Returns:[/b] `true` if the tween data is still in memory (hasn't been freed yet); `false` otherwise.
            ///
            /// [b]Usage:[/b] As long as the tween is valid, it can always be played/paused/stopped,
            /// or edited in any other way.
            ///
            /// [b]Note:[/b] This object is merely a handle to the actual "tween"
            /// (which is just a glob of data in "Rust" memory). This means that even if this handle's instance
            /// is valid (which can be checked by using [method Object.is_instance_valid]),
            /// the data it points to may have been removed already. That being said, this handle
            /// inherits [RefCounted], so [method Object.is_instance_valid]
            /// will always return `true` since the very reference you're using to call that method is keeping
            /// this handle alive.
            ///
            /// [b]Note:[/b] This method and [method Spire.is_valid] are the only ways of checking
            /// if the actual tween data still exists in memory.
            #[func(gd_self)]
            pub fn is_valid(this: Gd<Self>) -> bool { TM.is_valid(gd_handle_id!(this)) }

            /// [b]Behavior:[/b] Immediately removes the tween from memory, stopping it if necessary.
            ///
            /// [b]Note:[/b] Calling this on a tween that has already been killed does nothing, is harmless, and
            /// will not cause any warnings to be emitted. The same is true in case this handle is null.
            ///
            /// [b]Inside sequences:[/b] If this tween is inside a [SpireSequence], the sequence will automatically
            /// get rid of the tween on the next update.
            #[func(gd_self)]
            pub fn kill(this: Gd<Self>) { TM.remove(gd_handle_id!(this)); }
             */

            /// [b]Returns:[/b] `true` if this tween is currently registered with the TweenManager; `false` otherwise.
            ///
            /// [b]Usage:[/b] Being registered means that the tween will automatically "tick" at the end of
            /// each frame.
            ///
            /// [b]Inside sequences:[/b] This will return `false` if the tween is inside a [SpireSequence],
            /// since the Sequence is the one that "owns" the tween and is responsible for ticking it, not the
            /// TweenManager.
            #[func]
            pub fn is_registered(&self) -> bool {
                TM.tween_is_registered(self.to_ref())
            }

            /// [b]Behavior:[/b] Registers this tween with the TweenManager, which makes the tween tick each frame.
            ///
            /// [b]Note:[/b] Tweens automatically register themselves once created, this method is only useful
            /// if the tween was previously unregistered somehow.
            ///
            /// [b]Note:[/b] Calling this on a tween that's already registered does nothing, is harmless, and
            /// will not cause any warnings to be emitted.
            ///
            /// [b]Inside sequences:[/b] Registering a tween that's inside a [SpireSequence] results in
            /// undefined behavior, since that would make both the Sequence and the TweenManager to tick it.
            /// Undefined behavior here will not cause crashes or memory corruption, but the tween will most likely
            /// not behave as expected.
            #[func]
            pub fn register(&self) {
                TM.tween_register(self.to_ref().clone());
            }

            /// [b]Behavior:[/b] Unregisters this tween from the TweenManager, which means the tween
            /// will no longer tick each frame. Since tweens are ref-counted, this does not necessarily
            /// mean that the tween is immediately freed from memory, as there might be other references to it
            /// (such as the very reference you're using to call this method).
            ///
            /// [b]Note:[/b] Calling this on a tween that's already unregistered does nothing, is harmless, and
            /// will not cause any warnings to be emitted.
            ///
            /// [b]Inside sequences:[/b] A [SpireSequence] automatically unregisters any tweens added to it, which
            /// means that, if this tween is inside a sequence, it is already unregistered. If you want to remove
            /// a tween from a sequence, use [method SpireSequence.remove] instead.
            #[func]
            pub fn unregister(&self) {
                TM.tween_unregister(self.to_ref());
            }

            /// [b]Behavior:[/b] Resumes a paused or starts a stopped tween.
            ///
            /// [b]Note:[/b] Calling this on a tween that's already playing does nothing, and is harmless.
            ///
            /// [b]Note:[/b] By default, tweens will automatically play when created,
            /// calling this method is only necessary if the tween was paused or stopped.
            ///
            /// [b]Inside sequences:[/b] It is not recommended to call this method on tweens that are inside sequences,
            /// since the sequence controls their playback.
            #[func(gd_self)]
            pub fn play(this: Gd<Self>) -> Gd<Self> {
                this.bind().to_mut().play();
                this
            }

            /// [b]Returns:[/b] `true` if [method get_state] == [constant Spire.STATE_PLAYING]; `false` otherwise.
            #[func]
            pub fn is_playing(&self) -> bool {
                self.to_ref().is_playing()
            }

            /// [b]Behavior:[/b] Pauses a tween, which stops it from processing until resumed.
            ///
            /// [b]Note:[/b] Calling this on a paused tween does nothing.
            ///
            /// [b]Note:[/b] Calling this on a stopped tween does nothing.
            /// The state will remain as [constant Spire.STATE_STOPPED].
            ///
            /// [b]Inside sequences:[/b] It is not recommended to call this method on tweens inside sequences,
            /// since the sequence controls their playback.
            #[func(gd_self)]
            pub fn pause(this: Gd<Self>) -> Gd<Self> {
                this.bind().to_mut().pause();
                this
            }

            /// [b]Returns:[/b] `true` if [method get_state] == [constant Spire.STATE_PAUSED]; `false` otherwise.
            ///
            /// [b]Usage:[/b] A paused tween works like a node outside the SceneTree - it doesn't interact
            /// with the game in any way.
            #[func]
            pub fn is_paused(&self) -> bool {
                self.to_ref().is_paused()
            }

            /// [b]Behavior:[/b] Stops tweening and resets its internal timers and counters
            /// Notably, these are reset:
            /// - `total_elapsed_time`
            /// - `loop_time` (AKA `animation_position`)
            /// - `loop_counter` (AKA `loops_completed`)
            ///
            /// [b]Note:[/b] Connections established with the flag [constant Spire.FLAGS_CONNECT_ONE_SHOT] are NOT "restored".
            ///
            /// [b]Note:[/b] If this tween is a [SpireSequence], this method will also call `stop` on all the
            /// sequence's child tweens.
            ///
            /// [b]Inside sequences:[/b] Calling this on a tween that's a child of a [SpireSequence] will simply
            /// make the sequence skip it, the sequence will not stop.
            #[func(gd_self)]
            pub fn stop(this: Gd<Self>) -> Gd<Self> {
                this.bind().to_mut().stop();
                this
            }

            /// [b]Returns:[/b] `true` if [method get_state] == [constant Spire.STATE_STOPPED]; `false` otherwise.
            ///
            /// [b]Note:[/b] A tween automatically "stops" once it finishes playing, but you can forcibly stop
            /// it by calling [method stop].
            #[func]
            pub fn is_stopped(&self) -> bool {
                self.to_ref().is_stopped()
            }

            /// Forces a tween to finish immediately, applying its end values right away.
            ///
            /// - This does nothing if the tween is stopped.
            /// - This will emit the `finished` signal if the tween was paused/playing.
            /// - This ignores any remaining loops and will not emit `loop_finished` for them.
            ///
            /// After this is called, the tween's state will reset to [constant Spire.STATE_STOPPED].
            #[func]
            pub fn force_complete(&self) {
                self.to_mut().force_complete();
            }
        }

        impl $Ty {
            /// Works almost exactly like the built-in's [method Tween.set_ignore_time_scale].
            ///
            /// From the built-in's docs:
            /// "
            /// If [param ignore] is `true`, the tween will ignore [property Engine.time_scale] and update
            /// with the real, elapsed time. This affects all Tweeners and their delays. Default value is `false`.
            /// "
            ///
            /// [b]Note:[/b] In Spire, "Tweeners" are analogous to the children of a [SpireSequence].
            pub fn set_ignore_time_scale(this: Gd<Self>, ignore: bool) -> Gd<Self> {
                this.bind().to_mut().set_ignore_time_scale(ignore);
                this
            }


            /// Works slight different from the built-in's [method Tween.set_loops].
            ///
            /// Parameter [param loops]: The number of times the tweening animation will be repeated.
            /// - Any positive number will make the tween run that many times, i.e. `set_loops(1)` will make it run once (which is the default behavior).
            /// - Tweens always run at least once, so `set_loops(0)` is the same as `set_loops(1)`.
            /// - Any negative number (usually -1) will make the tween run infinitely.
            ///
            /// Parameter [param loop_mode]: The way the tween will behave when starting a new loop.
            /// - [constant Spire.LOOP_MODE_RESTART] : The tween will jump back to the beginning and play forward again.
            /// - [constant Spire.LOOP_MODE_YOYO] : The tween will reverse direction and play backward, then forward, then backward, and so on.
            /// - [constant Spire.LOOP_MODE_INCREMENTAL] : The tween will overshoot the end value, continuously incrementing
            ///   it based on the difference between the start and end values. E.g.: if animating from 0 to 10, the first loop will go from 0 to 10,
            ///   the second from 10 to 20, the third from 20 to 30, and so on.
            ///
            /// [b]Note:[/b] The built-in [Tween]'s loop system can loop infinitely in the same frame if the duration
            /// of a tween is 0. This is **NOT** the case with Spire, where tweens will only begin the next loop
            /// on the next update.
            ///
            /// For more details on loop modes and how they behave internally, see [enum Spire.LoopMode].
            pub fn set_loops(this: Gd<Self>, loops: i64, loop_mode: LoopMode) -> Gd<Self> {
                this.bind().to_mut().set_loops(loops, loop_mode);
                this
            }
        }

        ::godot::sys::plugin_execute_pre_main!({
            #[allow(unused)]
            $ {concat(__registration_methods_, $Ty)}.lock().unwrap().push(|| {
                extern_calls_with_default_param! { $Ty,
                    fn set_ignore_time_scale(
                        (): (),
                        ignore: bool = true,
                    ) -> Gd<$Ty>;
                }

                extern_calls_with_default_param! { $Ty,
                    fn set_loops(
                        (loops,): (i64,),
                        loop_mode: LoopMode = LoopMode::default(),
                    ) -> Gd<$Ty>;
                }
            });
        });

        #[allow(unused)]
        impl $ {concat(__godot_, $Ty, _Funcs)} {
            #[doc(hidden)]
            #[allow(non_upper_case_globals)]
            pub const set_ignore_time_scale: &str = "set_ignore_time_scale";

            #[doc(hidden)]
            #[allow(non_upper_case_globals)]
            pub const set_loops: &str = "set_loops";
        }
        ::godot::sys::plugin_add!(
            :: godot :: private :: __GODOT_DOCS_REGISTRY ;
            :: godot :: private :: DocsPlugin :: new :: < $Ty > (
                :: godot :: private :: DocsItem :: InherentImpl (
                    :: godot :: private :: InherentImplDocs {
                        methods : Box::leak(format!(
                        "<method name=\"set_ignore_time_scale\">\n  <return type=\"Gd &lt; {class} &gt;\" />\n  \
                        <param index=\"0\" name=\"ignore\" type=\"bool\" />\n  \
                        <description>\n  Works almost exactly like the built-in&#39;s [method Tween.set_ignore_time_scale].\
                        [br][br]From the built-in&#39;s docs:[br][br]\
                        &quot;If [param ignore] is `true`, the tween will ignore [property Engine.time_scale] \
                        and update with the real, elapsed time. This affects all Tweeners and their delays. \
                        Default value is `false`.&quot;[br][br][b]Note:[/b] In Spire, &quot;Tweeners&quot; \
                        are analogous to the children of a [SpireSequence].\n  </description>\n</method>\n\n\
                        <method name=\"set_loops\">\n  <return type=\"Gd &lt; {class} &gt;\" />\n  \
                        <param index=\"0\" name=\"loops\" type=\"i64\" /><param index=\"1\" name=\"loop_mode\" type=\"LoopMode\" />\n  \
                        <description>\n  Works slight different from the built-in&#39;s [method Tween.set_loops].\
                        [br][br]Parameter [param loops]: The number of times the tweening animation will be repeated.[br][br]• \
                        Any positive number will make the tween run that many times, i.e. [code]set_loops(1)[/code] will make it run once \
                        (which is the default behavior).[br]• Tweens always run at least once, so [code]set_loops(0)[/code] is the same as \
                        [code]set_loops(1)[/code].[br]• Any negative number (usually -1) will make the tween run infinitely.\
                        [br][br]Parameter [param loop_mode]: The way the tween will behave when starting a new loop.[br][br]• \
                        [constant Spire.LOOP_MODE_RESTART] : The tween will jump back to the beginning and play forward again.[br]• \
                        [constant Spire.LOOP_MODE_YOYO] : The tween will reverse direction and play backward, then forward, \
                        then backward, and so on.[br]• [constant Spire.LOOP_MODE_INCREMENTAL] : The tween will overshoot the end value, \
                        continuously incrementing it based on the difference between the start and end values. \
                        E.g.: if animating from 0 to 10, the first loop will go from 0 to 10, the second from 10 to 20, \
                        the third from 20 to 30, and so on.[br][br][b]Note:[/b] The built-in [Tween]&#39;s loop system can loop \
                        infinitely in the same frame if the duration of a tween is 0. This is [b]NOT[/b] the case with Spire, \
                        where tweens will only begin the next loop on the next update.[br][br]For more details on loop modes \
                        and how they behave internally, see [enum Spire.LoopMode].\n  \
                        </description>\n</method>\n", class = stringify!($Ty)
                        ).into_boxed_str()) ,
                        signals : "" ,
                        constants : ""
                    }
                )
            )
        );
    };
}

pub(crate) use define_base_gd_methods;
