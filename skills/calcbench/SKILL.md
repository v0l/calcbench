---
name: calcbench
description: Convert units, look up physical constants and solve formulas for electronics, physics and maths with the calcbench CLI. Use when a task needs a unit conversion (60 mph to m/s, dBm to mW, mpg to L/100km), a CODATA constant, a resistor or capacitor value (E-series, colour code, capacitor code), a voltage divider, a filter cut-off, a path loss, or any formula solved for one unknown.
---

# calcbench

Offline calculator for builders. Every command prints plain text and exits non-zero with `error: ...` on stderr when it fails. Dimension mistakes are errors, never silent.

## Units and expressions

```sh
calcbench "60 mph to m/s"
calcbench "3 ft 2 in to cm"
calcbench "1 kWh / 3.6 MJ"
calcbench "20 C to F"
calcbench "30 mpg to L/100km"
calcbench "-30 dBm to mW"
calcbench "1 W to dBm"
calcbench "(1+2i)*(3-i)"
```

Supports SI prefixes, imperial and US units, offset temperatures, inverse fuel economy, dBm, dBW, dBV, dBmV, dBuV, dBu, dBi, dB, functions, variables and complex numbers. `1 m + 1 s` is an error.

## Constants

```sh
calcbench const c
calcbench const electron mass
calcbench const list atmosphere
calcbench const --json h
```

Each result has the value, unit, uncertainty (or `exact`) and source. CODATA 2022 plus maths, earth, astronomy, atmosphere and material tables. Categories: `calcbench const list`.

## Formulas

```sh
calcbench formula                       # every formula by domain
calcbench formula search divider
calcbench formula ohm                   # the card: equation, variables, units
calcbench formula ohm V=12 R=470        # leave one variable out, it is solved
calcbench formula ohm V="12 V" I="25 mA" --json
```

Inputs may carry units and are converted to the formula's unit. Give every variable but one. The answer is printed with an SI prefix.

## Helpers

```sh
calcbench eseries E24 4800              # nearest standard value
calcbench divider E24 0.66              # resistor pairs for Vout/Vin
calcbench colour 470 5                  # yellow violet brown gold
calcbench colour yellow violet brown gold
calcbench cap 104                       # 100 nF
calcbench cap 100nF                     # 104
calcbench base 255 8                    # dec, hex, oct, bin, two's complement
calcbench float 3.14159 32              # IEEE-754 fields
calcbench bits xor 0xF0 0xFF 8
calcbench stats 1,2,3,4
```

## Adding a formula

Formulas live in `data/electronics.toml`, `data/physics.toml` and `data/maths.toml`. Add a `[[formula]]` entry with `id`, `name`, `tags`, `equation`, `source` and `vars` (`s` symbol, `n` name, `u` unit, `x` an example value, optional `signed`, `min`, `max`). `cargo test --test formulas` checks that every variable can be solved from the others to 1e-9.
