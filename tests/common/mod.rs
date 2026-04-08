#![allow(dead_code)]

use lux_rs::{Observer, Spectrum, TristimulusObserver, WavelengthGrid};

pub const WHITE_E: [f64; 3] = [0.5, 0.5, 0.5];
pub const WHITE_D65: [f64; 3] = [95.047, 100.0, 108.883];
pub const XYZ_SAMPLE: [f64; 3] = [0.25, 0.5, 0.25];
pub const XYZ_SAMPLE_ALT: [f64; 3] = [0.2, 0.3, 0.4];
pub const XYZ_BRIGHT: [f64; 3] = [20.0, 21.0, 22.0];

pub fn observer_1931() -> TristimulusObserver {
    Observer::Cie1931_2.standard().unwrap()
}

pub fn grid_360_365() -> WavelengthGrid {
    WavelengthGrid::new(360.0, 365.0, 1.0).unwrap()
}

pub fn grid_380_385() -> WavelengthGrid {
    WavelengthGrid::new(380.0, 385.0, 1.0).unwrap()
}

pub fn spectrum_400_420() -> Spectrum {
    Spectrum::new(vec![400.0, 410.0, 420.0], vec![1.0, 2.0, 3.0]).unwrap()
}

pub fn spectrum_555_556() -> Spectrum {
    Spectrum::new(vec![555.0, 556.0], vec![1.0, 1.0]).unwrap()
}

pub fn matrix_555_556() -> Spectrum {
    Spectrum::new(vec![555.0, 556.0], vec![vec![1.0, 1.0], vec![2.0, 2.0]]).unwrap()
}
