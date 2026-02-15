# Publishing `rsn_db` to PyPI

## 1) Prerequisites

```bash
python -m pip install --upgrade maturin twine
```

## 2) Build distributable artifacts

```bash
maturin build --release
```

Generated wheels and source distributions are placed in `target/wheels/`.

## 3) Validate package metadata

```bash
python -m twine check target/wheels/*
```

## 4) Upload to TestPyPI (recommended dry run)

```bash
python -m twine upload --repository testpypi target/wheels/*
```

## 5) Upload to PyPI

```bash
python -m twine upload target/wheels/*
```

## 6) Verify install path

```bash
python -m pip install rsn_db
python -c "from rsn_db import Database; print(Database())"
```

## Notes

- Package install name is `rsn_db`, while the Rust extension module remains `rsn_db._core`.
- Build from a clean git state and increment versions in both `Cargo.toml` and `pyproject.toml` before each release.
