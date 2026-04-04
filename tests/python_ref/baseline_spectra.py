import luxpy as lx
import numpy as np

from baseline_common import scalar_line, vec_line


def generate_spectral_baselines() -> list[tuple[str, str]]:
    return [
        vec_line(
            "xyzbar_1931_interp",
            lx.xyzbar(
                cieobs="1931_2",
                wl_new=np.array([554.5, 555.0, 555.5, 556.0]),
                kind="linear",
                extrap_kind="linear",
            ).ravel(),
        ),
        vec_line(
            "vlbar_1931_interp",
            lx.vlbar(
                cieobs="1931_2",
                wl_new=np.array([554.5, 555.0, 555.5, 556.0]),
                kind="linear",
                extrap_kind="linear",
            ).ravel(),
        ),
        vec_line(
            "spd_interp",
            lx.cie_interp(
                np.array([[400.0, 410.0, 420.0], [1.0, 2.0, 3.0]]),
                np.array([395.0, 405.0, 420.0, 425.0]),
                kind="linear",
                extrap_kind="linear",
            ).ravel(),
        ),
        vec_line(
            "normalize_max",
            lx.spd_normalize(
                np.array([[400.0, 410.0, 420.0], [1.0, 2.0, 3.0]]),
                norm_type="max",
                norm_f=2.0,
            ).ravel(),
        ),
        vec_line(
            "normalize_area",
            lx.spd_normalize(
                np.array([[400.0, 410.0, 420.0], [1.0, 2.0, 3.0]]),
                norm_type="area",
                norm_f=1.0,
            ).ravel(),
        ),
        vec_line(
            "normalize_lambda",
            lx.spd_normalize(
                np.array([[400.0, 410.0, 420.0], [1.0, 2.0, 3.0]]),
                norm_type="lambda",
                norm_f=410.0,
            ).ravel(),
        ),
        vec_line(
            "normalize_ru",
            lx.spd_normalize(
                np.array([[400.0, 410.0, 420.0], [1.0, 2.0, 3.0]]),
                norm_type="ru",
                norm_f=10.0,
            ).ravel(),
        ),
        vec_line(
            "normalize_pu",
            lx.spd_normalize(
                np.array([[555.0, 556.0], [1.0, 1.0]]),
                norm_type="pu",
                norm_f=1000.0,
                cieobs="1931_2",
            ).ravel(),
        ),
        vec_line(
            "normalize_qu",
            lx.spd_normalize(
                np.array([[500.0, 510.0], [1.0, 1.0]]),
                norm_type="qu",
                norm_f=1e18,
            ).ravel(),
        ),
        vec_line("getwlr", lx.getwlr([360, 365, 1])),
        scalar_line("getwld_equal_scalar", lx.getwld(np.array([400.0, 410.0, 420.0]))),
        vec_line("getwld_unequal", lx.getwld(np.array([400.0, 410.0, 430.0]))),
        scalar_line(
            "power_ru",
            lx.spd_to_power(np.array([[400.0, 410.0, 420.0], [1.0, 2.0, 3.0]]), "ru")[0, 0],
        ),
        scalar_line(
            "power_pu",
            lx.spd_to_power(
                np.array([[555.0, 556.0], [1.0, 1.0]]),
                "pu",
                cieobs="1931_2",
            )[0, 0],
        ),
        scalar_line(
            "power_qu",
            lx.spd_to_power(np.array([[500.0, 510.0], [1.0, 1.0]]), "qu")[0, 0],
        ),
        scalar_line(
            "ler_1931",
            lx.spd_to_ler(np.array([[555.0, 556.0], [1.0, 1.0]]), cieobs="1931_2")[0, 0],
        ),
        scalar_line(
            "ler_1964",
            lx.spd_to_ler(np.array([[555.0, 556.0], [1.0, 1.0]]), cieobs="1964_10")[0, 0],
        ),
        vec_line(
            "ler_many_1931",
            lx.spd_to_ler(
                np.array([[555.0, 556.0], [1.0, 1.0], [2.0, 2.0]]),
                cieobs="1931_2",
            ).ravel(),
        ),
        vec_line(
            "xyz_relative",
            lx.spd_to_xyz(np.array([[555.0, 556.0], [1.0, 1.0]]), cieobs="1931_2")[0],
        ),
        vec_line(
            "xyz_absolute",
            lx.spd_to_xyz(
                np.array([[555.0, 556.0], [1.0, 1.0]]),
                cieobs="1931_2",
                relative=False,
            )[0],
        ),
        vec_line(
            "xyz_relative_1964_10",
            lx.spd_to_xyz(np.array([[555.0, 556.0], [1.0, 1.0]]), cieobs="1964_10")[0],
        ),
        vec_line(
            "xyz_relative_many",
            lx.spd_to_xyz(
                np.array([[555.0, 556.0], [1.0, 1.0], [2.0, 2.0]]),
                cieobs="1931_2",
            ).ravel(),
        ),
        vec_line(
            "xyz_absolute_many",
            lx.spd_to_xyz(
                np.array([[555.0, 556.0], [1.0, 1.0], [2.0, 2.0]]),
                cieobs="1931_2",
                relative=False,
            ).ravel(),
        ),
    ]
