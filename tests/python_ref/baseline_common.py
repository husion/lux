from collections.abc import Iterable


def _iter_scalars(values):
    for value in values:
        if isinstance(value, Iterable) and not isinstance(value, (str, bytes)):
            yield from _iter_scalars(value)
        else:
            yield value


def fmt_scalar(value: float) -> str:
    return repr(float(value))


def fmt_vec(values: Iterable[float]) -> str:
    return ",".join(repr(float(value)) for value in _iter_scalars(values))


def scalar_line(key: str, value: float) -> tuple[str, str]:
    return key, fmt_scalar(value)


def vec_line(key: str, values: Iterable[float]) -> tuple[str, str]:
    return key, fmt_vec(values)


def usize_vec_line(key: str, values: Iterable[int]) -> tuple[str, str]:
    return key, ",".join(str(int(value)) for value in values)
