from pathlib import Path

from baseline_color import generate_color_baselines
from baseline_observers import generate_observer_baselines
from baseline_sources import generate_source_baselines
from baseline_spectra import generate_spectral_baselines


BASELINE_GROUPS = (
    generate_observer_baselines,
    generate_color_baselines,
    generate_spectral_baselines,
)


def main() -> None:
    root = Path(__file__).resolve().parents[2]
    for generator in BASELINE_GROUPS:
        for key, value in generator():
            print(f"{key}={value}")
    for key, value in generate_source_baselines(root):
        print(f"{key}={value}")


if __name__ == "__main__":
    main()
