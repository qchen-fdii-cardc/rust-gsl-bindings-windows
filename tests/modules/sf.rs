use random_rust::gsl;

#[test]
fn sf_const_and_struct_and_function() {
    assert!(gsl::GSL_VERSION.len() >= 2);

    let r = gsl::gsl_sf_result { val: 0.0, err: 0.0 };
    assert_eq!(r.val, 0.0);

    let y = random_rust::gsl!(gsl_sf_bessel_J0(30.2_f64));
    assert!(y.is_finite());
}
