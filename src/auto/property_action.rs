// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use Action;
use ffi;
#[cfg(any(feature = "v2_38", feature = "dox"))]
use glib;
#[cfg(any(feature = "v2_46", feature = "dox"))]
use glib::StaticType;
#[cfg(any(feature = "v2_46", feature = "dox"))]
use glib::Value;
use glib::object::IsA;
#[cfg(any(feature = "v2_38", feature = "dox"))]
use glib::object::ObjectType;
#[cfg(any(feature = "v2_38", feature = "dox"))]
use glib::signal::SignalHandlerId;
#[cfg(any(feature = "v2_38", feature = "dox"))]
use glib::signal::connect_raw;
use glib::translate::*;
#[cfg(any(feature = "v2_38", feature = "dox"))]
use glib_ffi;
#[cfg(any(feature = "v2_46", feature = "dox"))]
use gobject_ffi;
#[cfg(any(feature = "v2_38", feature = "dox"))]
use std::boxed::Box as Box_;
use std::fmt;
#[cfg(any(feature = "v2_38", feature = "dox"))]
use std::mem::transmute;

glib_wrapper! {
    pub struct PropertyAction(Object<ffi::GPropertyAction, PropertyActionClass>) @implements Action;

    match fn {
        get_type => || ffi::g_property_action_get_type(),
    }
}

impl PropertyAction {
    #[cfg(any(feature = "v2_38", feature = "dox"))]
    pub fn new<P: IsA<glib::Object>>(name: &str, object: &P, property_name: &str) -> PropertyAction {
        unsafe {
            from_glib_full(ffi::g_property_action_new(name.to_glib_none().0, object.as_ref().to_glib_none().0, property_name.to_glib_none().0))
        }
    }

    #[cfg(any(feature = "v2_46", feature = "dox"))]
    pub fn get_property_invert_boolean(&self) -> bool {
        unsafe {
            let mut value = Value::from_type(<bool as StaticType>::static_type());
            gobject_ffi::g_object_get_property(self.as_ptr() as *mut gobject_ffi::GObject, b"invert-boolean\0".as_ptr() as *const _, value.to_glib_none_mut().0);
            value.get().unwrap()
        }
    }

    #[cfg(any(feature = "v2_38", feature = "dox"))]
    pub fn connect_property_enabled_notify<F: Fn(&PropertyAction) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::enabled\0".as_ptr() as *const _,
                Some(transmute(notify_enabled_trampoline::<F> as usize)), Box_::into_raw(f))
        }
    }

    #[cfg(any(feature = "v2_38", feature = "dox"))]
    pub fn connect_property_parameter_type_notify<F: Fn(&PropertyAction) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::parameter-type\0".as_ptr() as *const _,
                Some(transmute(notify_parameter_type_trampoline::<F> as usize)), Box_::into_raw(f))
        }
    }

    #[cfg(any(feature = "v2_38", feature = "dox"))]
    pub fn connect_property_state_notify<F: Fn(&PropertyAction) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::state\0".as_ptr() as *const _,
                Some(transmute(notify_state_trampoline::<F> as usize)), Box_::into_raw(f))
        }
    }

    #[cfg(any(feature = "v2_38", feature = "dox"))]
    pub fn connect_property_state_type_notify<F: Fn(&PropertyAction) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::state-type\0".as_ptr() as *const _,
                Some(transmute(notify_state_type_trampoline::<F> as usize)), Box_::into_raw(f))
        }
    }
}

#[cfg(any(feature = "v2_38", feature = "dox"))]
unsafe extern "C" fn notify_enabled_trampoline<F: Fn(&PropertyAction) + 'static>(this: *mut ffi::GPropertyAction, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer) {
    let f: &F = transmute(f);
    f(&from_glib_borrow(this))
}

#[cfg(any(feature = "v2_38", feature = "dox"))]
unsafe extern "C" fn notify_parameter_type_trampoline<F: Fn(&PropertyAction) + 'static>(this: *mut ffi::GPropertyAction, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer) {
    let f: &F = transmute(f);
    f(&from_glib_borrow(this))
}

#[cfg(any(feature = "v2_38", feature = "dox"))]
unsafe extern "C" fn notify_state_trampoline<F: Fn(&PropertyAction) + 'static>(this: *mut ffi::GPropertyAction, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer) {
    let f: &F = transmute(f);
    f(&from_glib_borrow(this))
}

#[cfg(any(feature = "v2_38", feature = "dox"))]
unsafe extern "C" fn notify_state_type_trampoline<F: Fn(&PropertyAction) + 'static>(this: *mut ffi::GPropertyAction, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer) {
    let f: &F = transmute(f);
    f(&from_glib_borrow(this))
}

impl fmt::Display for PropertyAction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "PropertyAction")
    }
}
