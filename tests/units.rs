use calcbench::engine::Calc;

fn num(c: &mut Calc, expr: &str, unit: &str) -> f64 {
    c.value_in(expr, unit).unwrap_or_else(|e| panic!("{expr}: {e}"))
}

fn close(a: f64, b: f64) {
    assert!((a - b).abs() <= 1e-9 * b.abs().max(1e-300), "{a} vs {b}");
}

#[test]
fn known_conversions() {
    let mut c = Calc::new();
    let table: &[(&str, &str, f64)] = &[
        ("60 mph", "m/s", 26.8224),
        ("3 ft 2 in", "cm", 96.52),
        ("1 kWh / 3.6 MJ", "", 1.0),
        ("1 inch", "mm", 25.4),
        ("1 mile", "km", 1.609344),
        ("1 lb", "kg", 0.45359237),
        ("1 psi", "Pa", 6894.757293168),
        ("1 bar", "kPa", 100.0),
        ("1 atm", "Pa", 101325.0),
        ("1 hp", "W", 745.69987158227),
        ("1 BTU", "J", 1055.05585),
        ("1 cal", "J", 4.184),
        ("1 eV", "J", 1.602176634e-19),
        ("1 knot", "km/h", 1.852),
        ("1 gal", "L", 3.785411784),
        ("1 fl oz", "mL", 29.5735295625),
        ("1 L/min", "m^3/s", 1.0 / 60000.0),
        ("1 g/cm^3", "kg/m^3", 1000.0),
        ("1 cP", "Pa s", 0.001),
        ("1 gauss", "tesla", 1e-4),
        ("1 rem", "mSv", 10.0),
        ("1 Gbit/s", "MB/s", 125.0),
        ("1 kgf m", "N m", 9.80665),
        ("2 kΩ", "ohm", 2000.0),
        ("10 uF", "nF", 10000.0),
        ("1 rad", "deg", 57.29577951308232),
        ("1 rpm", "rad/s", 0.10471975511965977),
        ("12 V / 470 ohm", "mA", 25.531914893617),
        ("5 mA * 2 kohm", "V", 10.0),
        ("1 mmHg", "Pa", 133.322368421),
        ("1 mil", "um", 25.4),
        ("1 ukgal", "L", 4.54609),
    ];
    for (expr, unit, want) in table {
        let got = num(&mut c, expr, unit);
        assert!((got - want).abs() <= 1e-8 * want.abs(), "{expr} to {unit}: {got} vs {want}");
    }
}

#[test]
fn temperature_offsets() {
    let mut c = Calc::new();
    close(num(&mut c, "20 C", "K"), 293.15);
    close(num(&mut c, "100 C", "F"), 212.0);
    close(num(&mut c, "-40 F", "C"), -40.0);
    close(num(&mut c, "0 K", "C"), -273.15);
    close(num(&mut c, "98.6 F", "C"), 37.0);
    assert_eq!(c.eval("20 C to F").unwrap(), "68 °F");
}

#[test]
fn inverse_units() {
    let mut c = Calc::new();
    assert_eq!(c.eval("30 mpg to L/100km").unwrap(), "7.840486111 L/100km");
    assert_eq!(c.eval("8 L/100km to mpg").unwrap(), "29.4018229167 mpg");
    close(num(&mut c, "100 km / 6 L", "km/L"), 100.0 / 6.0);
}

#[test]
fn decibels() {
    let mut c = Calc::new();
    assert_eq!(c.eval("0 dBm to mW").unwrap(), "1 mW");
    assert_eq!(c.eval("30 dBm to W").unwrap(), "1 W");
    assert_eq!(c.eval("1 W to dBm").unwrap(), "30 dBm");
    assert_eq!(c.eval("0 dBW to dBm").unwrap(), "30 dBm");
    assert_eq!(c.eval("1 V to dBuV").unwrap(), "120 dBuV");
    assert_eq!(c.eval("0 dBV to dBmV").unwrap(), "60 dBmV");
    assert_eq!(c.eval("20 dB to x").unwrap(), "100");
    assert_eq!(c.eval("6 dBi to x").unwrap(), "3.981071706");
    close(num(&mut c, "0 dBu", "V"), 0.7745966692414834);
    assert!(c.eval("0 dBm to V").is_err());
}

#[test]
fn dimension_errors() {
    let mut c = Calc::new();
    assert!(c.eval("1 m + 1 s").unwrap_err().contains("incompatible"));
    assert!(c.eval("5 kg to m").unwrap_err().contains("incompatible"));
    assert!(c.eval("1 V to A").is_err());
    assert!(c.eval("12 to V").is_err());
}

#[test]
fn complex_numbers_and_functions() {
    let mut c = Calc::new();
    assert_eq!(c.eval("(1+2i)*(3-i)").unwrap(), "5 + 5i");
    assert_eq!(c.eval("sqrt(16)").unwrap(), "4");
    assert_eq!(c.eval("sin(30 degrees)").unwrap(), "0.5");
    assert_eq!(c.eval("x = 4").unwrap(), "4");
    assert_eq!(c.eval("x^2").unwrap(), "16");
}

#[test]
fn every_listed_quantity_has_a_working_unit() {
    let mut c = Calc::new();
    for (expr, unit) in [
        ("1 m^2", "ft^2"), ("1 acre", "m^2"), ("1 L", "m^3"), ("1 year", "s"), ("1 g", "kg"), ("1 m/s^2", "ft/s^2"),
        ("1 lbf", "N"), ("1 J", "kWh"), ("1 rad", "deg"), ("1 kHz", "Hz"), ("1 MB", "bit"), ("1 A", "mA"),
        ("1 H", "uH"), ("1 C", "mC"), ("1 weber", "maxwell"), ("1 tesla", "gauss"), ("1 lumen", "cd sr"),
        ("1 lux", "lm/m^2"), ("1 Gy", "cGy"), ("1 Sv", "rem"), ("1 N m", "ft lbf"), ("1 gal/min", "L/s"),
        ("1 g/mL", "kg/L"), ("1 mol/L", "mol/m^3"), ("1 ppm", ""),
    ] {
        c.value_in(expr, unit).unwrap_or_else(|e| panic!("{expr} to {unit}: {e}"));
    }
}

#[test]
fn farad_and_coulomb_are_not_temperatures() {
    let mut c = Calc::new();
    close(num(&mut c, "100 nF", "uF"), 0.1);
    close(num(&mut c, "1 F", "mF"), 1000.0);
    close(num(&mut c, "2 C", "mC"), 2000.0);
    assert_eq!(c.eval("100 nF to F").unwrap(), "0.0000001 farad");
}
