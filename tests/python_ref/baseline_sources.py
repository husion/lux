from pathlib import Path

import luxpy as lx
import numpy as np

from baseline_common import scalar_line, vec_line


def generate_source_baselines(root: Path) -> list[tuple[str, str]]:
    d65 = np.loadtxt(root / "data" / "spds" / "CIE_D65.csv", delimiter=",").T
    f_series = np.loadtxt(root / "data" / "spds" / "CIE_F_1to12_1nm.csv", delimiter=",")
    f4 = np.vstack((f_series[:, 0], f_series[:, 4]))
    x_d, y_d = lx.daylightlocus(6500.0)
    cct_sample, duv_sample = lx.xyz_to_cct(np.array([[100.0, 100.0, 100.0]]), out="cct,duv")

    return [
        vec_line(
            "blackbody_relative_6500",
            lx.blackbody(6500.0, wl3=[360.0, 365.0, 1.0], relative=True).ravel(),
        ),
        vec_line(
            "blackbody_absolute_6500_560",
            lx.blackbody(6500.0, wl3=[560.0, 560.0, 1.0], relative=False).ravel(),
        ),
        vec_line("daylightlocus_6500", [x_d, y_d]),
        vec_line(
            "daylightphase_6500",
            lx.daylightphase(6500.0, wl3=[360.0, 365.0, 1.0]).ravel(),
        ),
        vec_line(
            "daylightphase_3500",
            lx.daylightphase(3500.0, wl3=[360.0, 365.0, 1.0]).ravel(),
        ),
        scalar_line("ciera_d65", lx.cri.spd_to_ciera(d65).ravel()[0]),
        vec_line("ciera_d65_ri", lx.cri.spd_to_ciera(d65, out="Rfi").ravel()),
        scalar_line("ciera_f4", lx.cri.spd_to_ciera(f4).ravel()[0]),
        vec_line("ciera_f4_ri", lx.cri.spd_to_ciera(f4, out="Rfi").ravel()),
        scalar_line("cierf_d65", lx.cri.spd_to_cierf(d65).ravel()[0]),
        vec_line("cierf_d65_rfi", lx.cri.spd_to_cierf(d65, out="Rfi").ravel()),
        scalar_line("cierg_d65", lx.cri.spd_to_cri(d65, cri_type="cierf", out="Rg").ravel()[0]),
        scalar_line("cierf_f4", lx.cri.spd_to_cierf(f4).ravel()[0]),
        vec_line("cierf_f4_rfi", lx.cri.spd_to_cierf(f4, out="Rfi").ravel()),
        scalar_line("cierg_f4", lx.cri.spd_to_cri(f4, cri_type="cierf", out="Rg").ravel()[0]),
        vec_line(
            "cri_ref_3000_6500",
            lx.cri_ref([3000.0, 6500.0], wl3=[360.0, 365.0, 1.0]).ravel(),
        ),
        vec_line(
            "xyz_to_cct_sample",
            [np.ravel(cct_sample)[0], np.ravel(duv_sample)[0]],
        ),
        vec_line("cct_to_xyz_6500", lx.cct_to_xyz(6500.0).ravel()),
        vec_line("illuminant_A", lx._CIE_ILLUMINANTS["A"][:, 0:6].ravel()),
        vec_line("illuminant_D65", lx._CIE_ILLUMINANTS["D65"][:, 0:6].ravel()),
        vec_line("illuminant_F4", lx._CIE_ILLUMINANTS["F4"][:, 0:6].ravel()),
        vec_line("illuminant_LED_B1", lx._CIE_ILLUMINANTS["LED_B1"][:, 0:6].ravel()),
        vec_line(
            "illuminant_D50",
            lx.daylightphase(5000.0, wl3=[360.0, 365.0, 1.0], cct_is_nominal=True).ravel(),
        ),
    ]
