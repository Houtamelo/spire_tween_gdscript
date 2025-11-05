#![allow(non_snake_case)]

use std::{fmt::Display, sync::LazyLock};

use godot::sys::{GodotFfi, UtilityFunctionBind};

use super::*;

pub trait LogIfErr<T> {
    type Output;

    fn log_if_err(self) -> Option<Self::Output>;
}

impl<T, E: Debug> LogIfErr<T> for Result<T, E> {
    type Output = T;

    fn log_if_err(self) -> Option<T> {
        match self {
            Err(err) => {
                godot_error!("{err:?}");
                None
            }
            Ok(ok) => Some(ok),
        }
    }
}

pub trait LogNullArg {
    fn log_null_arg<F, D>(self, arg_name: F) -> Self
    where
        Self: Sized,
        F: FnOnce() -> D,
        D: Display;
}

impl<C: GodotClass> LogNullArg for Option<Gd<C>> {
    fn log_null_arg<F, D>(self, arg_name: F) -> Self
    where
        Self: Sized,
        F: FnOnce() -> D,
        D: Display,
    {
        if self.is_none() {
            godot_error!("Parameter `{}`: expected non-null value, got `null`.", arg_name());
        }

        self
    }
}

pub trait LogBadArg {
    type Output;

    fn log_bad_spire_arg<F, D>(self, arg_name: F) -> Option<Self::Output>
    where
        Self: Sized,
        F: FnOnce() -> D,
        D: Display;
}

impl LogBadArg for Option<Gd<RefCounted>> {
    type Output = AnyTween;

    fn log_bad_spire_arg<F, D>(self, arg_name: F) -> Option<Self::Output>
    where
        F: FnOnce() -> D,
        D: Display,
    {
        let Some(handle) = self
        else {
            godot_error!("Parameter `{}`: expected Spire type, got `null`.", arg_name());
            return None;
        };

        handle.log_bad_spire_arg(arg_name)
    }
}

impl LogBadArg for Gd<RefCounted> {
    type Output = AnyTween;

    fn log_bad_spire_arg<F, D>(self, arg_name: F) -> Option<Self::Output>
    where
        F: FnOnce() -> D,
        D: Display,
    {
        tween_from_gd_handle(self).log_bad_spire_arg(arg_name)
    }
}

impl LogBadArg for Result<AnyTween, Gd<RefCounted>> {
    type Output = AnyTween;

    fn log_bad_spire_arg<F, D>(self, arg_name: F) -> Option<AnyTween>
    where
        F: FnOnce() -> D,
        D: Display,
    {
        match self {
            Ok(tween) => Some(tween),
            Err(non_spire) => {
                godot_error!("Parameter `{}`: expected Spire type, got `{}`.", arg_name(), non_spire.get_class());
                None
            }
        }
    }
}

pub trait TryVarAt {
    #[allow(unused)]
    fn try_var_at<T: FromGodot>(&self, key: &str) -> Result<T, ConvertError>;
}

impl TryVarAt for Dictionary {
    fn try_var_at<T: FromGodot>(&self, key: &str) -> Result<T, ConvertError> {
        self.get(key.to_godot())
            .ok_or_else(|| ConvertError::new("Expected Dictionary to contain key `{key}`."))?
            .try_to_relaxed()
    }
}

static IS_INSTANCE_ID_VALID_FN: LazyLock<UtilityFunctionBind> =
    LazyLock::new(|| unsafe { godot::sys::utility_function_table().is_instance_id_valid });

pub fn is_instance_id_valid(id: i64) -> bool {
    type CallRet = bool;
    type CallParams = (i64,);
    let args = (id,);
    unsafe {
        let utility_fn = *IS_INSTANCE_ID_VALID_FN;
        Signature::<CallParams, CallRet>::out_utility_ptrcall(utility_fn, "is_instance_id_valid", args)
    }
}

#[repr(transparent)]
pub struct SafeGString(pub GString);

unsafe impl Send for SafeGString {}
unsafe impl Sync for SafeGString {}

macro_rules! extern_calls_with_default_param {
    (
        $Ty: ty,
        fn $MethodIdent: ident(
            ($($CallParamsBinds: ident,)*) : ($($CallParamsTys: ty,)*),
            $DefaultParamBind: ident: $DefaultParamTy: ty = $DefaultValue:expr $(,)?
        ) -> $RetTy: ty;
    ) => {{
        use ::godot::{
            builtin::{StringName, Variant},
            obj::GodotClass,
            register::private::method::ClassMethodInfo,
            sys,
        };

        type CallParams = ($($CallParamsTys,)* $DefaultParamTy,);
        type CallParamsNoDefault = ($($CallParamsTys,)*);
        type CallRet = $RetTy;

        let method_name = StringName::from(stringify!($MethodIdent));

        unsafe extern "C" fn varcall_fn(
            _method_data: *mut std::ffi::c_void,
            instance_ptr: ::godot::sys::GDExtensionClassInstancePtr,
            args_ptr: *const ::godot::sys::GDExtensionConstVariantPtr,
            arg_count: ::godot::sys::GDExtensionInt,
            ret: ::godot::sys::GDExtensionVariantPtr,
            err: *mut ::godot::sys::GDExtensionCallError,
        ) {
            let call_ctx = ::godot::meta::CallContext::func(stringify!($Ty), stringify!($MethodIdent));

            ::godot::private::handle_varcall_panic(&call_ctx, &mut *err, || {
                if let Ok(_) = ::godot::meta::error::CallError::check_arg_count(&call_ctx, arg_count as usize, CallParams::LEN) {
                    let ($($CallParamsBinds,)* $DefaultParamBind,) = unsafe { CallParams::from_varcall_args(args_ptr, &call_ctx)? };

                    let rust_result = unsafe {
                        let storage = ::godot::private::as_storage::<$Ty>(instance_ptr);
                        <$Ty>::$MethodIdent(
                            ::godot::private::Storage::get_gd(storage),
                            $($CallParamsBinds,)*
                            $DefaultParamBind,
                        )
                    };

                    unsafe { varcall_return::<CallRet>(rust_result, ret, err) };
                    Ok(())
                } else {
                    ::godot::meta::error::CallError::check_arg_count(&call_ctx, arg_count as usize, CallParamsNoDefault::LEN)?;

                    let ($($CallParamsBinds,)*) =
                        unsafe { CallParamsNoDefault::from_varcall_args(args_ptr, &call_ctx)? };

                    let $DefaultParamBind = $DefaultValue;

                    let rust_result = unsafe {
                        let storage = ::godot::private::as_storage::<$Ty>(instance_ptr);
                        <$Ty>::$MethodIdent(
                            ::godot::private::Storage::get_gd(storage),
                            $($CallParamsBinds,)*
                            $DefaultParamBind,
                        )
                    };

                    unsafe { varcall_return::<CallRet>(rust_result, ret, err) };
                    Ok(())
                }
            });
        };

        unsafe extern "C" fn ptrcall_fn(
            _method_data: *mut std::ffi::c_void,
            instance_ptr: godot::sys::GDExtensionClassInstancePtr,
            args_ptr: *const godot::sys::GDExtensionConstTypePtr,
            ret: godot::sys::GDExtensionTypePtr,
        ) {
            let call_ctx = ::godot::meta::CallContext::func(stringify!($Ty), stringify!($MethodIdent));
            let _success = ::godot::private::handle_panic(
                || format!("{call_ctx}"),
                || {
                    let ($($CallParamsBinds,)* $DefaultParamBind,) =
                        unsafe { CallParams::from_ptrcall_args(args_ptr, godot::sys::PtrcallType::Standard, &call_ctx) };

                    let rust_result = {
                        let storage = unsafe { ::godot::private::as_storage::<$Ty>(instance_ptr) };
                        <$Ty>::$MethodIdent(
                            ::godot::private::Storage::get_gd(storage),
                            $($CallParamsBinds,)* $DefaultParamBind,
                        )
                    };

                    unsafe { ptrcall_return::<CallRet>(rust_result, ret, &call_ctx, godot::sys::PtrcallType::Standard) }
                },
            );
        };
        let mut method_info = unsafe {
            ClassMethodInfo::from_signature::<$Ty, CallParams, CallRet>(
                method_name,
                Some(varcall_fn),
                Some(ptrcall_fn),
                ::godot::global::MethodFlags::NORMAL,
                &[$(stringify!($CallParamsBinds),)* stringify!($DefaultParamBind)],
            )
        };
        method_info.default_arguments.push(($DefaultValue).to_variant());
        ::godot::private::out!(
            "   Register fn:   {}::{}",
            stringify!($Ty),
            stringify!($MethodIdent)
        );
        method_info.register_extension_class_method();
    }};
}

pub(crate) use extern_calls_with_default_param;

#[doc(hidden)]
pub(crate) unsafe fn varcall_return<R: ToGodot>(
    ret_val: R,
    ret: godot::sys::GDExtensionVariantPtr,
    err: *mut godot::sys::GDExtensionCallError,
) {
    let ret_variant = ret_val.to_variant();
    *(ret as *mut Variant) = ret_variant;
    (*err).error = godot::sys::GDEXTENSION_CALL_OK;
}

#[doc(hidden)]
pub(crate) unsafe fn ptrcall_return<R: ToGodot>(
    ret_val: R,
    ret: godot::sys::GDExtensionTypePtr,
    _call_ctx: &godot::meta::CallContext,
    call_type: godot::sys::PtrcallType,
) {
    let val = ret_val.to_godot();
    let ffi = val.into_ffi();

    ffi.move_return_ptr(ret, call_type);
}

#[macro_export]
macro_rules! print_every_nth_frame {
    ($fmt:literal $(, $args:expr)* $(,)?) => {{
        let frames = godot::classes::Engine::singleton().get_physics_frames();
        if frames % 60 == 0 {
            let str = format!($fmt $(, $args)*);
            let str_with_frames = format!("[{frames}]{str}");
            godot_print!("{str_with_frames}");
        }
    }};
}

#[macro_export]
macro_rules! print_with_frame {
    ($fmt:literal $(, $args:expr)* $(,)?) => {{
        let str = format!($fmt $(, $args)*);
        let str_with_frames = format!("[{}]{str}", godot::classes::Engine::singleton().get_physics_frames());
        godot_print!("{str_with_frames}");
    }};
}
