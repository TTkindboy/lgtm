# LGTM
![PyPI - Status](https://img.shields.io/pypi/status/lgtm?pypiBaseUrl=https%3A%2F%2Ftest.pypi.org)
![Python Version from PEP 621 TOML](https://img.shields.io/python/required-version-toml?tomlFilePath=https%3A%2F%2Fraw.githubusercontent.com%2FTTkindboy%2Flgtm%2Frefs%2Fheads%2Fmain%2Fpyproject.toml)
![Static Badge](https://img.shields.io/badge/OS-Linux%20%7C%20macOS%2014%2B%20%7C%20Windows-DA644F)

A CYOA game about code review purgatory

## Usage

Run the game directly from the GitHub source during development. (ARM macOS only for now)

### With uv: (recommended) 
```bash
uvx --index https://test.pypi.org/simple/ --index-strategy unsafe-best-match --prerelease allow lgtm
```
### With pipx:
```bash
pipx run --index-url https://test.pypi.org/simple/ --pip-args="--pre" lgtm
```
