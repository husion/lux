pub mod color;
pub mod error;
pub mod illuminants;
pub mod photometry;
pub mod spectrum;

#[allow(deprecated)]
pub use color::{
    cat_apply, cat_apply_mode, cat_apply_with_conditions, cat_degree_of_adaptation, delta_e,
    delta_e_cie76, delta_e_ciede2000, get_cie_mesopic_adaptation, lab_to_xyz, lms_to_xyz,
    lms_to_xyz_with_matrix, luv_to_xyz, srgb_to_xyz, vlbar_cie_mesopic, xyz_to_lab, xyz_to_lms,
    xyz_to_lms_with_matrix, xyz_to_luv, xyz_to_srgb, xyz_to_yuv, xyz_to_yxy, yuv_to_xyz,
    yxy_to_xyz, CatMode, CatSurround, CatTransform,
    DeltaEFormula, Matrix3, MesopicLuminousEfficiency, Observer, Tristimulus,
    TristimulusObserver, TristimulusSet,
};
pub use error::{LuxError, LuxResult};
pub use illuminants::{
    blackbody, cct_to_xyz, cri_ref, daylightlocus, daylightphase, standard_illuminant,
    standard_illuminant_names, xyz_to_cct,
};
pub use photometry::{
    spd_to_ler, spd_to_power, spd_to_xyz, PowerType,
};
pub use spectrum::{
    getwld, getwlr, SpectralMatrix, Spectrum, SpectrumNormalization, WavelengthGrid,
};
