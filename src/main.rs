use rust_gsl_bindings::*;
use std::prelude::rust_2024::*;

fn main() {
    // Print GSL version as a C string instead of raw bytes.

    let gsl_version = gsl_cstr!(gsl::GSL_VERSION);

    println!("GSL version: {gsl_version}");

    let x = 0.20;
    let y = gsl_call!(gsl_sf_bessel_J0(x));

    println!("J0({x}) = {y}");

    // Example: solve x^5 - 1 = 0 using GSL's polynomial root solver.
    // Coefficients are in ascending order: c0 + c1*x + ... + c5*x^5 = 0.
    let coeffs: [f64; 6] = [-1.0, 0.0, 0.0, 0.0, 0.0, 1.0];
    let mut roots_packed = [0.0_f64; 10]; // 5 complex roots => 10 doubles (re,im pairs)

    let w = gsl_call!(gsl_poly_complex_workspace_alloc(coeffs.len()));
    assert!(
        !w.is_null(),
        "failed to allocate gsl_poly_complex_workspace"
    );

    let status = gsl_call!(gsl_poly_complex_solve(
        coeffs.as_ptr(),
        coeffs.len(),
        w,
        roots_packed.as_mut_ptr(),
    ));

    gsl_call!(gsl_poly_complex_workspace_free(w));
    assert_eq!(
        status, 0,
        "gsl_poly_complex_solve failed with status={status}"
    );

    println!("Roots of x^5 - 1 = 0:");
    for i in 0..5 {
        let re = roots_packed[2 * i];
        let im = roots_packed[2 * i + 1];
        println!("  root[{i}] = {re:+.12} {im:+.12}i");
    }

    // Example: compute integral_0^1 x^2 dx = 1/3 with GSL adaptive integration.
    let limit: usize = 1000;
    let workspace = gsl_call!(gsl_integration_workspace_alloc(limit));
    assert!(
        !workspace.is_null(),
        "failed to allocate integration workspace"
    );
    unsafe extern "C" fn square_integrand(x: f64, _params: *mut std::os::raw::c_void) -> f64 {
        x * x
    }
    let mut f = gsl::gsl_function {
        function: Some(square_integrand),
        params: std::ptr::null_mut(),
    };

    let mut result = 0.0_f64;
    let mut abserr = 0.0_f64;
    let status = gsl_call!(gsl_integration_qag(
        &mut f,
        0.0,
        1.0,
        1e-12,
        1e-12,
        limit,
        6,
        workspace,
        &mut result,
        &mut abserr,
    ));

    gsl_call!(gsl_integration_workspace_free(workspace));
    assert_eq!(status, 0, "gsl_integration_qag failed with status={status}");

    println!(
        "Integral of x^2 on [0,1] = {result:.12} (expected {:.12}, abs err {:.3e})",
        1.0 / 3.0,
        abserr
    );

    // 矩阵乘法示例：计算 C = A * B，其中 A 是 2x3 矩阵，B 是 3x2 矩阵，结果 C 是 2x2 矩阵。
    let a: [f64; 6] = [1.0, 2.0, 3.0, 4.0, 5.0, 6.0]; // 2x3 矩阵，行优先存储
    let b: [f64; 6] = [7.0, 8.0, 9.0, 10.0, 11.0, 12.0]; // 3x2 矩阵，行优先存储
    let mut c: [f64; 4] = [0.0; 4]; // 2x2 矩阵，行优先存储

    // 使用 GSL 的 BLAS 接口进行矩阵乘法：C = A * B
    // gsl_blas_dgemm 需要 gsl_matrix*，因此先把数组包装成 matrix view。
    let m = 2_usize; // A 的行数
    let n = 2_usize; // B 的列数
    let k = 3_usize; // A 的列数和 B 的行数
    let alpha = 1.0;
    let beta = 0.0;

    let a_view = gsl_call!(gsl_matrix_view_array(a.as_ptr() as *mut f64, m, k));
    let b_view = gsl_call!(gsl_matrix_view_array(b.as_ptr() as *mut f64, k, n));
    let mut c_view = gsl_call!(gsl_matrix_view_array(c.as_mut_ptr(), m, n));

    let status = gsl_call!(gsl_blas_dgemm(
        gsl::CBLAS_TRANSPOSE_CblasNoTrans as gsl::CBLAS_TRANSPOSE_t,
        gsl::CBLAS_TRANSPOSE_CblasNoTrans as gsl::CBLAS_TRANSPOSE_t,
        alpha,
        &a_view.matrix,
        &b_view.matrix,
        beta,
        &mut c_view.matrix,
    ));
    assert_eq!(status, 0, "gsl_blas_dgemm failed with status={status}");

    println!("Result of C = A * B:");
    for i in 0..m {
        for j in 0..n {
            print!("{:8.2} ", c[i * n + j]);
        }
        println!();
    }

    // gsl_vector_alloc/gsl_matrix_alloc 示例：分配一个 3x3 的矩阵，设置一些值，进行计算。
    let size = 3_usize;
    let matrix = gsl_call!(gsl_matrix_alloc(size, size));
    assert!(
        !matrix.is_null(),
        "failed to allocate gsl_matrix for 3x3 matrix"
    );
    // 设置矩阵元素：matrix[i][j] = i + j
    for i in 0..size {
        for j in 0..size {
            // unsafe {
            //     *gsl!(gsl_matrix_ptr(matrix, i, j)) = (i + j) as f64;
            // }
            gsl_call!(gsl_matrix_set(matrix, i, j, (i + j) as f64));
        }
    }

    println!("3x3 matrix with elements matrix[i][j] = i + j:");
    for i in 0..size {
        for j in 0..size {
            let val = gsl_call!(gsl_matrix_get(matrix, i, j));
            print!("{:5.1} ", val);
        }
        println!();
    }

    // show how to access matrix elements directly
    println!("Accessing matrix elements directly using gsl_matrix_ptr:");
    let rows = unsafe { (*matrix).size1 };
    let cols = unsafe { (*matrix).size2 };
    for i in 0..rows {
        for j in 0..cols {
            let mut val = unsafe { *gsl_call!(gsl_matrix_ptr(matrix, i, j)) };
            val += 0.5; // add 0.5 to show that we can modify the value through the pointer
            print!("{:5.1} ", val);
        }
        println!();
    }

    gsl_call!(gsl_matrix_free(matrix));

    // Sobol quasi Monte Carlo example:
    // estimate integral over [0,1]^D of f(x1, x2, ..., xD).
    let dim = 6_usize; // 3-dimensional integral
    let dim_qrng = u32::try_from(dim).expect("dimension does not fit in u32");
    let n_points = 2000_usize; // number of quasi-random points to sample
    let mut points = vec![0.0_f64; dim * n_points];
    let sobol_type = unsafe { gsl::gsl_qrng_sobol };
    let rng = gsl_call!(gsl_qrng_alloc(sobol_type, dim_qrng));
    assert!(
        !rng.is_null(),
        "failed to allocate gsl_qrng for Sobol sequence generator"
    );

    let fn_to_integrate = |x: &[f64]| -> f64 {
        // Example integrand: f(x1,x2,x3) = exp(-(x1^2 + x2^2 + x3^2))
        let sum_squares: f64 = x.iter().map(|xi| xi * xi).sum();
        (-sum_squares).exp()
    };

    gsl_call!(gsl_qrng_init(rng));
    for i in 0..n_points {
        let point = &mut points[i * dim..(i + 1) * dim];
        let status = gsl_call!(gsl_qrng_get(rng, point.as_mut_ptr()));
        assert_eq!(status, 0, "gsl_qrng_get failed with status={status}");
    }
    gsl_call!(gsl_qrng_free(rng));

    let integral_estimate: f64 =
        points.chunks(dim).map(fn_to_integrate).sum::<f64>() / (n_points as f64);
    println!(
        "Integral exp(-sum(x1^2 + ... x{dim}^2)) over [0,1]^{dim} using {n_points} Sobol points: {integral_estimate:.16}"
    );
    // show the theoretical value for comparison: integral_0^1 exp(-x^2) dx = (sqrt(pi) * erf(1)) / 2
    let theoretical_1d = (std::f64::consts::PI.sqrt() * gsl_call!(gsl_sf_erf(1.0_f64))) / 2.0;
    let theoretical = theoretical_1d.powi(dim as i32);
    println!("Theoretical value for comparison: (sqrt(pi) * erf(1) / 2)^{dim} = {theoretical:.16}");
    println!(
        "Absolute error of the estimate: {:.3e}",
        (integral_estimate - theoretical).abs()
    );
}
