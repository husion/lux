import luxpy as lx
import numpy as np
from luxpy.toolboxes import spectral_mismatch_and_uncertainty as smu

from baseline_common import usize_vec_line, vec_line


def generate_spectral_mismatch_baselines() -> list[tuple[str, str]]:
    detectors = lx.xyzbar(cieobs="1931_2")[[0, 1, 2, 3]]
    f1prime_xyz = smu.f1prime(detectors, S_C="A", cieobs="1931_2", s_target_index=2)
    measured_sources = np.vstack((lx._CIE_ILLUMINANTS["D65"], lx._CIE_ILLUMINANTS["A"][1:]))
    factors = smu.get_spectral_mismatch_correction_factors(
        measured_sources,
        detectors,
        S_C="A",
        cieobs="1931_2",
        s_target_index=2,
    )

    return [
        usize_vec_line("spectral_mismatch_f1prime_shape", [len(f1prime_xyz)]),
        vec_line("spectral_mismatch_f1prime_xyz", np.ravel(f1prime_xyz)),
        usize_vec_line("spectral_mismatch_factors_shape", factors.shape),
        vec_line("spectral_mismatch_factors_d65_a_xyz", factors.ravel()),
    ]


def main() -> None:
    for key, value in generate_spectral_mismatch_baselines():
        print(f"{key}={value}")


if __name__ == "__main__":
    main()
