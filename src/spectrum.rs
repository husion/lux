use crate::error::{LuxError, LuxResult};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct WavelengthGrid {
    pub start: f64,
    pub end: f64,
    pub step: f64,
}

impl WavelengthGrid {
    pub fn new(start: f64, end: f64, step: f64) -> LuxResult<Self> {
        if !start.is_finite()
            || !end.is_finite()
            || !step.is_finite()
            || step <= 0.0
            || end < start
        {
            return Err(LuxError::InvalidGridSpec);
        }
        Ok(Self { start, end, step })
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Spectrum {
    wavelengths: Vec<f64>,
    values: Vec<f64>,
}

impl Spectrum {
    pub fn new(wavelengths: Vec<f64>, values: Vec<f64>) -> LuxResult<Self> {
        if wavelengths.is_empty() || values.is_empty() {
            return Err(LuxError::EmptyInput);
        }
        if wavelengths.len() != values.len() {
            return Err(LuxError::MismatchedLengths {
                wavelengths: wavelengths.len(),
                values: values.len(),
            });
        }
        if wavelengths
            .windows(2)
            .any(|pair| !(pair[1].is_finite() && pair[0].is_finite() && pair[1] > pair[0]))
        {
            return Err(LuxError::NonMonotonicWavelengths);
        }
        Ok(Self {
            wavelengths,
            values,
        })
    }

    pub fn wavelengths(&self) -> &[f64] {
        &self.wavelengths
    }

    pub fn values(&self) -> &[f64] {
        &self.values
    }

    pub fn spacing(&self) -> LuxResult<Vec<f64>> {
        getwld(&self.wavelengths)
    }

    pub fn interpolate_linear(&self, target_wavelengths: &[f64]) -> LuxResult<Self> {
        if target_wavelengths.is_empty() {
            return Err(LuxError::EmptyInput);
        }
        if target_wavelengths
            .windows(2)
            .any(|pair| !(pair[1].is_finite() && pair[0].is_finite() && pair[1] > pair[0]))
        {
            return Err(LuxError::NonMonotonicWavelengths);
        }

        let mut values = Vec::with_capacity(target_wavelengths.len());
        for &target in target_wavelengths {
            values.push(self.interpolate_one_linear(target));
        }
        Spectrum::new(target_wavelengths.to_vec(), values)
    }

    fn interpolate_one_linear(&self, target: f64) -> f64 {
        let wavelengths = &self.wavelengths;
        let values = &self.values;

        if target <= wavelengths[0] {
            return linear_segment(
                wavelengths[0],
                values[0],
                wavelengths[1],
                values[1],
                target,
            );
        }
        if target >= wavelengths[wavelengths.len() - 1] {
            let last = wavelengths.len() - 1;
            return linear_segment(
                wavelengths[last - 1],
                values[last - 1],
                wavelengths[last],
                values[last],
                target,
            );
        }

        let idx = wavelengths.partition_point(|wavelength| *wavelength < target);
        if wavelengths[idx] == target {
            values[idx]
        } else {
            linear_segment(
                wavelengths[idx - 1],
                values[idx - 1],
                wavelengths[idx],
                values[idx],
                target,
            )
        }
    }
}

fn linear_segment(x0: f64, y0: f64, x1: f64, y1: f64, x: f64) -> f64 {
    y0 + (y1 - y0) * ((x - x0) / (x1 - x0))
}

pub fn getwlr(grid: WavelengthGrid) -> LuxResult<Vec<f64>> {
    let mut wavelengths = Vec::new();
    let mut current = grid.start;
    let epsilon = grid.step.abs() * 1e-9;

    while current <= grid.end + epsilon {
        wavelengths.push(current);
        current += grid.step;
    }

    if wavelengths.is_empty() {
        return Err(LuxError::InvalidGridSpec);
    }

    Ok(wavelengths)
}

pub fn getwld(wavelengths: &[f64]) -> LuxResult<Vec<f64>> {
    if wavelengths.is_empty() {
        return Err(LuxError::EmptyInput);
    }
    if wavelengths.len() == 1 {
        return Ok(vec![0.0]);
    }
    if wavelengths
        .windows(2)
        .any(|pair| !(pair[1].is_finite() && pair[0].is_finite() && pair[1] > pair[0]))
    {
        return Err(LuxError::NonMonotonicWavelengths);
    }

    let diffs: Vec<f64> = wavelengths.windows(2).map(|pair| pair[1] - pair[0]).collect();
    let mut spacing = Vec::with_capacity(wavelengths.len());
    spacing.push(diffs[0]);
    for idx in 1..wavelengths.len() - 1 {
        spacing.push((diffs[idx - 1] + diffs[idx]) / 2.0);
    }
    spacing.push(*diffs.last().unwrap());
    Ok(spacing)
}

#[cfg(test)]
mod tests {
    use super::{getwld, getwlr, Spectrum, WavelengthGrid};

    #[test]
    fn grid_matches_luxpy_style_range() {
        let wl = getwlr(WavelengthGrid::new(360.0, 365.0, 1.0).unwrap()).unwrap();
        assert_eq!(wl, vec![360.0, 361.0, 362.0, 363.0, 364.0, 365.0]);
    }

    #[test]
    fn unequal_spacing_matches_luxpy_formula() {
        let dl = getwld(&[400.0, 410.0, 430.0]).unwrap();
        assert_eq!(dl, vec![10.0, 15.0, 20.0]);
    }

    #[test]
    fn validates_spectrum_lengths() {
        let result = Spectrum::new(vec![400.0], vec![1.0, 2.0]);
        assert!(result.is_err());
    }

    #[test]
    fn linearly_interpolates_and_extrapolates() {
        let spectrum = Spectrum::new(vec![400.0, 410.0, 420.0], vec![1.0, 2.0, 3.0]).unwrap();
        let resampled = spectrum
            .interpolate_linear(&[395.0, 405.0, 420.0, 425.0])
            .unwrap();
        assert_eq!(resampled.values(), &[0.5, 1.5, 3.0, 3.5]);
    }
}
