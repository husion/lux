import luxpy as lx
import numpy as np

from baseline_common import scalar_line, usize_vec_line, vec_line


def generate_observer_baselines() -> list[tuple[str, str]]:
    xyzbar_1931 = lx.xyzbar(cieobs="1931_2")
    vlbar_1931, k_1931 = lx.vlbar(cieobs="1931_2", out=2)
    xyzbar_1964 = lx.xyzbar(cieobs="1964_10")
    vlbar_1964, k_1964 = lx.vlbar(cieobs="1964_10", out=2)
    lmes_1, m_1 = lx.get_cie_mesopic_adaptation(1.0, SP=1.0)
    vlbar_mesopic, k_mesopic = lx.vlbar_cie_mesopic(
        m=[0.5, 1.0],
        wl_new=np.array([555.0]),
        out=2,
    )

    return [
        usize_vec_line("xyzbar_1931_shape", [xyzbar_1931.shape[0], xyzbar_1931.shape[1]]),
        vec_line(
            "xyzbar_1931_555",
            xyzbar_1931[:, xyzbar_1931[0] == 555.0].ravel(),
        ),
        usize_vec_line("vlbar_1931_shape", [vlbar_1931.shape[0], vlbar_1931.shape[1]]),
        vec_line(
            "vlbar_1931_555",
            vlbar_1931[:, vlbar_1931[0] == 555.0].ravel(),
        ),
        scalar_line("vlbar_1931_k", k_1931),
        vec_line(
            "xyzbar_1964_555",
            xyzbar_1964[:, xyzbar_1964[0] == 555.0].ravel(),
        ),
        vec_line(
            "vlbar_1964_555",
            vlbar_1964[:, vlbar_1964[0] == 555.0].ravel(),
        ),
        scalar_line("vlbar_1964_k", k_1964),
        scalar_line("mesopic_lmes_sp_1", np.ravel(lmes_1)[0]),
        scalar_line("mesopic_m_sp_1", np.ravel(m_1)[0]),
        usize_vec_line("mesopic_vlbar_shape", [vlbar_mesopic.shape[0], vlbar_mesopic.shape[1]]),
        vec_line("mesopic_vlbar_555", vlbar_mesopic.ravel()),
        vec_line("mesopic_k", np.ravel(k_mesopic)),
    ]
