# calcbench

Offline units, constants, formulas and a calculator for builders, in the Bench Suite look. Linux, macOS, Windows and Android, plus a CLI that agents can drive.

```sh
calcbench "60 mph to m/s"
calcbench formula ohm V=12 R=470
calcbench const c
calcbench eseries E24 4800
```

Run `calcbench` with no arguments for the window, `calcbench --help` for every command.

## What is in it

- **Convert**: expressions with units (`3 ft 2 in to cm`, `1 kWh / 3.6 MJ`), temperatures with offsets, fuel economy, dBm/dBW/dBV/dBuV/dBu/dBi, and a picker for 40 quantities. Mixed dimensions are errors.
- **Constants**: CODATA 2022 (355 values with uncertainty, checked against the NIST table), plus maths, earth, astronomy, standard atmosphere and material tables, each with source and copy.
- **Formulas**: 138 cards across electronics, physics and maths. Fill all variables but one and it solves for the empty one, with units checked.
- **Calculator**: history, variables, functions, complex numbers. **Plot** of y = f(x) with units on the axes.
- **Tools**: bases, two's complement, IEEE-754, bitwise, E3 to E192, divider pairs, resistor colour code, capacitor codes.

## Units engine

Units and expression parsing use [fend-core](https://github.com/printfn/fend) (MIT). It embeds as a small dependency with a large unit database, complex numbers and no runtime files. calcbench adds a normaliser on top for compound lengths (`5 ft 3 in`), `Ω` and `kohm`, `°C` versus coulomb, dB scales and `L/100km`.

## Adding a formula

Formulas are plain TOML in `data/electronics.toml`, `data/physics.toml` and `data/maths.toml`. Add a `[[formula]]` entry with `id`, `name`, `tags`, `equation`, `source` and `vars` (`s`, `n`, `u`, `x` example value, optional `signed`, `min`, `max`). `cargo test --test formulas` checks every variable of every entry solves round trip to 1e-9.

## Tests

```sh
cargo test
```

`tests/fixtures/nist-codata-2022-allascii.txt` is the NIST table (fetched 2026-11-10) the CODATA data is checked against.

## Releasing

Tag `vX.Y.Z` matching `Cargo.toml` and a `CHANGELOG.md` entry. CI builds deb, tar.gz, dmg, msi, zip, a signed Android APK and the source archive. A manual `workflow_dispatch` with a `tag` input is a dry run that publishes nothing. The Android job needs the `SIGNING_KEYSTORE_BASE64`, `SIGNING_STORE_PASSWORD`, `SIGNING_KEY_PASSWORD` and `SIGNING_KEY_ALIAS` secrets. Android: `packaging/android/build.sh` (`ABI=x86_64` for the emulator).

## Later

`gcad calc` (`gcad/crates/gcad-calc`: O-rings, threads, fits, materials) could share its tables with calcbench. Not merged.

## Licence

GPL-3.0-only.
