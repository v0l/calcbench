use calcbench::engine::Calc;
use calcbench::quantities::{QUANTITIES, is_log};

#[test]
fn every_unit_of_a_quantity_converts_to_its_first_unit() {
    let mut calc = Calc::new();
    let mut bad = Vec::new();
    for q in QUANTITIES {
        let base = q.units[0];
        for u in q.units {
            if is_log(u) {
                continue;
            }
            if let Err(e) = calc.eval(&format!("1 {u} to {base}")) {
                bad.push(format!("{}: {u} to {base}: {e}", q.name));
            }
        }
    }
    assert!(bad.is_empty(), "{}", bad.join("\n"));
}

#[test]
fn log_units_convert_inside_their_quantity() {
    let mut calc = Calc::new();
    for q in QUANTITIES.iter().filter(|q| q.units.iter().any(|u| is_log(u))) {
        for u in q.units {
            for v in q.units {
                if q.name == "gain" {
                    continue;
                }
                let r = calc.eval(&format!("1 {u} to {v}"));
                assert!(r.is_ok(), "{}: {u} to {v}: {:?}", q.name, r);
            }
        }
    }
}

#[test]
fn quantities_cover_the_requested_list() {
    let names: Vec<_> = QUANTITIES.iter().map(|q| q.name).collect();
    for want in [
        "length", "area", "volume", "mass", "time", "speed", "acceleration", "force", "pressure", "energy", "power", "temperature", "angle",
        "frequency", "data size", "data rate", "current", "voltage", "resistance", "capacitance", "inductance", "charge", "magnetic flux",
        "magnetic field", "luminous flux", "radiation dose", "torque", "flow", "density", "dynamic viscosity", "fuel economy", "molarity", "mass concentration",
        "power level", "voltage level",
    ] {
        assert!(names.contains(&want), "{want}");
    }
}

#[test]
fn spot_checks_inside_quantities() {
    let mut calc = Calc::new();
    assert_eq!(calc.eval("1 nmi to m").unwrap(), "1852 m");
    assert_eq!(calc.eval("1 gee to m/s^2").unwrap(), "9.80665 m / s^2");
    assert_eq!(calc.eval("1 KiB to B").unwrap(), "1024 B");
    assert_eq!(calc.eval("1 mAh to coulomb").unwrap(), "3.6 coulomb");
    assert_eq!(calc.eval("10 dBm to mW").unwrap(), "10 mW");
    assert_eq!(calc.eval("0 dBu to V").unwrap(), "0.7745966692 V");
}
