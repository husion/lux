from collections.abc import Iterable


def fmt_scalar(value: float) -> str:
    return repr(float(value))


def fmt_vec(values: Iterable[float]) -> str:
    return ",".join(repr(float(value)) for value in values)


def scalar_line(key: str, value: float) -> tuple[str, str]:
    return key, fmt_scalar(value)


def vec_line(key: str, values: Iterable[float]) -> tuple[str, str]:
    return key, fmt_vec(values)


def usize_vec_line(key: str, values: Iterable[int]) -> tuple[str, str]:
    return key, ",".join(str(int(value)) for value in values)
