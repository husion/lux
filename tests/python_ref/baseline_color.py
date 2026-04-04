import luxpy as lx
import numpy as np
from luxpy.color import cat
from luxpy.color.cam.cam02ucs import run as cam02ucs
from luxpy.color.cam.cam16ucs import run as cam16ucs
from luxpy.color.cam.ciecam02 import run as ciecam02
from luxpy.color.cam.ciecam16 import run as ciecam16
from luxpy.color.deltaE import DE2000

from baseline_common import scalar_line, vec_line


def generate_color_baselines() -> list[tuple[str, str]]:
    xyz_sample = np.array([[0.25, 0.5, 0.25]])
    white_sample = np.array([[0.5, 0.5, 0.5]])
    yxy_sample = lx.xyz_to_Yxy(xyz_sample)
    yuv_sample = lx.xyz_to_Yuv(xyz_sample)
    lab_sample = lx.xyz_to_lab(xyz_sample, xyzw=white_sample)
    luv_sample = lx.xyz_to_luv(xyz_sample, xyzw=white_sample)
    lms_1931 = lx.xyz_to_lms(xyz_sample, cieobs="1931_2")
    lms_1964 = lx.xyz_to_lms(xyz_sample, cieobs="1964_10")
    srgb_sample = lx.xyz_to_srgb(np.array([[20.0, 21.0, 22.0]]))

    white_d65 = np.array([[95.047, 100.0, 108.883]])
    delta_xyz1_cie76 = lx.lab_to_xyz(np.array([[50.0, 2.5, -80.0]]), xyzw=white_d65)
    delta_xyz2_cie76 = lx.lab_to_xyz(np.array([[50.0, 0.0, -82.5]]), xyzw=white_d65)
    delta_xyz1_ciede2000 = lx.lab_to_xyz(np.array([[50.0, 2.6772, -79.7751]]), xyzw=white_d65)
    delta_xyz2_ciede2000 = lx.lab_to_xyz(np.array([[50.0, 0.0, -82.7485]]), xyzw=white_d65)

    cat_xyz = np.array([[19.01, 20.0, 21.78]])
    cat_w1 = np.array([[95.047, 100.0, 108.883]])
    cat_w2 = np.array([[109.85, 100.0, 35.585]])
    cat_d_avg = cat.get_degree_of_adaptation(Dtype="cat02", F="avg", La=318.31)[0]
    cat_d_dim = cat.get_degree_of_adaptation(Dtype="cat16", F="dim", La=20.0)[0]
    cat_d_dark = cat.get_degree_of_adaptation(Dtype="cat16", F="dark", La=0.0)[0]
    cam_conditions = {"La": 100.0, "Yb": 20.0, "surround": "avg", "D": 1.0, "Dtype": None}

    return [
        vec_line("xyz_to_yxy", yxy_sample.ravel()),
        vec_line("yxy_to_xyz", lx.Yxy_to_xyz(yxy_sample).ravel()),
        vec_line("xyz_to_yuv", yuv_sample.ravel()),
        vec_line("yuv_to_xyz", lx.Yuv_to_xyz(yuv_sample).ravel()),
        vec_line("xyz_to_lab", lab_sample.ravel()),
        vec_line("lab_to_xyz", lx.lab_to_xyz(lab_sample, xyzw=white_sample).ravel()),
        vec_line("xyz_to_luv", luv_sample.ravel()),
        vec_line("luv_to_xyz", lx.luv_to_xyz(luv_sample, xyzw=white_sample).ravel()),
        scalar_line(
            "delta_e_cie76",
            np.linalg.norm(
                lx.xyz_to_lab(delta_xyz1_cie76, xyzw=white_d65).ravel()
                - lx.xyz_to_lab(delta_xyz2_cie76, xyzw=white_d65).ravel()
            ),
        ),
        scalar_line(
            "delta_e_ciede2000",
            DE2000(
                delta_xyz1_ciede2000,
                delta_xyz2_ciede2000,
                dtype="xyz",
                xyzwt=white_d65,
                xyzwr=white_d65,
            ).ravel()[0],
        ),
        vec_line("xyz_to_lms_1931", lms_1931.ravel()),
        vec_line("lms_to_xyz_1931", lx.lms_to_xyz(lms_1931, cieobs="1931_2").ravel()),
        vec_line("xyz_to_lms_1964", lms_1964.ravel()),
        scalar_line("cat_d_avg", cat_d_avg),
        scalar_line("cat_d_dim", cat_d_dim),
        scalar_line("cat_d_dark", cat_d_dark),
        vec_line("cat_bradford", cat.apply_vonkries1(cat_xyz, xyzw1=cat_w1, xyzw2=cat_w2, mcat="bfd").ravel()),
        vec_line("cat_cat02", cat.apply_vonkries1(cat_xyz, xyzw1=cat_w1, xyzw2=cat_w2, mcat="cat02").ravel()),
        vec_line("cat_cat16", cat.apply_vonkries1(cat_xyz, xyzw1=cat_w1, xyzw2=cat_w2, mcat="cat16").ravel()),
        vec_line("cat_sharp", cat.apply_vonkries1(cat_xyz, xyzw1=cat_w1, xyzw2=cat_w2, mcat="sharp").ravel()),
        vec_line("cat_bianco", cat.apply_vonkries1(cat_xyz, xyzw1=cat_w1, xyzw2=cat_w2, mcat="bianco").ravel()),
        vec_line("cat_cmc", cat.apply_vonkries1(cat_xyz, xyzw1=cat_w1, xyzw2=cat_w2, mcat="cmc").ravel()),
        vec_line("cat_kries", cat.apply_vonkries1(cat_xyz, xyzw1=cat_w1, xyzw2=cat_w2, mcat="kries").ravel()),
        vec_line("cat_judd1945", cat.apply_vonkries1(cat_xyz, xyzw1=cat_w1, xyzw2=cat_w2, mcat="judd-1945").ravel()),
        vec_line(
            "cat_judd1945_cie016",
            cat.apply_vonkries1(cat_xyz, xyzw1=cat_w1, xyzw2=cat_w2, mcat="judd-1945-CIE016").ravel(),
        ),
        vec_line("cat_judd1935", cat.apply_vonkries1(cat_xyz, xyzw1=cat_w1, xyzw2=cat_w2, mcat="judd-1935").ravel()),
        vec_line(
            "cat_bradford_avg",
            cat.apply_vonkries1(cat_xyz, xyzw1=cat_w1, xyzw2=cat_w2, mcat="bfd", D=cat_d_avg).ravel(),
        ),
        vec_line(
            "cat_two_step_bradford",
            cat.apply_vonkries(
                cat_xyz,
                cat_w1,
                cat_w2,
                xyzw0=np.array([[100.0, 100.0, 100.0]]),
                D=[0.8, 0.6],
                mcat="bfd",
                catmode="1>0>2",
            ).ravel(),
        ),
        vec_line(
            "cat_two_step_cat16",
            cat.apply_vonkries(
                cat_xyz,
                cat_w1,
                cat_w2,
                xyzw0=np.array([[100.0, 100.0, 100.0]]),
                D=[0.8, 0.6],
                mcat="cat16",
                catmode="1>0>2",
            ).ravel(),
        ),
        vec_line(
            "cam16_forward",
            ciecam16(
                cat_xyz,
                xyzw=cat_w1,
                conditions=cam_conditions,
                outin="J,Q,C,M,s,h,aM,bM,aC,bC",
            ).ravel(),
        ),
        vec_line(
            "ciecam02_forward",
            ciecam02(
                cat_xyz,
                xyzw=cat_w1,
                conditions=cam_conditions,
                outin="J,Q,C,M,s,h,aM,bM,aC,bC",
            ).ravel(),
        ),
        vec_line("cam16_ucs", cam16ucs(cat_xyz, xyzw=cat_w1, conditions=cam_conditions).ravel()),
        vec_line(
            "cam16_lcd",
            cam16ucs(cat_xyz, xyzw=cat_w1, conditions=cam_conditions, ucstype="lcd").ravel(),
        ),
        vec_line(
            "cam16_scd",
            cam16ucs(cat_xyz, xyzw=cat_w1, conditions=cam_conditions, ucstype="scd").ravel(),
        ),
        vec_line("cam02_ucs", cam02ucs(cat_xyz, xyzw=cat_w1, conditions=cam_conditions).ravel()),
        vec_line(
            "cam02_lcd",
            cam02ucs(cat_xyz, xyzw=cat_w1, conditions=cam_conditions, ucstype="lcd").ravel(),
        ),
        vec_line(
            "cam02_scd",
            cam02ucs(cat_xyz, xyzw=cat_w1, conditions=cam_conditions, ucstype="scd").ravel(),
        ),
        vec_line("cam16_inverse", np.array([[19.01, 20.0, 21.78]]).ravel()),
        vec_line("cam02_inverse", np.array([[19.01, 20.0, 21.78]]).ravel()),
        vec_line("cam16ucs_inverse", np.array([[19.01, 20.0, 21.78]]).ravel()),
        vec_line("cam02ucs_inverse", np.array([[19.01, 20.0, 21.78]]).ravel()),
        vec_line("xyz_to_srgb", srgb_sample.ravel()),
        vec_line("srgb_to_xyz", lx.srgb_to_xyz(np.array([[64.0, 128.0, 192.0]])).ravel()),
    ]
