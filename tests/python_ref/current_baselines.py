import luxpy as lx
import numpy as np


def fmt_scalar(value: float) -> str:
    return repr(float(value))


def fmt_vec(values) -> str:
    return ",".join(repr(float(value)) for value in values)


def main() -> None:
    xyzbar_1931 = lx.xyzbar(cieobs="1931_2")
    vlbar_1931, k_1931 = lx.vlbar(cieobs="1931_2", out=2)
    xyzbar_1964 = lx.xyzbar(cieobs="1964_10")
    vlbar_1964, k_1964 = lx.vlbar(cieobs="1964_10", out=2)

    print(f"xyzbar_1931_shape={xyzbar_1931.shape[0]},{xyzbar_1931.shape[1]}")
    print(f"xyzbar_1931_555={fmt_vec(xyzbar_1931[:, xyzbar_1931[0] == 555.0].ravel())}")
    print(f"vlbar_1931_shape={vlbar_1931.shape[0]},{vlbar_1931.shape[1]}")
    print(f"vlbar_1931_555={fmt_vec(vlbar_1931[:, vlbar_1931[0] == 555.0].ravel())}")
    print(f"vlbar_1931_k={fmt_scalar(k_1931)}")
    print(f"xyzbar_1964_555={fmt_vec(xyzbar_1964[:, xyzbar_1964[0] == 555.0].ravel())}")
    print(f"vlbar_1964_555={fmt_vec(vlbar_1964[:, vlbar_1964[0] == 555.0].ravel())}")
    print(f"vlbar_1964_k={fmt_scalar(k_1964)}")
    print(
        "xyzbar_1931_interp="
        + fmt_vec(
            lx.xyzbar(
                cieobs="1931_2",
                wl_new=np.array([554.5, 555.0, 555.5, 556.0]),
                kind="linear",
                extrap_kind="linear",
            ).ravel()
        )
    )
    print(
        "vlbar_1931_interp="
        + fmt_vec(
            lx.vlbar(
                cieobs="1931_2",
                wl_new=np.array([554.5, 555.0, 555.5, 556.0]),
                kind="linear",
                extrap_kind="linear",
            ).ravel()
        )
    )
    print(
        "spd_interp="
        + fmt_vec(
            lx.cie_interp(
                np.array([[400.0, 410.0, 420.0], [1.0, 2.0, 3.0]]),
                np.array([395.0, 405.0, 420.0, 425.0]),
                kind="linear",
                extrap_kind="linear",
            ).ravel()
        )
    )
    print(
        "normalize_max="
        + fmt_vec(
            lx.spd_normalize(
                np.array([[400.0, 410.0, 420.0], [1.0, 2.0, 3.0]]),
                norm_type="max",
                norm_f=2.0,
            ).ravel()
        )
    )
    print(
        "normalize_area="
        + fmt_vec(
            lx.spd_normalize(
                np.array([[400.0, 410.0, 420.0], [1.0, 2.0, 3.0]]),
                norm_type="area",
                norm_f=1.0,
            ).ravel()
        )
    )
    print(
        "normalize_lambda="
        + fmt_vec(
            lx.spd_normalize(
                np.array([[400.0, 410.0, 420.0], [1.0, 2.0, 3.0]]),
                norm_type="lambda",
                norm_f=410.0,
            ).ravel()
        )
    )
    print(
        "normalize_ru="
        + fmt_vec(
            lx.spd_normalize(
                np.array([[400.0, 410.0, 420.0], [1.0, 2.0, 3.0]]),
                norm_type="ru",
                norm_f=10.0,
            ).ravel()
        )
    )
    print(
        "normalize_pu="
        + fmt_vec(
            lx.spd_normalize(
                np.array([[555.0, 556.0], [1.0, 1.0]]),
                norm_type="pu",
                norm_f=1000.0,
                cieobs="1931_2",
            ).ravel()
        )
    )
    print(
        "normalize_qu="
        + fmt_vec(
            lx.spd_normalize(
                np.array([[500.0, 510.0], [1.0, 1.0]]),
                norm_type="qu",
                norm_f=1e18,
            ).ravel()
        )
    )

    print(f"getwlr={fmt_vec(lx.getwlr([360, 365, 1]))}")
    print(f"getwld_equal_scalar={fmt_scalar(lx.getwld(np.array([400.0, 410.0, 420.0])))}")
    print(f"getwld_unequal={fmt_vec(lx.getwld(np.array([400.0, 410.0, 430.0])))}")
    print(
        "power_ru="
        + fmt_scalar(lx.spd_to_power(np.array([[400.0, 410.0, 420.0], [1.0, 2.0, 3.0]]), "ru")[0, 0])
    )
    print(
        "power_pu="
        + fmt_scalar(
            lx.spd_to_power(
                np.array([[555.0, 556.0], [1.0, 1.0]]),
                "pu",
                cieobs="1931_2",
            )[0, 0]
        )
    )
    print(
        "power_qu="
        + fmt_scalar(lx.spd_to_power(np.array([[500.0, 510.0], [1.0, 1.0]]), "qu")[0, 0])
    )
    print(
        "ler_1931="
        + fmt_scalar(lx.spd_to_ler(np.array([[555.0, 556.0], [1.0, 1.0]]), cieobs="1931_2")[0, 0])
    )
    print(
        "ler_1964="
        + fmt_scalar(lx.spd_to_ler(np.array([[555.0, 556.0], [1.0, 1.0]]), cieobs="1964_10")[0, 0])
    )
    print(
        "ler_many_1931="
        + fmt_vec(
            lx.spd_to_ler(np.array([[555.0, 556.0], [1.0, 1.0], [2.0, 2.0]]), cieobs="1931_2").ravel()
        )
    )
    print(
        "xyz_relative="
        + fmt_vec(lx.spd_to_xyz(np.array([[555.0, 556.0], [1.0, 1.0]]), cieobs="1931_2")[0])
    )
    print(
        "xyz_absolute="
        + fmt_vec(
            lx.spd_to_xyz(
                np.array([[555.0, 556.0], [1.0, 1.0]]),
                cieobs="1931_2",
                relative=False,
            )[0]
        )
    )
    print(
        "xyz_relative_1964_10="
        + fmt_vec(lx.spd_to_xyz(np.array([[555.0, 556.0], [1.0, 1.0]]), cieobs="1964_10")[0])
    )
    print(
        "xyz_relative_many="
        + fmt_vec(
            lx.spd_to_xyz(np.array([[555.0, 556.0], [1.0, 1.0], [2.0, 2.0]]), cieobs="1931_2").ravel()
        )
    )
    print(
        "xyz_absolute_many="
        + fmt_vec(
            lx.spd_to_xyz(
                np.array([[555.0, 556.0], [1.0, 1.0], [2.0, 2.0]]),
                cieobs="1931_2",
                relative=False,
            ).ravel()
        )
    )


if __name__ == "__main__":
    main()
