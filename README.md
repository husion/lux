# lux-rs

Pure Rust lighting and color science library for spectral, photometric, and colorimetric workflows.

## Overview

`lux-rs` provides a native Rust API for core lighting and color science calculations without requiring Python at runtime.

The crate currently includes:

- spectral foundations: wavelength grids, spacing helpers, interpolation, normalization, and single/batch spectrum models
- observers and photometry: embedded standard observers, tristimulus integration, radiometric / photometric / quantal power, and mesopic support
- illuminants and reference sources: blackbody, daylight family, CRI reference sources, and a registry for common CIE illuminants and LED series
- color kernels: CCT, common XYZ-derived transforms, color difference, and chromatic adaptation including viewing-condition and compiled-adapter workflows
- appearance models: `CIECAM02`, `CAM16`, `CAM02-UCS`, and `CAM16-UCS` forward / inverse paths plus wrapper APIs on top of the color data models
- color quality metrics: `CIE Ra`, `CIE Rf / Rg`, and structured `TM-30` result objects for single and batch spectral workflows

## Design Goals

`lux-rs` is designed for:

- predictable native deployment
- easier integration into Rust systems
- clearer data ownership and API design
- parity testing against an existing scientific reference implementation

## Verification

The crate is validated with Rust tests and parity checks against [`luxpy`](https://github.com/ksmet1977/luxpy). Covered paths include:

- spectral grid helpers
- interpolation and normalization
- observer and CMF access
- `spd_to_power`, `spd_to_ler`, `spd_to_xyz`
- `blackbody`, `daylightphase`, `cri_ref`
- `xyz_to_cct`, `cct_to_xyz`
- standard illuminants
- one-step and two-step `CAT`
- `CIECAM02`, `CAM16`, `CAM02-UCS`, and `CAM16-UCS`
- `CIE Ra`
- `CIE Rf / Rg`
- `TM-30` result objects

## Install

```bash
cargo add lux-rs
```

## Quick Example

```rust
use lux_rs::{spd_to_ler, spd_to_xyz, standard_illuminant, xyz_to_cct, Observer};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let observer = Observer::Cie1931_2.standard()?;
    let d65 = standard_illuminant("D65", None)?;

    let xyz = spd_to_xyz(&d65, &observer, true)?;
    let ler = spd_to_ler(&d65, &observer)?;
    let (cct, duv) = xyz_to_cct(xyz, Observer::Cie1931_2)?;

    println!("XYZ: {:?}", xyz);
    println!("LER: {:.3} lm/W", ler);
    println!("CCT: {:.1} K, Duv: {:.6}", cct, duv);
    Ok(())
}
```

For spectral workflows, use `Spectrum` and `SpectralMatrix` as the primary API.
For tristimulus and color workflows, use `Tristimulus` and `TristimulusSet`.

## Relationship To LuxPy

[`luxpy`](https://github.com/ksmet1977/luxpy) is a comprehensive Python toolbox for lighting and color science. `lux-rs` is not a binding layer around Python; it is a native Rust implementation that draws on the same problem domain and uses LuxPy for parity-oriented validation during development.

Scope difference, in short:

- `luxpy`: broad toolbox including CAM, CAT, CRI/TM-30, photobiology, hyperspectral imaging, instrument/toolbox integrations, and more
- `lux-rs`: focused on spectral kernels, observers, integration, reference illuminants, photometry, CCT, color transforms, CAM, and CRI/TM-30 core workflows

That means `lux-rs` is narrower in scope than `luxpy`, while still being suitable for a meaningful subset of core numerical workflows.

## Citing LuxPy

If this repository or its design work benefits from `luxpy`, please cite the original `luxpy` project and tutorial paper.

Recommended citation from the upstream [`luxpy` README](https://github.com/ksmet1977/luxpy/blob/master/README.md):

> Smet, K. A. G. (2020). Tutorial: The LuxPy Python Toolbox for Lighting and Color Science. LEUKOS, 1-23. https://doi.org/10.1080/15502724.2018.1518717

Useful upstream references:

- LuxPy repository: <https://github.com/ksmet1977/luxpy>
- LuxPy tutorial paper: <https://www.tandfonline.com/doi/full/10.1080/15502724.2018.1518717>
- LuxPy Zenodo DOI: <https://doi.org/10.5281/zenodo.1298963>

## License

This crate is licensed under `GPL-3.0-only`. See [`Cargo.toml`](./Cargo.toml) and the repository license terms for details.
