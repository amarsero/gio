// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use InetAddress;
use InetSocketAddress;
use SocketAddress;
use SocketConnectable;
use ffi;
use glib;
use glib::object::Downcast;
use glib::object::IsA;
use glib::signal::SignalHandlerId;
use glib::signal::connect;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
use std::boxed::Box as Box_;
use std::mem;
use std::mem::transmute;
use std::ptr;

glib_wrapper! {
    pub struct ProxyAddress(Object<ffi::GProxyAddress, ffi::GProxyAddressClass>): InetSocketAddress, SocketAddress, SocketConnectable;

    match fn {
        get_type => || ffi::g_proxy_address_get_type(),
    }
}

impl ProxyAddress {
    pub fn new<'a, 'b, P: Into<Option<&'a str>>, Q: Into<Option<&'b str>>>(inetaddr: &InetAddress, port: u16, protocol: &str, dest_hostname: &str, dest_port: u16, username: P, password: Q) -> ProxyAddress {
        let username = username.into();
        let username = username.to_glib_none();
        let password = password.into();
        let password = password.to_glib_none();
        unsafe {
            SocketAddress::from_glib_full(ffi::g_proxy_address_new(inetaddr.to_glib_none().0, port, protocol.to_glib_none().0, dest_hostname.to_glib_none().0, dest_port, username.0, password.0)).downcast_unchecked()
        }
    }
}

unsafe impl Send for ProxyAddress {}
unsafe impl Sync for ProxyAddress {}

pub trait ProxyAddressExt {
    fn get_destination_hostname(&self) -> String;

    fn get_destination_port(&self) -> u16;

    #[cfg(any(feature = "v2_34", feature = "dox"))]
    fn get_destination_protocol(&self) -> Option<String>;

    fn get_password(&self) -> Option<String>;

    fn get_protocol(&self) -> String;

    #[cfg(any(feature = "v2_34", feature = "dox"))]
    fn get_uri(&self) -> Option<String>;

    fn get_username(&self) -> Option<String>;

    fn connect_property_destination_hostname_notify<F: Fn(&Self) + Send + Sync + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_destination_port_notify<F: Fn(&Self) + Send + Sync + 'static>(&self, f: F) -> SignalHandlerId;

    #[cfg(any(feature = "v2_34", feature = "dox"))]
    fn connect_property_destination_protocol_notify<F: Fn(&Self) + Send + Sync + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_password_notify<F: Fn(&Self) + Send + Sync + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_protocol_notify<F: Fn(&Self) + Send + Sync + 'static>(&self, f: F) -> SignalHandlerId;

    #[cfg(any(feature = "v2_34", feature = "dox"))]
    fn connect_property_uri_notify<F: Fn(&Self) + Send + Sync + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_username_notify<F: Fn(&Self) + Send + Sync + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<ProxyAddress> + IsA<glib::object::Object>> ProxyAddressExt for O {
    fn get_destination_hostname(&self) -> String {
        unsafe {
            from_glib_none(ffi::g_proxy_address_get_destination_hostname(self.to_glib_none().0))
        }
    }

    fn get_destination_port(&self) -> u16 {
        unsafe {
            ffi::g_proxy_address_get_destination_port(self.to_glib_none().0)
        }
    }

    #[cfg(any(feature = "v2_34", feature = "dox"))]
    fn get_destination_protocol(&self) -> Option<String> {
        unsafe {
            from_glib_none(ffi::g_proxy_address_get_destination_protocol(self.to_glib_none().0))
        }
    }

    fn get_password(&self) -> Option<String> {
        unsafe {
            from_glib_none(ffi::g_proxy_address_get_password(self.to_glib_none().0))
        }
    }

    fn get_protocol(&self) -> String {
        unsafe {
            from_glib_none(ffi::g_proxy_address_get_protocol(self.to_glib_none().0))
        }
    }

    #[cfg(any(feature = "v2_34", feature = "dox"))]
    fn get_uri(&self) -> Option<String> {
        unsafe {
            from_glib_none(ffi::g_proxy_address_get_uri(self.to_glib_none().0))
        }
    }

    fn get_username(&self) -> Option<String> {
        unsafe {
            from_glib_none(ffi::g_proxy_address_get_username(self.to_glib_none().0))
        }
    }

    fn connect_property_destination_hostname_notify<F: Fn(&Self) + Send + Sync + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + Send + Sync + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::destination-hostname",
                transmute(notify_destination_hostname_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_destination_port_notify<F: Fn(&Self) + Send + Sync + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + Send + Sync + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::destination-port",
                transmute(notify_destination_port_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    #[cfg(any(feature = "v2_34", feature = "dox"))]
    fn connect_property_destination_protocol_notify<F: Fn(&Self) + Send + Sync + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + Send + Sync + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::destination-protocol",
                transmute(notify_destination_protocol_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_password_notify<F: Fn(&Self) + Send + Sync + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + Send + Sync + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::password",
                transmute(notify_password_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_protocol_notify<F: Fn(&Self) + Send + Sync + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + Send + Sync + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::protocol",
                transmute(notify_protocol_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    #[cfg(any(feature = "v2_34", feature = "dox"))]
    fn connect_property_uri_notify<F: Fn(&Self) + Send + Sync + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + Send + Sync + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::uri",
                transmute(notify_uri_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_username_notify<F: Fn(&Self) + Send + Sync + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + Send + Sync + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::username",
                transmute(notify_username_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }
}

unsafe extern "C" fn notify_destination_hostname_trampoline<P>(this: *mut ffi::GProxyAddress, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<ProxyAddress> {
    let f: &&(Fn(&P) + Send + Sync + 'static) = transmute(f);
    f(&ProxyAddress::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_destination_port_trampoline<P>(this: *mut ffi::GProxyAddress, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<ProxyAddress> {
    let f: &&(Fn(&P) + Send + Sync + 'static) = transmute(f);
    f(&ProxyAddress::from_glib_borrow(this).downcast_unchecked())
}

#[cfg(any(feature = "v2_34", feature = "dox"))]
unsafe extern "C" fn notify_destination_protocol_trampoline<P>(this: *mut ffi::GProxyAddress, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<ProxyAddress> {
    let f: &&(Fn(&P) + Send + Sync + 'static) = transmute(f);
    f(&ProxyAddress::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_password_trampoline<P>(this: *mut ffi::GProxyAddress, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<ProxyAddress> {
    let f: &&(Fn(&P) + Send + Sync + 'static) = transmute(f);
    f(&ProxyAddress::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_protocol_trampoline<P>(this: *mut ffi::GProxyAddress, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<ProxyAddress> {
    let f: &&(Fn(&P) + Send + Sync + 'static) = transmute(f);
    f(&ProxyAddress::from_glib_borrow(this).downcast_unchecked())
}

#[cfg(any(feature = "v2_34", feature = "dox"))]
unsafe extern "C" fn notify_uri_trampoline<P>(this: *mut ffi::GProxyAddress, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<ProxyAddress> {
    let f: &&(Fn(&P) + Send + Sync + 'static) = transmute(f);
    f(&ProxyAddress::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_username_trampoline<P>(this: *mut ffi::GProxyAddress, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<ProxyAddress> {
    let f: &&(Fn(&P) + Send + Sync + 'static) = transmute(f);
    f(&ProxyAddress::from_glib_borrow(this).downcast_unchecked())
}
