use rust_gsl_bindings::*;

#[test]
fn chapter3_error_handling() {
    unsafe extern "C" fn handler(
        reason: *const std::os::raw::c_char,
        file: *const std::os::raw::c_char,
        line: i32,
        gsl_errno: i32,
    ) {
        let reason = cstr!(reason);
        let file = cstr!(file);
        eprintln!("GSL error: reason=[{reason}], file=[{file}], line={line}, gsl_errno={gsl_errno}");
        // no panic, just print the error and return to the caller
    }

    let old_handler = gsl_call!(gsl_set_error_handler(Some(handler)));

    let data = [0.0_f64; 7];
    let status = gsl_call!(gsl_fft_complex_radix2_forward(
        data.as_ptr() as *mut f64,
        0,
        7
    )); // invalid input, should trigger error handler
    assert_eq!(status, gsl::GSL_EINVAL);
    // Restore the old error handler to avoid affecting other tests.

    gsl_call!(gsl_set_error_handler(old_handler));
}
