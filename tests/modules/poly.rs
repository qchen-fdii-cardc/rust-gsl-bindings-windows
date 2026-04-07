use rust_gsl_bindings::gsl;

#[test]
fn poly_const_and_struct_and_function() {
    assert_eq!(gsl::GSL_SUCCESS, 0);

    let dummy = gsl::gsl_poly_complex_workspace {
        nc: 0,
        matrix: core::ptr::null_mut(),
    };
    assert_eq!(dummy.nc, 0);

    let coeffs: [f64; 6] = [-1.0, 0.0, 0.0, 0.0, 0.0, 1.0];
    let mut roots = [0.0_f64; 10];

    let ws = gsl_call!(gsl_poly_complex_workspace_alloc(coeffs.len()));
    assert!(!ws.is_null());

    let status = gsl_call!(gsl_poly_complex_solve(
        coeffs.as_ptr(),
        coeffs.len(),
        ws,
        roots.as_mut_ptr(),
    ));

    gsl_call!(gsl_poly_complex_workspace_free(ws));
    assert_eq!(status, 0);

    let has_one = (0..5).any(|i| {
        let re = roots[2 * i];
        let im = roots[2 * i + 1];
        (re - 1.0).abs() < 1e-9 && im.abs() < 1e-9
    });
    assert!(has_one);
}
