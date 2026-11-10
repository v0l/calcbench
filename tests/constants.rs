use calcbench::constants;

struct Row {
    name: String,
    value: f64,
    uncertainty: Option<f64>,
    unit: String,
}

fn parse_num(s: &str) -> f64 {
    s.replace(' ', "").replace("...", "").parse().unwrap_or_else(|_| panic!("number {s:?}"))
}

fn nist_rows() -> Vec<Row> {
    let text = include_str!("fixtures/nist-codata-2022-allascii.txt");
    text.lines()
        .skip(11)
        .filter(|l| !l.trim().is_empty())
        .map(|l| {
            let field = |a: usize, b: usize| l.get(a..b.min(l.len())).unwrap_or("").trim().to_string();
            let unc = field(85, 110);
            Row {
                name: field(0, 60),
                value: parse_num(&field(60, 85)),
                uncertainty: if unc == "(exact)" { None } else { Some(parse_num(&unc)) },
                unit: l.get(110..).unwrap_or("").trim().to_string(),
            }
        })
        .collect()
}

#[test]
fn codata_matches_the_nist_table() {
    let rows = nist_rows();
    assert!(rows.len() > 340);
    let ours: Vec<_> = constants::all().iter().filter(|c| c.category == "codata").collect();
    assert_eq!(ours.len(), rows.len());
    for (row, c) in rows.iter().zip(ours) {
        assert_eq!(row.name, c.name);
        assert_eq!(row.value, c.value, "{}", row.name);
        assert_eq!(row.unit, c.unit, "{}", row.name);
        match row.uncertainty {
            None => assert!(c.exact, "{}", row.name),
            Some(u) => {
                assert!(!c.exact, "{}", row.name);
                assert_eq!(u, c.uncertainty, "{}", row.name);
            }
        }
    }
}

#[test]
fn well_known_values() {
    let get = |s: &str| constants::find(s)[0];
    assert_eq!(get("c").value, 299792458.0);
    assert!(get("c").exact);
    assert_eq!(get("h").value, 6.62607015e-34);
    assert_eq!(get("k").value, 1.380649e-23);
    assert_eq!(get("NA").value, 6.02214076e23);
    assert_eq!(get("qe").value, 1.602176634e-19);
    assert_eq!(get("G").value, 6.67430e-11);
    assert_eq!(get("G").uncertainty, 0.00015e-11);
    assert_eq!(get("e").value, std::f64::consts::E);
    assert_eq!(get("pi").value, std::f64::consts::PI);
}

#[test]
fn search_by_words() {
    let hits = constants::find("electron mass");
    assert!(hits.iter().any(|c| c.name == "electron mass"));
    assert!(constants::find("copper resistivity")[0].value > 1e-8);
    assert!(constants::find("zzzz nothing").is_empty());
}

#[test]
fn curated_units_are_understood_by_the_engine() {
    let mut calc = calcbench::engine::Calc::new();
    for c in constants::all().iter().filter(|c| c.category != "codata" && !c.unit.is_empty()) {
        let r = calc.eval(&format!("1 ({})", c.unit.replace(' ', " * ")));
        assert!(r.is_ok(), "{}: {} {:?}", c.name, c.unit, r);
    }
}

#[test]
fn categories_are_present() {
    let cats = constants::categories();
    for want in ["codata", "maths", "earth", "astronomy", "atmosphere", "materials"] {
        assert!(cats.contains(&want), "{want}");
    }
}
