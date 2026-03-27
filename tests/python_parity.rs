use std::collections::HashMap;
use std::path::PathBuf;
use std::process::Command;

use lux::{
    get_cie_mesopic_adaptation, getwld, getwlr, lab_to_xyz, lms_to_xyz, luv_to_xyz, spd_to_ler,
    spd_to_ler_many, spd_to_power, spd_to_xyz, spd_to_xyz_many, srgb_to_xyz, vlbar_cie_mesopic,
    xyz_to_lab, xyz_to_lms, xyz_to_luv, xyz_to_srgb, xyz_to_yuv, xyz_to_yxy, yuv_to_xyz,
    yxy_to_xyz, Observer, PowerType, SpectralMatrix, Spectrum, SpectrumNormalization,
    WavelengthGrid,
};

fn parse_scalar(value: &str) -> f64 {
    value.parse::<f64>().unwrap()
}

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
    let script = root.join("tests/python_ref/current_baselines.py");

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
fn current_rust_basics_match_luxpy() {
    let baselines = load_python_baselines();
    let observer = Observer::Cie1931_2.standard().unwrap();
    let observer_1964 = Observer::Cie1964_10.standard().unwrap();

    let xyzbar_1931 = Observer::Cie1931_2.xyzbar().unwrap();
    assert_eq!(
        vec![
            xyzbar_1931.spectrum_count() + 1,
            xyzbar_1931.wavelength_count()
        ],
        parse_usize_vec(&baselines["xyzbar_1931_shape"])
    );
    assert_vec_close(
        &[
            555.0,
            xyzbar_1931.spectra()[0][195],
            xyzbar_1931.spectra()[1][195],
            xyzbar_1931.spectra()[2][195],
        ],
        &parse_vec(&baselines["xyzbar_1931_555"]),
        1e-9,
    );

    let (vlbar_1931, k_1931) = Observer::Cie1931_2.vlbar().unwrap();
    assert_eq!(
        vec![2, vlbar_1931.wavelengths().len()],
        parse_usize_vec(&baselines["vlbar_1931_shape"])
    );
    assert_vec_close(
        &[555.0, vlbar_1931.values()[195]],
        &parse_vec(&baselines["vlbar_1931_555"]),
        1e-9,
    );
    assert_close(k_1931, parse_scalar(&baselines["vlbar_1931_k"]), 1e-12);

    let xyzbar_1964 = Observer::Cie1964_10.xyzbar().unwrap();
    assert_vec_close(
        &[
            555.0,
            xyzbar_1964.spectra()[0][195],
            xyzbar_1964.spectra()[1][195],
            xyzbar_1964.spectra()[2][195],
        ],
        &parse_vec(&baselines["xyzbar_1964_555"]),
        1e-9,
    );

    let (vlbar_1964, k_1964) = Observer::Cie1964_10.vlbar().unwrap();
    assert_vec_close(
        &[555.0, vlbar_1964.values()[195]],
        &parse_vec(&baselines["vlbar_1964_555"]),
        1e-9,
    );
    assert_close(k_1964, parse_scalar(&baselines["vlbar_1964_k"]), 1e-12);

    let (mesopic_lmes, mesopic_m) = get_cie_mesopic_adaptation(&[1.0], None, Some(&[1.0])).unwrap();
    assert_close(
        mesopic_lmes[0],
        parse_scalar(&baselines["mesopic_lmes_sp_1"]),
        1e-12,
    );
    assert_close(
        mesopic_m[0],
        parse_scalar(&baselines["mesopic_m_sp_1"]),
        1e-12,
    );

    let mesopic = vlbar_cie_mesopic(&[0.5, 1.0], Some(&[555.0])).unwrap();
    assert_eq!(
        vec![
            mesopic.curves.spectrum_count() + 1,
            mesopic.curves.wavelength_count()
        ],
        parse_usize_vec(&baselines["mesopic_vlbar_shape"])
    );
    let mut mesopic_flat = mesopic.curves.wavelengths().to_vec();
    for spectrum in mesopic.curves.spectra() {
        mesopic_flat.extend_from_slice(spectrum);
    }
    assert_vec_close(
        &mesopic_flat,
        &parse_vec(&baselines["mesopic_vlbar_555"]),
        1e-9,
    );
    assert_vec_close(
        &mesopic.k_mesopic,
        &parse_vec(&baselines["mesopic_k"]),
        1e-9,
    );
    assert_vec_close(
        &xyz_to_yxy([0.25, 0.5, 0.25]),
        &parse_vec(&baselines["xyz_to_yxy"]),
        1e-12,
    );
    assert_vec_close(
        &yxy_to_xyz([0.5, 0.25, 0.5]),
        &parse_vec(&baselines["yxy_to_xyz"]),
        1e-12,
    );
    assert_vec_close(
        &xyz_to_yuv([0.25, 0.5, 0.25]),
        &parse_vec(&baselines["xyz_to_yuv"]),
        1e-12,
    );
    assert_vec_close(
        &yuv_to_xyz([0.5, 0.117_647_058_823_529_41, 0.529_411_764_705_882_4]),
        &parse_vec(&baselines["yuv_to_xyz"]),
        1e-12,
    );
    assert_vec_close(
        &xyz_to_lab([0.25, 0.5, 0.25], [0.5, 0.5, 0.5]),
        &parse_vec(&baselines["xyz_to_lab"]),
        1e-9,
    );
    assert_vec_close(
        &lab_to_xyz(
            [100.0, -103.149_737_007_950_17, 41.259_894_803_180_07],
            [0.5, 0.5, 0.5],
        ),
        &parse_vec(&baselines["lab_to_xyz"]),
        1e-9,
    );
    assert_vec_close(
        &xyz_to_luv([0.25, 0.5, 0.25], [0.5, 0.5, 0.5]),
        &parse_vec(&baselines["xyz_to_luv"]),
        1e-9,
    );
    assert_vec_close(
        &luv_to_xyz(
            [100.0, -120.743_034_055_727_58, 72.445_820_433_436_54],
            [0.5, 0.5, 0.5],
        ),
        &parse_vec(&baselines["luv_to_xyz"]),
        1e-9,
    );
    assert_vec_close(
        &xyz_to_lms([0.25, 0.5, 0.25], Observer::Cie1931_2).unwrap(),
        &parse_vec(&baselines["xyz_to_lms_1931"]),
        1e-12,
    );
    let lms_1931 = parse_vec(&baselines["xyz_to_lms_1931"]);
    assert_vec_close(
        &lms_to_xyz([lms_1931[0], lms_1931[1], lms_1931[2]], Observer::Cie1931_2).unwrap(),
        &parse_vec(&baselines["lms_to_xyz_1931"]),
        1e-12,
    );
    assert_vec_close(
        &xyz_to_lms([0.25, 0.5, 0.25], Observer::Cie1964_10).unwrap(),
        &parse_vec(&baselines["xyz_to_lms_1964"]),
        1e-12,
    );
    assert_vec_close(
        &xyz_to_srgb([20.0, 21.0, 22.0], 2.4, -0.055, true),
        &parse_vec(&baselines["xyz_to_srgb"]),
        1e-9,
    );
    assert_vec_close(
        &srgb_to_xyz([64.0, 128.0, 192.0], 2.4, -0.055, true),
        &parse_vec(&baselines["srgb_to_xyz"]),
        1e-9,
    );

    let xyzbar_1931_interp = Observer::Cie1931_2
        .xyzbar_linear(&[554.5, 555.0, 555.5, 556.0])
        .unwrap();
    let mut xyzbar_1931_interp_flat = xyzbar_1931_interp.wavelengths().to_vec();
    xyzbar_1931_interp_flat.extend_from_slice(&xyzbar_1931_interp.spectra()[0]);
    xyzbar_1931_interp_flat.extend_from_slice(&xyzbar_1931_interp.spectra()[1]);
    xyzbar_1931_interp_flat.extend_from_slice(&xyzbar_1931_interp.spectra()[2]);
    assert_vec_close(
        &xyzbar_1931_interp_flat,
        &parse_vec(&baselines["xyzbar_1931_interp"]),
        1e-9,
    );

    let (vlbar_1931_interp, _) = Observer::Cie1931_2
        .vlbar_linear(&[554.5, 555.0, 555.5, 556.0])
        .unwrap();
    let mut vlbar_1931_interp_flat = vlbar_1931_interp.wavelengths().to_vec();
    vlbar_1931_interp_flat.extend_from_slice(vlbar_1931_interp.values());
    assert_vec_close(
        &vlbar_1931_interp_flat,
        &parse_vec(&baselines["vlbar_1931_interp"]),
        1e-9,
    );

    let spd_interp = Spectrum::new(vec![400.0, 410.0, 420.0], vec![1.0, 2.0, 3.0])
        .unwrap()
        .cie_interp_linear(&[395.0, 405.0, 420.0, 425.0], true)
        .unwrap();
    let mut spd_interp_flat = spd_interp.wavelengths().to_vec();
    spd_interp_flat.extend_from_slice(spd_interp.values());
    assert_vec_close(
        &spd_interp_flat,
        &parse_vec(&baselines["spd_interp"]),
        1e-12,
    );

    let normalize_max = Spectrum::new(vec![400.0, 410.0, 420.0], vec![1.0, 2.0, 3.0])
        .unwrap()
        .normalize(SpectrumNormalization::Max(2.0), None)
        .unwrap();
    let mut normalize_max_flat = normalize_max.wavelengths().to_vec();
    normalize_max_flat.extend_from_slice(normalize_max.values());
    assert_vec_close(
        &normalize_max_flat,
        &parse_vec(&baselines["normalize_max"]),
        1e-12,
    );

    let normalize_area = Spectrum::new(vec![400.0, 410.0, 420.0], vec![1.0, 2.0, 3.0])
        .unwrap()
        .normalize(SpectrumNormalization::Area(1.0), None)
        .unwrap();
    let mut normalize_area_flat = normalize_area.wavelengths().to_vec();
    normalize_area_flat.extend_from_slice(normalize_area.values());
    assert_vec_close(
        &normalize_area_flat,
        &parse_vec(&baselines["normalize_area"]),
        1e-12,
    );

    let normalize_lambda = Spectrum::new(vec![400.0, 410.0, 420.0], vec![1.0, 2.0, 3.0])
        .unwrap()
        .normalize(SpectrumNormalization::Lambda(410.0), None)
        .unwrap();
    let mut normalize_lambda_flat = normalize_lambda.wavelengths().to_vec();
    normalize_lambda_flat.extend_from_slice(normalize_lambda.values());
    assert_vec_close(
        &normalize_lambda_flat,
        &parse_vec(&baselines["normalize_lambda"]),
        1e-12,
    );

    let normalize_ru = Spectrum::new(vec![400.0, 410.0, 420.0], vec![1.0, 2.0, 3.0])
        .unwrap()
        .normalize(SpectrumNormalization::Radiometric(10.0), None)
        .unwrap();
    let mut normalize_ru_flat = normalize_ru.wavelengths().to_vec();
    normalize_ru_flat.extend_from_slice(normalize_ru.values());
    assert_vec_close(
        &normalize_ru_flat,
        &parse_vec(&baselines["normalize_ru"]),
        1e-12,
    );

    let normalize_pu = Spectrum::new(vec![555.0, 556.0], vec![1.0, 1.0])
        .unwrap()
        .normalize(SpectrumNormalization::Photometric(1000.0), Some(&observer))
        .unwrap();
    let mut normalize_pu_flat = normalize_pu.wavelengths().to_vec();
    normalize_pu_flat.extend_from_slice(normalize_pu.values());
    assert_vec_close(
        &normalize_pu_flat,
        &parse_vec(&baselines["normalize_pu"]),
        1e-12,
    );

    let normalize_qu = Spectrum::new(vec![500.0, 510.0], vec![1.0, 1.0])
        .unwrap()
        .normalize(SpectrumNormalization::Quantal(1e18), None)
        .unwrap();
    let mut normalize_qu_flat = normalize_qu.wavelengths().to_vec();
    normalize_qu_flat.extend_from_slice(normalize_qu.values());
    assert_vec_close(
        &normalize_qu_flat,
        &parse_vec(&baselines["normalize_qu"]),
        1e-12,
    );

    let rust_wl = getwlr(WavelengthGrid::new(360.0, 365.0, 1.0).unwrap()).unwrap();
    assert_vec_close(&rust_wl, &parse_vec(&baselines["getwlr"]), 1e-12);

    let rust_equal_spacing = getwld(&[400.0, 410.0, 420.0]).unwrap();
    let python_equal_spacing = parse_scalar(&baselines["getwld_equal_scalar"]);
    assert_vec_close(&rust_equal_spacing, &[python_equal_spacing; 3], 1e-12);

    let rust_unequal_spacing = getwld(&[400.0, 410.0, 430.0]).unwrap();
    assert_vec_close(
        &rust_unequal_spacing,
        &parse_vec(&baselines["getwld_unequal"]),
        1e-12,
    );

    let power_ru = spd_to_power(
        &Spectrum::new(vec![400.0, 410.0, 420.0], vec![1.0, 2.0, 3.0]).unwrap(),
        PowerType::Radiometric,
        None,
    )
    .unwrap();
    assert_close(power_ru, parse_scalar(&baselines["power_ru"]), 1e-12);

    let power_pu = spd_to_power(
        &Spectrum::new(vec![555.0, 556.0], vec![1.0, 1.0]).unwrap(),
        PowerType::Photometric,
        Some(&observer),
    )
    .unwrap();
    assert_close(power_pu, parse_scalar(&baselines["power_pu"]), 1e-9);

    let power_qu = spd_to_power(
        &Spectrum::new(vec![500.0, 510.0], vec![1.0, 1.0]).unwrap(),
        PowerType::Quantal,
        None,
    )
    .unwrap();
    assert_close(power_qu, parse_scalar(&baselines["power_qu"]), 1e7);

    let ler_1931 = spd_to_ler(
        &Spectrum::new(vec![555.0, 556.0], vec![1.0, 1.0]).unwrap(),
        &observer,
    )
    .unwrap();
    assert_close(ler_1931, parse_scalar(&baselines["ler_1931"]), 1e-9);

    let ler_1964 = spd_to_ler(
        &Spectrum::new(vec![555.0, 556.0], vec![1.0, 1.0]).unwrap(),
        &observer_1964,
    )
    .unwrap();
    assert_close(ler_1964, parse_scalar(&baselines["ler_1964"]), 1e-9);

    let ler_many = spd_to_ler_many(
        &SpectralMatrix::new(vec![555.0, 556.0], vec![vec![1.0, 1.0], vec![2.0, 2.0]]).unwrap(),
        &observer,
    )
    .unwrap();
    assert_vec_close(&ler_many, &parse_vec(&baselines["ler_many_1931"]), 1e-9);

    let xyz = spd_to_xyz(
        &Spectrum::new(vec![555.0, 556.0], vec![1.0, 1.0]).unwrap(),
        &observer,
        true,
    )
    .unwrap();
    assert_vec_close(&xyz, &parse_vec(&baselines["xyz_relative"]), 1e-9);

    let xyz_absolute = spd_to_xyz(
        &Spectrum::new(vec![555.0, 556.0], vec![1.0, 1.0]).unwrap(),
        &observer,
        false,
    )
    .unwrap();
    assert_vec_close(&xyz_absolute, &parse_vec(&baselines["xyz_absolute"]), 1e-9);

    let xyz_1964 = spd_to_xyz(
        &Spectrum::new(vec![555.0, 556.0], vec![1.0, 1.0]).unwrap(),
        &observer_1964,
        true,
    )
    .unwrap();
    assert_vec_close(
        &xyz_1964,
        &parse_vec(&baselines["xyz_relative_1964_10"]),
        1e-9,
    );

    let xyz_many = spd_to_xyz_many(
        &SpectralMatrix::new(vec![555.0, 556.0], vec![vec![1.0, 1.0], vec![2.0, 2.0]]).unwrap(),
        &observer,
        true,
    )
    .unwrap();
    let xyz_many_flat: Vec<f64> = xyz_many
        .into_iter()
        .flat_map(|xyz| xyz.into_iter())
        .collect();
    assert_vec_close(
        &xyz_many_flat,
        &parse_vec(&baselines["xyz_relative_many"]),
        1e-9,
    );

    let xyz_many_absolute = spd_to_xyz_many(
        &SpectralMatrix::new(vec![555.0, 556.0], vec![vec![1.0, 1.0], vec![2.0, 2.0]]).unwrap(),
        &observer,
        false,
    )
    .unwrap();
    let xyz_many_absolute_flat: Vec<f64> = xyz_many_absolute
        .into_iter()
        .flat_map(|xyz| xyz.into_iter())
        .collect();
    assert_vec_close(
        &xyz_many_absolute_flat,
        &parse_vec(&baselines["xyz_absolute_many"]),
        1e-9,
    );
}
