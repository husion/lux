mod common;

use std::collections::HashMap;
use std::path::PathBuf;
use std::process::Command;

use lux_rs::{
    spectral_mismatch_correction_factor, spectral_mismatch_correction_factors,
    spectral_mismatch_f1prime, spectral_mismatch_f1primes, standard_illuminant, Observer,
    SpectralMatrix,
};

fn parse_vec(value: &str) -> Vec<f64> {
    value
        .split(',')
        .map(|item| item.parse::<f64>().unwrap())
        .collect()
}

fn parse_usize_vec(value: &str) -> Vec<usize> {
    value
        .split(',')
        .map(|item| item.parse::<usize>().unwrap())
        .collect()
}

fn assert_close(actual: f64, expected: f64, tolerance: f64) {
    let diff = (actual - expected).abs();
    assert!(
        diff <= tolerance,
        "expected {expected}, got {actual}, diff {diff}, tolerance {tolerance}"
    );
}

fn assert_vec_close(actual: &[f64], expected: &[f64], tolerance: f64) {
    assert_eq!(actual.len(), expected.len(), "length mismatch");
    for (&actual, &expected) in actual.iter().zip(expected.iter()) {
        assert_close(actual, expected, tolerance);
    }
}

fn load_python_baselines() -> HashMap<String, String> {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let python = root.join("luxpy/.venv/bin/python");
    let script = root.join("tests/python_ref/baseline_spectral_mismatch.py");

    let output = Command::new(python)
        .env("MPLCONFIGDIR", "/tmp/mpl")
        .arg(script)
        .output()
        .expect("failed to run Python baseline script");

    assert!(
        output.status.success(),
        "python baseline script failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    String::from_utf8(output.stdout)
        .unwrap()
        .lines()
        .map(|line| {
            let (key, value) = line
                .split_once('=')
                .unwrap_or_else(|| panic!("invalid baseline line: {line}"));
            (key.to_string(), value.to_string())
        })
        .collect()
}

#[test]
fn computes_zero_f1prime_for_target_matched_detector() {
    let (target, _) = Observer::Cie1931_2.vlbar().unwrap();
    let calibration = standard_illuminant("A", None).unwrap();
    let f1p = spectral_mismatch_f1prime(&target, &calibration, &target).unwrap();
    assert_close(f1p, 0.0, 1e-12);
}

#[test]
fn computes_unit_correction_factor_for_target_matched_detector() {
    let (target, _) = Observer::Cie1931_2.vlbar().unwrap();
    let calibration = standard_illuminant("A", None).unwrap();
    let measured = standard_illuminant("D65", None).unwrap();
    let factor =
        spectral_mismatch_correction_factor(&measured, &target, &calibration, &target).unwrap();
    assert_close(factor, 1.0, 1e-12);
}

#[test]
fn computes_batch_f1primes_for_xyzbar_detectors() {
    let detectors = Observer::Cie1931_2.xyzbar().unwrap();
    let (target, _) = Observer::Cie1931_2.vlbar().unwrap();
    let calibration = standard_illuminant("A", None).unwrap();
    let f1p = spectral_mismatch_f1primes(&detectors, &calibration, &target).unwrap();

    assert_eq!(f1p.len(), 3);
    assert!(f1p[0] > 0.0);
    assert_close(f1p[1], 0.0, 1e-12);
    assert!(f1p[2] > 0.0);
}

#[test]
fn matches_luxpy_detector_spectral_mismatch_baselines() {
    let baselines = load_python_baselines();
    let detectors = Observer::Cie1931_2.xyzbar().unwrap();
    let (target, _) = Observer::Cie1931_2.vlbar().unwrap();
    let calibration = standard_illuminant("A", None).unwrap();
    let d65 = standard_illuminant("D65", None).unwrap();
    let a = standard_illuminant("A", None).unwrap();
    let measured_sources = SpectralMatrix::new(
        d65.wavelengths().to_vec(),
        vec![d65.values().to_vec(), a.values().to_vec()],
    )
    .unwrap();

    let f1p = spectral_mismatch_f1primes(&detectors, &calibration, &target).unwrap();
    assert_eq!(
        vec![f1p.len()],
        parse_usize_vec(&baselines["spectral_mismatch_f1prime_shape"])
    );
    assert_vec_close(
        &f1p,
        &parse_vec(&baselines["spectral_mismatch_f1prime_xyz"]),
        1e-9,
    );

    let factors =
        spectral_mismatch_correction_factors(&measured_sources, &detectors, &calibration, &target)
            .unwrap();
    assert_eq!(
        vec![factors.len(), factors[0].len()],
        parse_usize_vec(&baselines["spectral_mismatch_factors_shape"])
    );

    let flattened: Vec<f64> = factors.into_iter().flatten().collect();
    assert_vec_close(
        &flattened,
        &parse_vec(&baselines["spectral_mismatch_factors_d65_a_xyz"]),
        1e-9,
    );
}
