from pathlib import Path

import numpy as np

from baseline_common import vec_line


M_2D = np.array(
    [
        [0.4151, -0.2424, 0.0425],
        [0.1355, 0.0833, -0.0043],
        [-0.0093, 0.0125, 0.2136],
    ]
)
M_10D = np.array(
    [
        [0.4499, -0.2630, 0.0460],
        [0.1617, 0.0726, -0.0011],
        [-0.0036, 0.0054, 0.2291],
    ]
)
WL = np.arange(390.0, 781.0, 5.0)
LMS_SAMPLE_IDXS = [0, 1, 33, 46, 47, 78]
TRANS_SAMPLE_IDXS = [0, 33, 78]


def load_data(root: Path) -> tuple[np.ndarray, np.ndarray, np.ndarray]:
    data_root = root / "data" / "indvcmf"
    lmsa = np.array(_read_numeric_rows(data_root / "asano_cie2006_Alms.dat", 3))
    rmd = np.array(
        _read_numeric_rows(data_root / "asano_cie2006_RelativeMacularDensity.dat", 1)
    ).reshape(-1)
    docul = np.array(_read_numeric_rows(data_root / "asano_cie2006_docul.dat", 2))
    return lmsa.T, rmd, docul.T


def _read_numeric_rows(path: Path, expected_columns: int) -> list[list[float]]:
    rows = []
    for line in path.read_text().splitlines():
        stripped = line.strip()
        if not stripped:
            continue
        parts = [part for part in stripped.replace(",", " ").split() if part]
        values = [float(part) for part in parts]
        if len(values) > expected_columns:
            raise ValueError(f"unexpected column count in {path}: {len(values)}")
        values.extend([0.0] * (expected_columns - len(values)))
        rows.append(values)
    return rows


def lms_to_xyz_matrix(field_size: float) -> np.ndarray:
    clamped = min(max(field_size, 2.0), 10.0)
    a = (10.0 - clamped) / (10.0 - 2.0)
    return M_2D * (1.0 - a) + M_10D * a


def sample_columns(matrix: np.ndarray, idxs: list[int]) -> list[float]:
    sampled = []
    for idx in idxs:
        sampled.extend([WL[idx], *matrix[:, idx]])
    return sampled


def sample_values(values: np.ndarray, idxs: list[int]) -> list[float]:
    sampled = []
    for idx in idxs:
        sampled.extend([WL[idx], values[idx]])
    return sampled


def shift_with_linear_extrapolation(values: np.ndarray, shift_nm: float) -> np.ndarray:
    out = np.empty_like(values)
    for idx, target in enumerate(WL):
        query_wl = target - shift_nm
        if query_wl <= WL[0]:
            x0, x1 = WL[0], WL[1]
            y0, y1 = values[0], values[1]
        elif query_wl >= WL[-1]:
            x0, x1 = WL[-2], WL[-1]
            y0, y1 = values[-2], values[-1]
        else:
            right = int(np.searchsorted(WL, query_wl, side="right"))
            left = right - 1
            x0, x1 = WL[left], WL[right]
            y0, y1 = values[left], values[right]
        out[idx] = y0 + (query_wl - x0) * (y1 - y0) / (x1 - x0)
    return out


def compute_observer(
    lmsa: np.ndarray,
    rmd: np.ndarray,
    docul: np.ndarray,
    *,
    age: float,
    field_size: float,
    lens_var: float,
    macular_var: float,
    cone_var: list[float],
    cone_peak_shift: list[float] | None = None,
    allow_negative_xyz: bool = False,
) -> tuple[np.ndarray, np.ndarray, np.ndarray, np.ndarray]:
    if cone_peak_shift is None:
        cone_peak_shift = [0.0, 0.0, 0.0]

    lmsa_shifted = np.vstack(
        [
            shift_with_linear_extrapolation(lmsa[0], cone_peak_shift[0]),
            shift_with_linear_extrapolation(lmsa[1], cone_peak_shift[1]),
            shift_with_linear_extrapolation(lmsa[2], cone_peak_shift[2]),
        ]
    )

    pk_od_macula = 0.485 * np.exp(-field_size / 6.132) * (1.0 + macular_var / 100.0)
    corrected_rmd = rmd * pk_od_macula

    if age <= 60.0:
        age_scale = 1.0 + 0.02 * (age - 32.0)
    else:
        age_scale = 1.56 + 0.0667 * (age - 60.0)
    corrected_docul = (docul[0] * age_scale + docul[1]) * (1.0 + lens_var / 100.0)

    pk_od_l = (0.38 + 0.54 * np.exp(-field_size / 1.333)) * (1.0 + cone_var[0] / 100.0)
    pk_od_m = (0.38 + 0.54 * np.exp(-field_size / 1.333)) * (1.0 + cone_var[1] / 100.0)
    pk_od_s = (0.30 + 0.45 * np.exp(-field_size / 1.333)) * (1.0 + cone_var[2] / 100.0)

    alpha = np.zeros_like(lmsa_shifted)
    alpha[0] = 1.0 - np.power(10.0, -pk_od_l * np.power(10.0, lmsa_shifted[0]))
    alpha[1] = 1.0 - np.power(10.0, -pk_od_m * np.power(10.0, lmsa_shifted[1]))
    alpha[2] = 1.0 - np.power(10.0, -pk_od_s * np.power(10.0, lmsa_shifted[2]))
    alpha[2][WL >= 620.0] = 0.0

    lms = alpha * np.power(10.0, -(corrected_rmd + corrected_docul)) * WL
    lms = 100.0 * lms / np.sum(lms, axis=1, keepdims=True)

    xyz = lms_to_xyz_matrix(field_size) @ lms
    if not allow_negative_xyz:
        xyz = np.maximum(xyz, 0.0)

    lens_trans = np.power(10.0, -corrected_docul)
    macular_trans = np.power(10.0, -corrected_rmd)
    return lms, xyz, lens_trans, macular_trans


def generate_indvcmf_baselines(root: Path) -> list[tuple[str, str]]:
    lmsa, rmd, docul = load_data(root)
    default_lms, default_xyz, _, _ = compute_observer(
        lmsa,
        rmd,
        docul,
        age=32.0,
        field_size=10.0,
        lens_var=0.0,
        macular_var=0.0,
        cone_var=[0.0, 0.0, 0.0],
    )
    varied_lms, varied_xyz, varied_lens, varied_macular = compute_observer(
        lmsa,
        rmd,
        docul,
        age=60.0,
        field_size=2.0,
        lens_var=15.0,
        macular_var=-10.0,
        cone_var=[5.0, -7.0, 3.0],
    )
    shifted_lms, shifted_xyz, _, _ = compute_observer(
        lmsa,
        rmd,
        docul,
        age=45.0,
        field_size=5.0,
        lens_var=0.0,
        macular_var=0.0,
        cone_var=[0.0, 0.0, 0.0],
        cone_peak_shift=[1.5, -0.75, 0.5],
    )

    return [
        vec_line("indvcmf_matrix_2", lms_to_xyz_matrix(2.0).ravel()),
        vec_line("indvcmf_matrix_5", lms_to_xyz_matrix(5.0).ravel()),
        vec_line("indvcmf_matrix_10", lms_to_xyz_matrix(10.0).ravel()),
        vec_line("indvcmf_default_lms_samples", sample_columns(default_lms, LMS_SAMPLE_IDXS)),
        vec_line("indvcmf_default_xyz_samples", sample_columns(default_xyz, LMS_SAMPLE_IDXS)),
        vec_line("indvcmf_varied_lms_samples", sample_columns(varied_lms, LMS_SAMPLE_IDXS)),
        vec_line("indvcmf_varied_xyz_samples", sample_columns(varied_xyz, LMS_SAMPLE_IDXS)),
        vec_line(
            "indvcmf_varied_lens_samples",
            sample_values(varied_lens, TRANS_SAMPLE_IDXS),
        ),
        vec_line(
            "indvcmf_varied_macular_samples",
            sample_values(varied_macular, TRANS_SAMPLE_IDXS),
        ),
        vec_line("indvcmf_shifted_lms_samples", sample_columns(shifted_lms, LMS_SAMPLE_IDXS)),
        vec_line("indvcmf_shifted_xyz_samples", sample_columns(shifted_xyz, LMS_SAMPLE_IDXS)),
    ]


def main() -> None:
    root = Path(__file__).resolve().parents[2]
    for key, value in generate_indvcmf_baselines(root):
        print(f"{key}={value}")


if __name__ == "__main__":
    main()
