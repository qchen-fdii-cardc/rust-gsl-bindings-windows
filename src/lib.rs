pub mod bindings {
    #![allow(non_snake_case)]
    #![allow(non_camel_case_types)]
    #![allow(non_upper_case_globals)]
    #![allow(dead_code)]
    #![allow(clippy::all)]

    include!("gsl_bindings.rs");
}

pub use bindings as gsl;

pub fn c_array_to_str<const N: usize>(bytes: &[u8; N]) -> std::borrow::Cow<'_, str> {
    let end = bytes.iter().position(|&b| b == 0).unwrap_or(N);
    String::from_utf8_lossy(&bytes[..end])
}

pub fn c_ptr_to_str(ptr: *const std::ffi::c_char) -> std::borrow::Cow<'static, str> {
    if ptr.is_null() {
        return std::borrow::Cow::Borrowed("<null>");
    }
    unsafe {
        std::ffi::CStr::from_ptr(ptr)
            .to_string_lossy()
            .into_owned()
            .into()
    }
}

#[macro_export]
macro_rules! cstr {
    ($bytes:expr) => {{ $crate::c_ptr_to_str($bytes) }};
}

#[macro_export]
macro_rules! gsl_cstr {
    ($bytes:expr) => {{ $crate::c_array_to_str($bytes) }};
}

#[macro_export]
macro_rules! gsl_call {
    ($fn:ident ( $($arg:expr),* $(,)? )) => {{
        unsafe { $crate::gsl::$fn($($arg),*) }
    }};
}
