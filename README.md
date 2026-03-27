# lux

`lux` is a standalone Rust library for lighting and color science calculations.

This crate is being built as a pure Rust rewrite path for LuxPy's computational
core. It does not call Python code at runtime.

Current scope:

- wavelength grid generation compatible with LuxPy `getwlr`
- wavelength spacing calculation compatible with LuxPy `getwld`
- `Spectrum` data model
- linear interpolation with linear extrapolation
- SPD power integration for:
  - radiometric units
  - photometric units using embedded standard observers
  - quantal units
- embedded CIE standard observers:
  - `1931_2`
  - `1964_10`
- CIE 191:2010 mesopic support:
  - `get_cie_mesopic_adaptation`
  - `vlbar_cie_mesopic`
- color transforms:
  - `XYZ <-> Yxy`
  - `XYZ <-> Yuv`
  - `XYZ <-> Lab` (explicit white point)
  - `XYZ <-> Luv` (explicit white point)
  - `XYZ <-> LMS` (observer/default matrix or explicit matrix)
  - `XYZ <-> sRGB`

Short example:

```rust
use lux::{spd_to_power, Observer, PowerType, Spectrum};

let observer = Observer::Cie1931_2.standard()?;
let spectrum = Spectrum::new(vec![555.0, 556.0], vec![1.0, 1.0])?;
let lumens = spd_to_power(&spectrum, PowerType::Photometric, Some(&observer))?;
```

Planned next:

- reflectance and illuminant datasets
- CCT and chromaticity transforms
- batch spectral matrices
- CLI and optional Python bindings as thin wrappers on top of Rust
