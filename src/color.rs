use crate::error::{LuxError, LuxResult};
use crate::spectrum::{SpectralMatrix, Spectrum};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Observer {
    Cie1931_2,
    Cie1964_10,
}

#[derive(Debug, Clone, PartialEq)]
pub struct TristimulusObserver {
    pub wavelengths: Vec<f64>,
    pub x_bar: Vec<f64>,
    pub y_bar: Vec<f64>,
    pub z_bar: Vec<f64>,
    pub k: f64,
}

#[derive(Debug, Clone, PartialEq)]
pub struct MesopicLuminousEfficiency {
    pub curves: SpectralMatrix,
    pub k_mesopic: Vec<f64>,
}

pub type Matrix3 = [[f64; 3]; 3];

const EPSILON: f64 = 1e-15;
const LAB_LINEAR_THRESHOLD: f64 = (24.0 / 116.0) * (24.0 / 116.0) * (24.0 / 116.0);
const LAB_LINEAR_SCALE: f64 = 841.0 / 108.0;
const LAB_INVERSE_LINEAR_SCALE: f64 = 108.0 / 841.0;
const LUV_LINEAR_THRESHOLD: f64 = (6.0 / 29.0) * (6.0 / 29.0) * (6.0 / 29.0);
const LUV_LINEAR_SCALE: f64 = (29.0 / 3.0) * (29.0 / 3.0) * (29.0 / 3.0);
const SRGB_XYZ_TO_RGB: Matrix3 = [
    [3.2404542, -1.5371385, -0.4985314],
    [-0.9692660, 1.8760108, 0.0415560],
    [0.0556434, -0.2040259, 1.0572252],
];
const SRGB_RGB_TO_XYZ: Matrix3 = [
    [0.4124564, 0.3575761, 0.1804375],
    [0.2126729, 0.7151522, 0.0721750],
    [0.0193339, 0.1191920, 0.9503041],
];

impl Observer {
    pub fn standard(self) -> LuxResult<TristimulusObserver> {
        match self {
            Self::Cie1931_2 => TristimulusObserver::from_csv(
                include_str!("../data/cmfs/ciexyz_1931_2.dat"),
                683.002,
            ),
            Self::Cie1964_10 => TristimulusObserver::from_csv(
                include_str!("../data/cmfs/ciexyz_1964_10.dat"),
                683.599,
            ),
        }
    }

    pub fn xyzbar(self) -> LuxResult<SpectralMatrix> {
        self.standard()?.xyz_spectra()
    }

    pub fn xyzbar_linear(self, target_wavelengths: &[f64]) -> LuxResult<SpectralMatrix> {
        self.xyzbar()?.cie_interp_linear(target_wavelengths, false)
    }

    pub fn vlbar(self) -> LuxResult<(Spectrum, f64)> {
        let observer = self.standard()?;
        Ok((observer.vl_spectrum()?, observer.k))
    }

    pub fn vlbar_linear(self, target_wavelengths: &[f64]) -> LuxResult<(Spectrum, f64)> {
        let (vl, k) = self.vlbar()?;
        Ok((vl.cie_interp_linear(target_wavelengths, false)?, k))
    }

    pub fn xyz_to_lms_matrix(self) -> LuxResult<Matrix3> {
        match self {
            Self::Cie1931_2 => Ok([
                [0.38971, 0.68898, -0.07868],
                [-0.22981, 1.1834, 0.04641],
                [0.0, 0.0, 1.0],
            ]),
            Self::Cie1964_10 => Ok([
                [
                    0.217_010_449_691_388_16,
                    0.835_733_670_117_584_4,
                    -0.043_510_597_212_556_935,
                ],
                [
                    -0.429_979_507_573_619_8,
                    1.203_889_456_462_98,
                    0.086_210_895_329_211_28,
                ],
                [0.0, 0.0, 0.465_792_338_736_113],
            ]),
        }
    }
}

impl TristimulusObserver {
    pub fn from_csv(csv: &str, k: f64) -> LuxResult<Self> {
        let mut wavelengths = Vec::new();
        let mut x_bar = Vec::new();
        let mut y_bar = Vec::new();
        let mut z_bar = Vec::new();

        for line in csv.lines() {
            let trimmed = line.trim();
            if trimmed.is_empty() {
                continue;
            }
            let mut parts = trimmed.split(',');
            let wl = parts
                .next()
                .ok_or(LuxError::ParseError("missing wavelength"))?
                .trim()
                .parse::<f64>()
                .map_err(|_| LuxError::ParseError("invalid wavelength"))?;
            let x = parts
                .next()
                .ok_or(LuxError::ParseError("missing x_bar"))?
                .trim()
                .parse::<f64>()
                .map_err(|_| LuxError::ParseError("invalid x_bar"))?;
            let y = parts
                .next()
                .ok_or(LuxError::ParseError("missing y_bar"))?
                .trim()
                .parse::<f64>()
                .map_err(|_| LuxError::ParseError("invalid y_bar"))?;
            let z = parts
                .next()
                .ok_or(LuxError::ParseError("missing z_bar"))?
                .trim()
                .parse::<f64>()
                .map_err(|_| LuxError::ParseError("invalid z_bar"))?;

            wavelengths.push(wl);
            x_bar.push(x);
            y_bar.push(y);
            z_bar.push(z);
        }

        if wavelengths.is_empty() {
            return Err(LuxError::EmptyInput);
        }

        Ok(Self {
            wavelengths,
            x_bar,
            y_bar,
            z_bar,
            k,
        })
    }

    pub fn vl_spectrum(&self) -> LuxResult<Spectrum> {
        Spectrum::new(self.wavelengths.clone(), self.y_bar.clone())
    }

    pub fn xyz_spectra(&self) -> LuxResult<SpectralMatrix> {
        SpectralMatrix::new(
            self.wavelengths.clone(),
            vec![self.x_bar.clone(), self.y_bar.clone(), self.z_bar.clone()],
        )
    }

    pub fn x_bar_spectrum(&self) -> LuxResult<Spectrum> {
        Spectrum::new(self.wavelengths.clone(), self.x_bar.clone())
    }

    pub fn z_bar_spectrum(&self) -> LuxResult<Spectrum> {
        Spectrum::new(self.wavelengths.clone(), self.z_bar.clone())
    }
}

fn nonzero(value: f64) -> f64 {
    if value == 0.0 {
        EPSILON
    } else {
        value
    }
}

fn lab_response_curve(value: f64, white: f64) -> f64 {
    let ratio = value / white;
    if ratio <= LAB_LINEAR_THRESHOLD {
        LAB_LINEAR_SCALE * ratio + 16.0 / 116.0
    } else {
        ratio.cbrt()
    }
}

fn lab_inverse_response_curve(response: f64, white: f64) -> f64 {
    if response <= 24.0 / 116.0 {
        white * ((response - 16.0 / 116.0) * LAB_INVERSE_LINEAR_SCALE)
    } else {
        white * response.powi(3)
    }
}

fn cie_lightness_from_ratio(y_ratio: f64) -> f64 {
    if y_ratio <= LUV_LINEAR_THRESHOLD {
        LUV_LINEAR_SCALE * y_ratio
    } else {
        116.0 * y_ratio.cbrt() - 16.0
    }
}

fn cie_y_ratio_from_lightness(lightness: f64) -> f64 {
    let y_ratio = ((lightness + 16.0) / 116.0).powi(3);
    if y_ratio < LUV_LINEAR_THRESHOLD {
        lightness / LUV_LINEAR_SCALE
    } else {
        y_ratio
    }
}

fn multiply_matrix3_vector3(matrix: Matrix3, vector: [f64; 3]) -> [f64; 3] {
    [
        matrix[0][0] * vector[0] + matrix[0][1] * vector[1] + matrix[0][2] * vector[2],
        matrix[1][0] * vector[0] + matrix[1][1] * vector[1] + matrix[1][2] * vector[2],
        matrix[2][0] * vector[0] + matrix[2][1] * vector[1] + matrix[2][2] * vector[2],
    ]
}

fn clamp(value: f64, min: f64, max: f64) -> f64 {
    value.max(min).min(max)
}

fn invert_matrix3(matrix: Matrix3) -> Matrix3 {
    let a = matrix[0][0];
    let b = matrix[0][1];
    let c = matrix[0][2];
    let d = matrix[1][0];
    let e = matrix[1][1];
    let f = matrix[1][2];
    let g = matrix[2][0];
    let h = matrix[2][1];
    let i = matrix[2][2];

    let cofactor00 = e * i - f * h;
    let cofactor01 = -(d * i - f * g);
    let cofactor02 = d * h - e * g;
    let cofactor10 = -(b * i - c * h);
    let cofactor11 = a * i - c * g;
    let cofactor12 = -(a * h - b * g);
    let cofactor20 = b * f - c * e;
    let cofactor21 = -(a * f - c * d);
    let cofactor22 = a * e - b * d;

    let determinant = a * cofactor00 + b * cofactor01 + c * cofactor02;
    let inv_det = 1.0 / determinant;

    [
        [
            cofactor00 * inv_det,
            cofactor10 * inv_det,
            cofactor20 * inv_det,
        ],
        [
            cofactor01 * inv_det,
            cofactor11 * inv_det,
            cofactor21 * inv_det,
        ],
        [
            cofactor02 * inv_det,
            cofactor12 * inv_det,
            cofactor22 * inv_det,
        ],
    ]
}

// CIE chromaticity transforms.

pub fn xyz_to_yxy(xyz: [f64; 3]) -> [f64; 3] {
    let sum = xyz[0] + xyz[1] + xyz[2];
    let denominator = nonzero(sum);
    [xyz[1], xyz[0] / denominator, xyz[1] / denominator]
}

pub fn yxy_to_xyz(yxy: [f64; 3]) -> [f64; 3] {
    let y = nonzero(yxy[2]);
    [
        yxy[0] * yxy[1] / y,
        yxy[0],
        yxy[0] * (1.0 - yxy[1] - yxy[2]) / y,
    ]
}

pub fn xyz_to_yuv(xyz: [f64; 3]) -> [f64; 3] {
    let denominator = xyz[0] + 15.0 * xyz[1] + 3.0 * xyz[2];
    let denominator = nonzero(denominator);
    [
        xyz[1],
        4.0 * xyz[0] / denominator,
        9.0 * xyz[1] / denominator,
    ]
}

pub fn yuv_to_xyz(yuv: [f64; 3]) -> [f64; 3] {
    let v = nonzero(yuv[2]);
    [
        yuv[0] * (9.0 * yuv[1]) / (4.0 * v),
        yuv[0],
        yuv[0] * (12.0 - 3.0 * yuv[1] - 20.0 * yuv[2]) / (4.0 * v),
    ]
}

// LMS transforms.

pub fn xyz_to_lms_with_matrix(xyz: [f64; 3], matrix: Matrix3) -> [f64; 3] {
    multiply_matrix3_vector3(matrix, xyz)
}

pub fn lms_to_xyz_with_matrix(lms: [f64; 3], matrix: Matrix3) -> [f64; 3] {
    multiply_matrix3_vector3(invert_matrix3(matrix), lms)
}

pub fn xyz_to_lms(xyz: [f64; 3], observer: Observer) -> LuxResult<[f64; 3]> {
    Ok(xyz_to_lms_with_matrix(xyz, observer.xyz_to_lms_matrix()?))
}

pub fn lms_to_xyz(lms: [f64; 3], observer: Observer) -> LuxResult<[f64; 3]> {
    Ok(lms_to_xyz_with_matrix(lms, observer.xyz_to_lms_matrix()?))
}

// sRGB transforms.

pub fn xyz_to_srgb(xyz: [f64; 3], gamma: f64, offset: f64, use_linear_part: bool) -> [f64; 3] {
    let linear = multiply_matrix3_vector3(
        SRGB_XYZ_TO_RGB,
        [xyz[0] / 100.0, xyz[1] / 100.0, xyz[2] / 100.0],
    );

    let mut rgb = [0.0; 3];
    for (index, linear_value) in linear.iter().enumerate() {
        let srgb = clamp(*linear_value, 0.0, 1.0);
        let mut encoded = ((1.0 - offset) * srgb.powf(1.0 / gamma) + offset) * 255.0;
        if use_linear_part && srgb <= 0.0031308 {
            encoded = srgb * 12.92 * 255.0;
        }
        rgb[index] = clamp(encoded, 0.0, 255.0);
    }

    rgb
}

pub fn srgb_to_xyz(rgb: [f64; 3], gamma: f64, offset: f64, use_linear_part: bool) -> [f64; 3] {
    let scaled = [rgb[0] / 255.0, rgb[1] / 255.0, rgb[2] / 255.0];
    let mut linear = [0.0; 3];

    for (index, encoded) in scaled.iter().enumerate() {
        let mut value = ((*encoded - offset) / (1.0 - offset)).powf(gamma);
        if use_linear_part && value < 0.0031308 {
            value = *encoded / 12.92;
        }
        linear[index] = value;
    }

    let xyz = multiply_matrix3_vector3(SRGB_RGB_TO_XYZ, linear);
    [xyz[0] * 100.0, xyz[1] * 100.0, xyz[2] * 100.0]
}

// CIE perceptual color spaces with explicit white point input.

pub fn xyz_to_lab(xyz: [f64; 3], white_point: [f64; 3]) -> [f64; 3] {
    let fx = lab_response_curve(xyz[0], white_point[0]);
    let fy = lab_response_curve(xyz[1], white_point[1]);
    let fz = lab_response_curve(xyz[2], white_point[2]);
    let l = cie_lightness_from_ratio(xyz[1] / white_point[1]);

    [l, 500.0 * (fx - fy), 200.0 * (fy - fz)]
}

pub fn lab_to_xyz(lab: [f64; 3], white_point: [f64; 3]) -> [f64; 3] {
    let fy = (lab[0] + 16.0) / 116.0;
    let fx = lab[1] / 500.0 + fy;
    let fz = fy - lab[2] / 200.0;

    [
        lab_inverse_response_curve(fx, white_point[0]),
        lab_inverse_response_curve(fy, white_point[1]),
        lab_inverse_response_curve(fz, white_point[2]),
    ]
}

pub fn xyz_to_luv(xyz: [f64; 3], white_point: [f64; 3]) -> [f64; 3] {
    let yuv = xyz_to_yuv(xyz);
    let white_yuv = xyz_to_yuv(white_point);
    let y_ratio = yuv[0] / white_yuv[0];
    let l = cie_lightness_from_ratio(y_ratio);

    [
        l,
        13.0 * l * (yuv[1] - white_yuv[1]),
        13.0 * l * (yuv[2] - white_yuv[2]),
    ]
}

pub fn luv_to_xyz(luv: [f64; 3], white_point: [f64; 3]) -> [f64; 3] {
    let white_yuv = xyz_to_yuv(white_point);
    let mut yuv = [0.0; 3];
    if luv[0] == 0.0 {
        yuv[1] = 0.0;
        yuv[2] = 0.0;
    } else {
        yuv[1] = luv[1] / (13.0 * luv[0]) + white_yuv[1];
        yuv[2] = luv[2] / (13.0 * luv[0]) + white_yuv[2];
    }

    yuv[0] = white_yuv[0] * cie_y_ratio_from_lightness(luv[0]);

    yuv_to_xyz(yuv)
}

pub fn get_cie_mesopic_adaptation(
    photopic_luminance: &[f64],
    scotopic_luminance: Option<&[f64]>,
    s_p_ratio: Option<&[f64]>,
) -> LuxResult<(Vec<f64>, Vec<f64>)> {
    if photopic_luminance.is_empty() {
        return Err(LuxError::EmptyInput);
    }
    if scotopic_luminance.is_some() == s_p_ratio.is_some() {
        return Err(LuxError::InvalidInput(
            "provide exactly one of scotopic_luminance or s_p_ratio",
        ));
    }

    let len = photopic_luminance.len();
    if let Some(ls) = scotopic_luminance {
        if ls.len() != len {
            return Err(LuxError::MismatchedLengths {
                wavelengths: len,
                values: ls.len(),
            });
        }
    }
    if let Some(sp) = s_p_ratio {
        if sp.len() != len {
            return Err(LuxError::MismatchedLengths {
                wavelengths: len,
                values: sp.len(),
            });
        }
    }

    let mut lmes = Vec::with_capacity(len);
    let mut m_values = Vec::with_capacity(len);

    for index in 0..len {
        let lp = photopic_luminance[index];
        if !lp.is_finite() || lp <= 0.0 {
            return Err(LuxError::InvalidInput(
                "photopic luminance values must be finite and positive",
            ));
        }

        let sp = if let Some(ls) = scotopic_luminance {
            let scotopic = ls[index];
            if !scotopic.is_finite() || scotopic < 0.0 {
                return Err(LuxError::InvalidInput(
                    "scotopic luminance values must be finite and non-negative",
                ));
            }
            scotopic / lp
        } else {
            let ratio = s_p_ratio.unwrap()[index];
            if !ratio.is_finite() || ratio < 0.0 {
                return Err(LuxError::InvalidInput(
                    "S/P ratio values must be finite and non-negative",
                ));
            }
            ratio
        };

        let f_lmes = |m: f64| {
            ((m * lp) + (1.0 - m) * sp * 683.0 / 1699.0) / (m + (1.0 - m) * 683.0 / 1699.0)
        };
        let f_m = |m: f64| 0.767 + 0.3334 * f_lmes(m).log10();

        let mut previous = 0.5;
        let mut current = f_m(previous);
        let mut iterations = 0;
        while (current - previous).abs() > 1e-12 && iterations < 100 {
            previous = current;
            current = f_m(previous);
            iterations += 1;
        }

        lmes.push(f_lmes(current));
        m_values.push(current.clamp(0.0, 1.0));
    }

    Ok((lmes, m_values))
}

pub fn vlbar_cie_mesopic(
    m_values: &[f64],
    target_wavelengths: Option<&[f64]>,
) -> LuxResult<MesopicLuminousEfficiency> {
    if m_values.is_empty() {
        return Err(LuxError::EmptyInput);
    }

    let photopic = Observer::Cie1931_2.vlbar()?.0;
    let wavelengths = photopic.wavelengths().to_vec();
    let scotopic = load_scotopic_vlbar_on(&wavelengths)?;
    let peak_index = wavelengths
        .iter()
        .position(|&wavelength| (wavelength - 555.0).abs() < 1e-12)
        .ok_or(LuxError::ParseError(
            "missing 555 nm in mesopic source data",
        ))?;

    let mut curves = Vec::with_capacity(m_values.len());
    let mut k_mesopic = Vec::with_capacity(m_values.len());

    for &m in m_values {
        let m = m.clamp(0.0, 1.0);
        let values: Vec<f64> = photopic
            .values()
            .iter()
            .zip(scotopic.values().iter())
            .map(|(vp, vs)| m * vp + (1.0 - m) * vs)
            .collect();

        let k = 683.0 / values[peak_index];
        curves.push(values);
        k_mesopic.push(k);
    }

    let curves = SpectralMatrix::new(wavelengths, curves)?;
    let curves = if let Some(target_wavelengths) = target_wavelengths {
        curves.cie_interp_linear(target_wavelengths, false)?
    } else {
        curves
    };
    let normalization =
        vec![crate::spectrum::SpectrumNormalization::Max(1.0); curves.spectrum_count()];
    let curves = curves.normalize_each(&normalization, None)?;

    Ok(MesopicLuminousEfficiency { curves, k_mesopic })
}

fn load_scotopic_vlbar_on(target_wavelengths: &[f64]) -> LuxResult<Spectrum> {
    let base = TristimulusObserver::from_csv(
        include_str!("../luxpy/luxpy/data/cmfs/ciexyz_1951_20_scotopic.dat"),
        1699.0,
    )?
    .vl_spectrum()?;

    let source_wavelengths = base.wavelengths().to_vec();
    let interpolated = base.cie_interp_linear(target_wavelengths, false)?;
    let clipped = target_wavelengths
        .iter()
        .zip(interpolated.values().iter())
        .map(|(&wavelength, &value)| {
            if wavelength < source_wavelengths[0]
                || wavelength > source_wavelengths[source_wavelengths.len() - 1]
            {
                0.0
            } else if value.is_sign_negative() {
                0.0
            } else {
                value
            }
        })
        .collect();

    Spectrum::new(target_wavelengths.to_vec(), clipped)
}

#[cfg(test)]
mod tests {
    use super::{
        get_cie_mesopic_adaptation, lab_to_xyz, lms_to_xyz, luv_to_xyz, srgb_to_xyz,
        vlbar_cie_mesopic, xyz_to_lab, xyz_to_lms, xyz_to_luv, xyz_to_srgb, xyz_to_yuv, xyz_to_yxy,
        yuv_to_xyz, yxy_to_xyz, Observer,
    };

    #[test]
    fn loads_standard_observer() {
        let observer = Observer::Cie1931_2.standard().unwrap();
        assert_eq!(observer.wavelengths.first().copied(), Some(360.0));
        assert_eq!(observer.wavelengths.last().copied(), Some(830.0));
        assert_eq!(observer.wavelengths.len(), 471);
    }

    #[test]
    fn exposes_xyzbar() {
        let xyzbar = Observer::Cie1931_2.xyzbar().unwrap();
        assert_eq!(xyzbar.wavelength_count(), 471);
        assert_eq!(xyzbar.spectrum_count(), 3);
    }

    #[test]
    fn exposes_vlbar_and_k() {
        let (vl, k) = Observer::Cie1931_2.vlbar().unwrap();
        assert_eq!(vl.wavelengths().len(), 471);
        assert_eq!(vl.values()[195], 1.0);
        assert_eq!(k, 683.002);
    }

    #[test]
    fn interpolates_xyzbar_linearly() {
        let xyzbar = Observer::Cie1931_2
            .xyzbar_linear(&[554.5, 555.0, 555.5, 556.0])
            .unwrap();
        assert!((xyzbar.spectra()[0][0] - 0.504_010_7).abs() < 1e-9);
        assert!((xyzbar.spectra()[1][1] - 1.0).abs() < 1e-12);
        assert!((xyzbar.spectra()[2][3] - 0.005_303_6).abs() < 1e-9);
    }

    #[test]
    fn computes_cie_mesopic_adaptation_from_s_p_ratio() {
        let (lmes, m_values) = get_cie_mesopic_adaptation(&[1.0], None, Some(&[1.0])).unwrap();
        assert!((lmes[0] - 1.0).abs() < 1e-12);
        assert!((m_values[0] - 0.767).abs() < 1e-12);
    }

    #[test]
    fn computes_mesopic_luminous_efficiency_curve() {
        let mesopic = vlbar_cie_mesopic(&[0.5, 1.0], None).unwrap();
        assert_eq!(mesopic.curves.spectrum_count(), 2);
        assert_eq!(mesopic.curves.wavelength_count(), 471);
        assert!((mesopic.k_mesopic[0] - 974.322_396_576_319_4).abs() < 1e-9);
        assert!((mesopic.k_mesopic[1] - 683.0).abs() < 1e-12);
        assert!((mesopic.curves.spectra()[0][195] - 0.837_061_500_974_263_2).abs() < 1e-9);
        assert!((mesopic.curves.spectra()[1][195] - 1.0).abs() < 1e-12);
    }

    #[test]
    fn converts_xyz_to_yxy() {
        let yxy = xyz_to_yxy([0.25, 0.5, 0.25]);
        assert!((yxy[0] - 0.5).abs() < 1e-12);
        assert!((yxy[1] - 0.25).abs() < 1e-12);
        assert!((yxy[2] - 0.5).abs() < 1e-12);
    }

    #[test]
    fn converts_yxy_to_xyz() {
        let xyz = yxy_to_xyz([0.5, 0.25, 0.5]);
        assert!((xyz[0] - 0.25).abs() < 1e-12);
        assert!((xyz[1] - 0.5).abs() < 1e-12);
        assert!((xyz[2] - 0.25).abs() < 1e-12);
    }

    #[test]
    fn converts_xyz_to_yuv() {
        let yuv = xyz_to_yuv([0.25, 0.5, 0.25]);
        assert!((yuv[0] - 0.5).abs() < 1e-12);
        assert!((yuv[1] - 0.117_647_058_823_529_41).abs() < 1e-12);
        assert!((yuv[2] - 0.529_411_764_705_882_4).abs() < 1e-12);
    }

    #[test]
    fn converts_yuv_to_xyz() {
        let xyz = yuv_to_xyz([0.5, 0.117_647_058_823_529_41, 0.529_411_764_705_882_4]);
        assert!((xyz[0] - 0.25).abs() < 1e-12);
        assert!((xyz[1] - 0.5).abs() < 1e-12);
        assert!((xyz[2] - 0.25).abs() < 1e-12);
    }

    #[test]
    fn converts_xyz_to_lab() {
        let lab = xyz_to_lab([0.25, 0.5, 0.25], [0.5, 0.5, 0.5]);
        assert!((lab[0] - 100.0).abs() < 1e-12);
        assert!((lab[1] + 103.149_737_007_950_17).abs() < 1e-9);
        assert!((lab[2] - 41.259_894_803_180_07).abs() < 1e-9);
    }

    #[test]
    fn converts_lab_to_xyz() {
        let xyz = lab_to_xyz(
            [100.0, -103.149_737_007_950_17, 41.259_894_803_180_07],
            [0.5, 0.5, 0.5],
        );
        assert!((xyz[0] - 0.25).abs() < 1e-9);
        assert!((xyz[1] - 0.5).abs() < 1e-12);
        assert!((xyz[2] - 0.25).abs() < 1e-9);
    }

    #[test]
    fn converts_xyz_to_luv() {
        let luv = xyz_to_luv([0.25, 0.5, 0.25], [0.5, 0.5, 0.5]);
        assert!((luv[0] - 100.0).abs() < 1e-12);
        assert!((luv[1] + 120.743_034_055_727_58).abs() < 1e-9);
        assert!((luv[2] - 72.445_820_433_436_54).abs() < 1e-9);
    }

    #[test]
    fn converts_luv_to_xyz() {
        let xyz = luv_to_xyz(
            [100.0, -120.743_034_055_727_58, 72.445_820_433_436_54],
            [0.5, 0.5, 0.5],
        );
        assert!((xyz[0] - 0.25).abs() < 1e-9);
        assert!((xyz[1] - 0.5).abs() < 1e-12);
        assert!((xyz[2] - 0.25).abs() < 1e-9);
    }

    #[test]
    fn converts_xyz_to_lms_for_1931() {
        let lms = xyz_to_lms([0.25, 0.5, 0.25], Observer::Cie1931_2).unwrap();
        assert!((lms[0] - 0.422_247_5).abs() < 1e-12);
        assert!((lms[1] - 0.545_850_000_000_000_1).abs() < 1e-12);
        assert!((lms[2] - 0.25).abs() < 1e-12);
    }

    #[test]
    fn converts_lms_to_xyz_for_1931() {
        let xyz = lms_to_xyz(
            [0.422_247_5, 0.545_850_000_000_000_1, 0.25],
            Observer::Cie1931_2,
        )
        .unwrap();
        assert!((xyz[0] - 0.25).abs() < 1e-12);
        assert!((xyz[1] - 0.5).abs() < 1e-12);
        assert!((xyz[2] - 0.25).abs() < 1e-12);
    }

    #[test]
    fn converts_xyz_to_lms_for_1964() {
        let lms = xyz_to_lms([0.25, 0.5, 0.25], Observer::Cie1964_10).unwrap();
        assert!((lms[0] - 0.461_241_798_178_5).abs() < 1e-12);
        assert!((lms[1] - 0.516_002_575_170_388).abs() < 1e-12);
        assert!((lms[2] - 0.116_448_084_684_028_24).abs() < 1e-12);
    }

    #[test]
    fn exposes_xyz_to_lms_matrix_for_1931() {
        let matrix = Observer::Cie1931_2.xyz_to_lms_matrix().unwrap();
        assert_eq!(matrix[0], [0.38971, 0.68898, -0.07868]);
        assert_eq!(matrix[2], [0.0, 0.0, 1.0]);
    }

    #[test]
    fn converts_xyz_to_srgb() {
        let rgb = xyz_to_srgb([20.0, 21.0, 22.0], 2.4, -0.055, true);
        assert!((rgb[0] - 127.932_633_053_083_4).abs() < 1e-9);
        assert!((rgb[1] - 126.171_697_951_843_17).abs() < 1e-9);
        assert!((rgb[2] - 123.804_791_369_705).abs() < 1e-9);
    }

    #[test]
    fn converts_srgb_to_xyz() {
        let xyz = srgb_to_xyz([64.0, 128.0, 192.0], 2.4, -0.055, true);
        assert!((xyz[0] - 19.344_430_750_022_802).abs() < 1e-9);
        assert!((xyz[1] - 20.332_127_014_120_942).abs() < 1e-9);
        assert!((xyz[2] - 52.763_974_844_108_34).abs() < 1e-9);
    }
}
