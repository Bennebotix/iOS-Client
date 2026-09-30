use std::ffi::CStr;
use std::os::raw::{c_char, c_void};
use std::sync::atomic::{AtomicBool, Ordering};

pub struct PommeInstance {
    pub is_running: AtomicBool,
    pub layer_ptr: *mut c_void,
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn pomme_init(metal_layer: *mut c_void) -> *mut PommeInstance {
    if metal_layer.is_null() {
        return std::ptr::null_mut();
    }

    let instance = Box::new(PommeInstance {
        is_running: AtomicBool::new(true),
        layer_ptr: metal_layer,
    });

    Box::into_raw(instance)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn pomme_connect_server(
    instance: *mut PommeInstance,
    address: *const c_char,
    _port: u16,
) -> bool {
    if instance.is_null() || address.is_null() {
        return false;
    }

    let c_str = unsafe { CStr::from_ptr(address) };
    let _addr_str = match c_str.to_str() {
        Ok(s) => s,
        Err(_) => return false,
    };

    true
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn pomme_render_frame(instance: *mut PommeInstance) {
    if instance.is_null() {
        return;
    }
    let instance = unsafe { &*instance };
    if !instance.is_running.load(Ordering::Relaxed) {
        return;
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn pomme_destroy(instance: *mut PommeInstance) {
    if instance.is_null() {
        return;
    }
    let instance = unsafe { Box::from_raw(instance) };
    instance.is_running.store(false, Ordering::Relaxed);
}
