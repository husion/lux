use crate::color::TristimulusObserver;
use crate::error::{LuxError, LuxResult};
use crate::spectrum::Spectrum;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PowerType {
    Radiometric,
    Photometric,
    Quantal,
}

const PLANCK_CONSTANT: f64 = 6.626_070_15e-34;
const SPEED_OF_LIGHT: f64 = 299_792_458.0;

pub fn spd_to_xyz(
    spectrum: &Spectrum,
    observer: &TristimulusObserver,
    relative: bool,
) -> LuxResult<[f64; 3]> {
    let wavelengths = spectrum.wavelengths();
    let x_bar = observer.x_bar_spectrum()?.interpolate_linear(wavelengths)?;
    let y_bar = observer.vl_spectrum()?.interpolate_linear(wavelengths)?;
    let z_bar = observer.z_bar_spectrum()?.interpolate_linear(wavelengths)?;
    integrate_xyz(
        spectrum,
        x_bar.values(),
        y_bar.values(),
        z_bar.values(),
        observer.k,
        relative,
    )
}

pub fn spd_to_ler(spectrum: &Spectrum, observer: &TristimulusObserver) -> LuxResult<f64> {
    let photometric = spd_to_power(spectrum, PowerType::Photometric, Some(observer))?;
    let radiometric = spd_to_power(spectrum, PowerType::Radiometric, None)?;
    Ok(photometric / radiometric)
}

pub(crate) fn integrate_xyz(
    spectrum: &Spectrum,
    x_bar: &[f64],
    y_bar: &[f64],
    z_bar: &[f64],
    k: f64,
    relative: bool,
) -> LuxResult<[f64; 3]> {
    let spacing = spectrum.spacing()?;
    let values = spectrum.values();

    let x: f64 = values
        .iter()
        .zip(&spacing)
        .zip(x_bar.iter())
        .map(|((value, dl), x_bar)| value * dl * x_bar)
        .sum();
    let y: f64 = values
        .iter()
        .zip(&spacing)
        .zip(y_bar.iter())
        .map(|((value, dl), y_bar)| value * dl * y_bar)
        .sum();
    let z: f64 = values
        .iter()
        .zip(&spacing)
        .zip(z_bar.iter())
        .map(|((value, dl), z_bar)| value * dl * z_bar)
        .sum();

    let scale = if relative { 100.0 / y } else { k };
    Ok([x * scale, y * scale, z * scale])
}

pub fn spd_to_power(
    spectrum: &Spectrum,
    power_type: PowerType,
    observer: Option<&TristimulusObserver>,
) -> LuxResult<f64> {
    let spacing = spectrum.spacing()?;
    let wavelengths = spectrum.wavelengths();
    let values = spectrum.values();

    let power = match power_type {
        PowerType::Radiometric => values
            .iter()
            .zip(spacing.iter())
            .map(|(value, dl)| value * dl)
            .sum(),
        PowerType::Photometric => {
            let observer = observer.ok_or(LuxError::MissingObserver)?;
            let vl = observer.vl_spectrum()?.interpolate_linear(wavelengths)?;
            values
                .iter()
                .zip(spacing.iter())
                .zip(vl.values().iter())
                .map(|((value, dl), v_lambda)| observer.k * value * dl * v_lambda)
                .sum()
        }
        PowerType::Quantal => {
            let factor = 1e-9 / (PLANCK_CONSTANT * SPEED_OF_LIGHT);
            values
                .iter()
                .zip(spacing.iter())
                .zip(wavelengths.iter())
                .map(|((value, dl), wavelength)| factor * value * dl * wavelength)
                .sum()
        }
    };

    Ok(power)
}

#[cfg(test)]
mod tests {
    use super::{spd_to_ler, spd_to_power, spd_to_xyz, PowerType};
    use crate::color::Observer;
    use crate::spectrum::{SpectralMatrix, Spectrum};

    #[test]
    fn computes_radiometric_power() {
        let spectrum = Spectrum::new(vec![400.0, 410.0, 420.0], vec![1.0, 2.0, 3.0]).unwrap();
        let power = spd_to_power(&spectrum, PowerType::Radiometric, None).unwrap();
        assert_eq!(power, 60.0);
    }

    #[test]
    fn computes_quantal_power() {
        let spectrum = Spectrum::new(vec![500.0, 510.0], vec![1.0, 1.0]).unwrap();
        let power = spd_to_power(&spectrum, PowerType::Quantal, None).unwrap();
        assert!(power.is_finite());
        assert!(power > 0.0);
    }

    #[test]
    fn computes_photometric_power() {
        let observer = Observer::Cie1931_2.standard().unwrap();
        let spectrum = Spectrum::new(vec![555.0, 556.0], vec![1.0, 1.0]).unwrap();
        let power = spd_to_power(&spectrum, PowerType::Photometric, Some(&observer)).unwrap();
        assert!(power > 680.0);
    }

    #[test]
    fn photometric_power_requires_observer() {
        let spectrum = Spectrum::new(vec![555.0, 556.0], vec![1.0, 1.0]).unwrap();
        let result = spd_to_power(&spectrum, PowerType::Photometric, None);
        assert!(result.is_err());
    }

    #[test]
    fn computes_relative_xyz() {
        let observer = Observer::Cie1931_2.standard().unwrap();
        let spectrum = Spectrum::new(vec![555.0, 556.0], vec![1.0, 1.0]).unwrap();
        let xyz = spd_to_xyz(&spectrum, &observer, true).unwrap();
        assert!((xyz[0] - 52.021_027_306_606_52).abs() < 1e-9);
        assert!((xyz[1] - 100.0).abs() < 1e-12);
        assert!((xyz[2] - 0.552_719_552_355_926_3).abs() < 1e-9);
    }

    #[test]
    fn computes_relative_xyz_for_multiple_spectra() {
        let observer = Observer::Cie1931_2.standard().unwrap();
        let spectra =
            SpectralMatrix::new(vec![555.0, 556.0], vec![vec![1.0, 1.0], vec![2.0, 2.0]]).unwrap();
        let xyz = spectra.spd_to_xyz(&observer, true).unwrap();
        assert_eq!(xyz.len(), 2);
        assert!((xyz[0][0] - 52.021_027_306_606_52).abs() < 1e-9);
        assert!((xyz[1][1] - 100.0).abs() < 1e-12);
    }

    #[test]
    fn computes_ler() {
        let observer = Observer::Cie1931_2.standard().unwrap();
        let spectrum = Spectrum::new(vec![555.0, 556.0], vec![1.0, 1.0]).unwrap();
        let ler = spd_to_ler(&spectrum, &observer).unwrap();
        assert!((ler - 682.953_062_906_7).abs() < 1e-9);
    }

    #[test]
    fn computes_ler_for_multiple_spectra() {
        let observer = Observer::Cie1931_2.standard().unwrap();
        let spectra =
            SpectralMatrix::new(vec![555.0, 556.0], vec![vec![1.0, 1.0], vec![2.0, 2.0]]).unwrap();
        let ler = spectra.spd_to_ler(&observer).unwrap();
        assert_eq!(ler.len(), 2);
        assert!((ler[0] - 682.953_062_906_7).abs() < 1e-9);
        assert!((ler[1] - 682.953_062_906_7).abs() < 1e-9);
    }
}
