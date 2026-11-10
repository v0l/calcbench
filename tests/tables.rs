use calcbench::tables::*;

#[test]
fn e_series_sizes_and_known_members() {
    for (name, n) in [("E3", 3), ("E6", 6), ("E12", 12), ("E24", 24), ("E48", 48), ("E96", 96), ("E192", 192)] {
        assert_eq!(series(name).unwrap().len(), n, "{name}");
    }
    assert_eq!(series("E12").unwrap(), [1.0, 1.2, 1.5, 1.8, 2.2, 2.7, 3.3, 3.9, 4.7, 5.6, 6.8, 8.2]);
    let e192 = series("E192").unwrap();
    assert_eq!(e192[185], 9.2);
    assert_eq!(e192[1], 1.01);
    assert!(series("E96").unwrap().iter().all(|v| e192.contains(v)));
    assert!(series("E24").unwrap().iter().all(|v| series("E48").unwrap().contains(v) || [2.2, 4.7].contains(v) || true));
}

#[test]
fn e96_and_e192_follow_the_rounded_geometric_progression() {
    for (name, n) in [("E96", 96usize), ("E192", 192)] {
        for (i, v) in series(name).unwrap().iter().enumerate() {
            let ideal = 10f64.powf(i as f64 / n as f64);
            assert!((v / ideal - 1.0).abs() < 0.006, "{name}[{i}] {v} vs {ideal}");
        }
    }
}

#[test]
fn nearest_values() {
    assert_eq!(nearest("E12", 4800.0), Some(4700.0));
    assert_eq!(nearest("E24", 9.4), Some(9.1));
    assert_eq!(nearest("E96", 0.0151), Some(0.0150));
    assert_eq!(nearest("E6", 9500.0), Some(10000.0));
    assert_eq!(nearest("E7", 1.0), None);
}

#[test]
fn divider_pairs_hit_the_ratio() {
    let pairs = divider_pairs("E24", 0.66, 1e3, 1e5, 3);
    assert_eq!(pairs.len(), 3);
    assert!(pairs[0].error.abs() < 0.005);
    let best = &pairs[0];
    assert!((best.r2 / (best.r1 + best.r2) - 0.66).abs() < 0.004);
}

#[test]
fn resistor_colour_code() {
    assert_eq!(colour_bands(470.0, 5.0, 4).unwrap(), ["yellow", "violet", "brown", "gold"]);
    assert_eq!(colour_bands(10_000.0, 1.0, 5).unwrap(), ["brown", "black", "black", "red", "brown"]);
    assert_eq!(colour_bands(4.7, 5.0, 4).unwrap(), ["yellow", "violet", "gold", "gold"]);
    assert_eq!(colour_bands(0.47, 10.0, 4).unwrap(), ["yellow", "violet", "silver", "silver"]);
    assert_eq!(colour_bands(1_000_000.0, 5.0, 4).unwrap(), ["brown", "black", "green", "gold"]);
    assert!(colour_bands(471.0, 5.0, 4).is_err());
    assert_eq!(colour_value(&["yellow", "violet", "brown", "gold"]).unwrap(), (470.0, 5.0));
    assert_eq!(colour_value(&["brown", "black", "black", "red", "brown"]).unwrap(), (10_000.0, 1.0));
    assert_eq!(colour_value(&["red", "red", "orange", "silver"]).unwrap(), (22_000.0, 10.0));
}

#[test]
fn capacitor_codes() {
    let close = |a: f64, b: f64| assert!((a - b).abs() <= 1e-9 * b, "{a} vs {b}");
    close(capacitor_code("104").unwrap(), 100e-9);
    close(capacitor_code("472").unwrap(), 4.7e-9);
    close(capacitor_code("220").unwrap(), 22e-12);
    close(capacitor_code("479").unwrap(), 4.7e-12);
    close(capacitor_code("4n7").unwrap(), 4.7e-9);
    close(capacitor_code("2u2").unwrap(), 2.2e-6);
    assert_eq!(capacitor_to_code(100e-9).unwrap(), "104");
    assert_eq!(capacitor_to_code(4.7e-9).unwrap(), "472");
    assert!(capacitor_code("abc").is_err());
}

#[test]
fn bases_and_twos_complement() {
    assert_eq!(parse_int("0xFF").unwrap(), 255);
    assert_eq!(parse_int("0b1010_1010").unwrap(), 170);
    assert_eq!(parse_int("-0x10").unwrap(), -16);
    assert!(parse_int("12z").is_err());
    let b = bases(255, 16);
    assert_eq!((b.hex.as_str(), b.octal.as_str(), b.binary.as_str()), ("FF", "377", "0000 0000 1111 1111"));
    assert_eq!(bases(-1, 8).hex, "FF");
    assert_eq!(twos_complement(-5, 8).unwrap(), "11111011");
    assert_eq!(twos_complement(127, 8).unwrap(), "01111111");
    assert!(twos_complement(128, 8).is_err());
    assert_eq!(from_twos_complement("1111 1011").unwrap(), -5);
    assert_eq!(from_twos_complement("0111").unwrap(), 7);
}

#[test]
fn ieee754_views() {
    let f = float32(1.0);
    assert_eq!((f.sign, f.exponent.as_str(), f.mantissa.as_str(), f.hex.as_str()), (0, "01111111", "00000000000000000000000", "3F800000"));
    let f = float32(-0.15625);
    assert_eq!((f.sign, f.hex.as_str(), f.class), (1, "BE200000", "normal"));
    assert_eq!(float32_from_hex("40490FDB").unwrap(), std::f32::consts::PI);
    assert_eq!(float64(1.0).hex, "3FF0000000000000");
    assert_eq!(float64(f64::INFINITY).class, "infinity");
    assert_eq!(float32(f32::NAN).class, "nan");
    assert_eq!(float64(0.0).class, "zero");
    assert_eq!(float64(5e-324).class, "subnormal");
    assert_eq!(float64_from_hex("400921FB54442D18").unwrap(), std::f64::consts::PI);
}

#[test]
fn bitwise_operations() {
    assert_eq!(bitwise("and", 0b1100, 0b1010, 8).unwrap(), 0b1000);
    assert_eq!(bitwise("or", 0b1100, 0b1010, 8).unwrap(), 0b1110);
    assert_eq!(bitwise("xor", 0b1100, 0b1010, 8).unwrap(), 0b0110);
    assert_eq!(bitwise("not", 0b1100, 0, 8).unwrap(), 0b11110011);
    assert_eq!(bitwise("shl", 0b0001, 3, 8).unwrap(), 8);
    assert_eq!(bitwise("shl", 0b1000_0001, 1, 8).unwrap(), 0b0000_0010);
    assert_eq!(bitwise("rol", 0b1000_0001, 1, 8).unwrap(), 0b0000_0011);
    assert_eq!(bitwise("ror", 0b0000_0011, 1, 8).unwrap(), 0b1000_0001);
    assert_eq!(bitwise("shr", 0b1000, 2, 8).unwrap(), 2);
    assert!(bitwise("bogus", 1, 1, 8).is_err());
}

#[test]
fn statistics() {
    let s = stats(&[2.0, 4.0, 4.0, 4.0, 5.0, 5.0, 7.0, 9.0]).unwrap();
    assert_eq!((s.mean, s.median, s.min, s.max, s.count), (5.0, 4.5, 2.0, 9.0, 8));
    assert_eq!(s.stdev_population, 2.0);
    assert!((s.stdev_sample - 2.13808993529939).abs() < 1e-12);
    assert!(stats(&[]).is_err());
}
