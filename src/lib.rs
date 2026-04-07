pub mod bindings {
    #![allow(non_snake_case)]
    #![allow(non_camel_case_types)]
    #![allow(non_upper_case_globals)]
    #![allow(dead_code)]

    include!("gsl_bindings.rs");
}

#[macro_export]
macro_rules! gsl {
    ($fn:ident ( $($arg:expr),* $(,)? )) => {{
        unsafe { $crate::gsl::$fn($($arg),*) }
    }};
}

pub use bindings as gsl;
