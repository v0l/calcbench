use std::process::Command;

fn run(args: &[&str]) -> (bool, String, String) {
    let out = Command::new(env!("CARGO_BIN_EXE_calcbench")).args(args).output().unwrap();
    (out.status.success(), String::from_utf8_lossy(&out.stdout).trim().to_string(), String::from_utf8_lossy(&out.stderr).trim().to_string())
}

#[test]
fn bare_expression_is_evaluated() {
    assert_eq!(run(&["60 mph to m/s"]), (true, "26.8224 m / s".into(), String::new()));
    assert_eq!(run(&["3", "ft", "2", "in", "to", "cm"]).1, "96.52 cm");
    assert_eq!(run(&["1 kWh / 3.6 MJ"]).1, "1");
}

#[test]
fn dimension_errors_exit_non_zero() {
    let (ok, out, err) = run(&["1 m + 1 s"]);
    assert!(!ok);
    assert!(out.is_empty());
    assert!(err.starts_with("error:") && err.contains("incompatible"), "{err}");
}

#[test]
fn formula_solves_for_the_missing_variable() {
    assert_eq!(run(&["formula", "ohm", "V=12", "R=470"]).1, "I = 25.53191489 mA  (current)");
    assert_eq!(run(&["formula", "ohm", "V=12 V", "I=25 mA"]).1, "R = 480 ohm  (resistance)");
    let json = run(&["formula", "ohm", "V=12", "R=470", "--json"]).1;
    assert!(json.starts_with("{\"formula\":\"ohm\",\"solved\":\"I\""), "{json}");
    assert!(json.contains("\"unit\":\"A\""));
}

#[test]
fn formula_card_and_list() {
    let card = run(&["formula", "rc-cutoff"]).1;
    assert!(card.contains("fc = 1/(2·π·R·C)"), "{card}");
    assert!(card.contains("source: standard"));
    assert!(run(&["formula"]).1.contains("ohm"));
    assert!(run(&["formula", "search", "sallen"]).1.contains("sallen-key-cutoff"));
    let (ok, _, err) = run(&["formula", "ohmm"]);
    assert!(!ok && err.contains("did you mean ohm"), "{err}");
}

#[test]
fn constants_are_printed_with_uncertainty_and_source() {
    let c = run(&["const", "c"]).1;
    assert!(c.contains("299792458 m s^-1") && c.contains("exact") && c.contains("CODATA 2022"), "{c}");
    let g = run(&["const", "G"]).1;
    assert!(g.contains("6.6743e-11") && g.contains("1.5e-15"), "{g}");
    assert!(run(&["const", "--json", "h"]).1.starts_with("[{\"name\":\"Planck constant\""));
    assert!(!run(&["const", "nonsense-thing"]).0);
    assert!(run(&["const", "list", "atmosphere"]).1.contains("standard atmosphere"));
}

#[test]
fn helpers() {
    assert_eq!(run(&["eseries", "E24", "4800"]).1, "E24 4700 (-2.08%)");
    assert_eq!(run(&["colour", "470", "5"]).1, "yellow violet brown gold");
    assert_eq!(run(&["colour", "4.7 kohm"]).1, "yellow violet red gold");
    assert_eq!(run(&["colour", "yellow", "violet", "brown", "gold"]).1, "470 ohm 5%");
    assert_eq!(run(&["cap", "104"]).1, "100 nF");
    assert_eq!(run(&["cap", "100nF"]).1, "104");
    assert!(run(&["base", "255", "8"]).1.contains("hex FF"));
    assert!(run(&["base", "-5", "8"]).1.contains("twos complement 11111011"));
    assert!(run(&["float", "1.0"]).1.contains("hex 3F800000"));
    assert!(run(&["bits", "xor", "0xF0", "0xFF", "8"]).1.starts_with("15 (hex F"));
    assert!(run(&["stats", "1,2,3,4"]).1.contains("mean 2.5"));
    assert!(run(&["divider", "E24", "0.5"]).1.lines().count() == 5);
}

#[test]
fn version_and_help() {
    assert!(run(&["--version"]).1.starts_with("calcbench 0.1.0"));
    assert!(run(&["--help"]).1.contains("usage:"));
}
