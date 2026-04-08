use rust_gsl_bindings::*;

#[test]
fn chapter2_sf_bessel() {
    let br = gsl_call!(gsl_sf_bessel_J0(5.0));
    assert!(br.is_finite());
    assert_eq!(br, -1.775967713143382642e-01);

    println!("J0(5.0) = {br:.18e}");
}

