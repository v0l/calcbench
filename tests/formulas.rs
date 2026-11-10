use calcbench::formula::{self, Formula};
use std::collections::HashMap;

fn consistent_set(f: &Formula) -> HashMap<String, f64> {
    for driver in &f.vars {
        let mut known: HashMap<String, f64> =
            f.vars.iter().filter(|v| v.s != driver.s).map(|v| (v.s.clone(), v.x)).collect();
        if let Ok(v) = formula::solve(f, &driver.s, &known) {
            known.insert(driver.s.clone(), v);
            return known;
        }
    }
    panic!("{}: no variable can be solved from the example values", f.id);
}

#[test]
fn every_entry_solves_round_trip_for_each_variable() {
    assert!(formula::all().len() >= 100);
    for f in formula::all() {
        let full = consistent_set(f);
        for v in &f.vars {
            let mut known = full.clone();
            let want = known.remove(&v.s).unwrap();
            let got = formula::solve(f, &v.s, &known).unwrap_or_else(|e| panic!("{} / {}: {e}", f.id, v.s));
            let tol = 1e-9 * want.abs().max(1e-300);
            assert!((got - want).abs() <= tol || (got - want).abs() < 1e-12, "{} / {}: got {got}, want {want}", f.id, v.s);
        }
    }
}

#[test]
fn ids_are_unique_and_variables_are_named_once() {
    let mut seen = std::collections::HashSet::new();
    for f in formula::all() {
        assert!(seen.insert(&f.id), "duplicate id {}", f.id);
        let mut syms = std::collections::HashSet::new();
        for v in &f.vars {
            assert!(syms.insert(&v.s), "{}: duplicate variable {}", f.id, v.s);
        }
        assert!(f.equation.contains('='), "{}", f.id);
        assert!(!f.source.is_empty(), "{} has no source", f.id);
    }
}

#[test]
fn units_of_every_variable_are_understood() {
    let mut calc = calcbench::engine::Calc::new();
    for f in formula::all() {
        for v in f.vars.iter().filter(|v| !v.u.is_empty()) {
            calc.value_in(&format!("1 {}", v.u), &v.u).unwrap_or_else(|e| panic!("{} / {}: {} {e}", f.id, v.s, v.u));
        }
    }
}

#[test]
fn domains_cover_the_listed_topics() {
    let text: String = formula::all().iter().map(|f| format!("{} {} ", f.id, f.tags)).collect();
    for topic in [
        "ohm", "series", "parallel", "divider", "rc", "rl", "resonance", "reactance", "impedance", "filter", "sallen", "led", "opamp", "555",
        "battery", "wire", "ipc", "microstrip", "stripline", "coax", "vswr", "return loss", "db", "adc", "kinematics", "newton", "energy",
        "momentum", "projectile", "circular", "gravity", "spring", "pendulum", "bernoulli", "reynolds", "gas", "conduction", "convection",
        "radiation", "doppler", "lens", "snell", "beam", "stress", "torque", "gears", "path loss", "friis", "antenna", "skin depth",
        "geometry", "trigonometry", "quadratic", "cubic", "statistics", "normal", "finance", "loan",
    ] {
        assert!(text.contains(topic), "no formula covers {topic}");
    }
}

#[test]
fn solves_with_units() {
    let mut calc = calcbench::engine::Calc::new();
    let ohm = formula::get("ohm").unwrap();
    let s = formula::solve_with_units(&mut calc, ohm, &[("V".into(), "12 V".into()), ("R".into(), "470 ohm".into())]).unwrap();
    assert_eq!(s.symbol, "I");
    assert!((s.value - 12.0 / 470.0).abs() < 1e-12);
    assert_eq!(s.engineering(), "25.53191489 mA");
    let s = formula::solve_with_units(&mut calc, ohm, &[("I".into(), "25 mA".into()), ("R".into(), "2 kΩ".into())]).unwrap();
    assert!((s.value - 50.0).abs() < 1e-9);
    assert!(formula::solve_with_units(&mut calc, ohm, &[("V".into(), "12 m".into()), ("R".into(), "1 ohm".into())]).is_err());
    assert!(formula::solve_with_units(&mut calc, ohm, &[("V".into(), "12 V".into())]).is_err());
}

#[test]
fn known_answers() {
    let get = |id: &str, target: &str, given: &[(&str, f64)]| {
        let known = given.iter().map(|(k, v)| (k.to_string(), *v)).collect();
        formula::solve(formula::get(id).unwrap(), target, &known).unwrap()
    };
    let close = |a: f64, b: f64, tol: f64| assert!((a - b).abs() <= tol * b.abs(), "{a} vs {b}");
    close(get("ohm", "I", &[("V", 12.0), ("R", 470.0)]), 0.025531914893617, 1e-12);
    close(get("lc-resonance", "f", &[("L", 1e-3), ("C", 1e-6)]), 5032.921210448704, 1e-12);
    close(get("rc-cutoff", "fc", &[("R", 1000.0), ("C", 1e-7)]), 1591.549430918953, 1e-12);
    close(get("free-space-path-loss", "FSPL", &[("d", 1000.0), ("f", 2.4e9)]), 100.052008, 1e-7);
    close(get("vswr", "vswr", &[("g", 0.2)]), 1.5, 1e-12);
    close(get("orbital-period", "T", &[("a", 6.778e6), ("M", 5.9722e24)]), 5553.0, 2e-3);
    close(get("normal-cdf", "p", &[("x", 1.96), ("mu", 0.0), ("sigma", 1.0)]), 0.9750021048517795, 1e-9);
    close(get("student-t-cdf", "p", &[("t", 2.228138851986274), ("nu", 10.0)]), 0.975, 1e-6);
    close(get("loan-payment", "P", &[("L", 100000.0), ("i", 0.005), ("n", 240.0)]), 716.431, 1e-5);
    close(get("awg-diameter", "d", &[("n", 18.0)]), 0.001023, 1e-3);
    close(get("quadratic", "x", &[("a", 1.0), ("b", -5.0), ("c", 6.0)]), 2.0, 1e-12);
    close(get("ipc2221-current", "I", &[("k", 0.048), ("dT", 10.0), ("A", 137.8)]), 4.701072188, 1e-8);
}

#[test]
fn search_finds_cards() {
    assert!(formula::search("ohm").iter().any(|f| f.id == "ohm"));
    assert!(formula::search("low pass filter").is_empty() || !formula::search("filter").is_empty());
    assert!(formula::search("rc filter").iter().any(|f| f.id == "rc-cutoff"));
    assert!(formula::search("zzzzzz").is_empty());
}

#[test]
fn pretty_renders_symbols() {
    assert_eq!(formula::pretty("tau = R*C"), "τ = R·C");
    assert_eq!(formula::pretty("v^2 = u^2 + 2*a*s"), "v² = u² + 2·a·s");
    assert_eq!(formula::pretty("f = 1/(2*pi*sqrt(L*C))"), "f = 1/(2·π·√(L·C))");
}
