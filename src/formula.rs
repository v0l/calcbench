use crate::expr::{self, Expr};
use serde::Deserialize;
use std::collections::HashMap;
use std::sync::OnceLock;

const FILES: &[(&str, &str)] = &[
    ("electronics", include_str!("../data/electronics.toml")),
    ("physics", include_str!("../data/physics.toml")),
    ("maths", include_str!("../data/maths.toml")),
];

#[derive(Debug, Clone, Deserialize)]
pub struct Var {
    pub s: String,
    pub n: String,
    #[serde(default)]
    pub u: String,
    pub x: f64,
    #[serde(default)]
    pub signed: bool,
    #[serde(default)]
    pub min: Option<f64>,
    #[serde(default)]
    pub max: Option<f64>,
    #[serde(default)]
    pub series: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Formula {
    pub id: String,
    pub name: String,
    #[serde(default)]
    pub tags: String,
    pub equation: String,
    #[serde(default)]
    pub notes: String,
    #[serde(default)]
    pub source: String,
    pub vars: Vec<Var>,
    #[serde(skip)]
    pub category: String,
}

#[derive(Deserialize)]
struct File {
    formula: Vec<Formula>,
}

pub fn all() -> &'static [Formula] {
    static ALL: OnceLock<Vec<Formula>> = OnceLock::new();
    ALL.get_or_init(|| {
        let mut out = Vec::new();
        for (cat, text) in FILES {
            let file: File = toml::from_str(text).unwrap_or_else(|e| panic!("{cat}.toml: {e}"));
            for mut f in file.formula {
                f.category = (*cat).to_string();
                out.push(f);
            }
        }
        out
    })
}

pub fn categories() -> Vec<&'static str> {
    FILES.iter().map(|(c, _)| *c).collect()
}

pub fn get(id: &str) -> Option<&'static Formula> {
    all().iter().find(|f| f.id == id)
}

pub fn search(query: &str) -> Vec<&'static Formula> {
    let words: Vec<String> = query.to_lowercase().split_whitespace().map(str::to_string).collect();
    all()
        .iter()
        .filter(|f| {
            let hay = format!("{} {} {} {} {}", f.id, f.name, f.tags, f.category, f.equation).to_lowercase();
            words.iter().all(|w| hay.contains(w.as_str()))
        })
        .collect()
}

#[derive(Debug, Clone)]
pub struct Solution {
    pub symbol: String,
    pub name: String,
    pub value: f64,
    pub unit: String,
}

impl Solution {
    pub fn engineering(&self) -> String {
        engineering(self.value, &self.unit)
    }
}

const PREFIXABLE: &[&str] = &[
    "V", "A", "ohm", "F", "H", "Hz", "W", "s", "m", "J", "N", "Pa", "C", "T", "S", "g", "bit", "Wb", "Wh", "B", "m^3", "W/m", "J/kg", "ohm m",
];

pub fn engineering(value: f64, unit: &str) -> String {
    use crate::engine::fmt_sig;
    if value == 0.0 || !value.is_finite() || !PREFIXABLE.contains(&unit) {
        return if unit.is_empty() { fmt_sig(value, 10) } else { format!("{} {unit}", fmt_sig(value, 10)) };
    }
    let table = [(-15, "f"), (-12, "p"), (-9, "n"), (-6, "u"), (-3, "m"), (0, ""), (3, "k"), (6, "M"), (9, "G"), (12, "T")];
    let exp = (value.abs().log10() / 3.0).floor() as i32 * 3;
    let (e, p) = table.iter().find(|(e, _)| *e == exp.clamp(-15, 12)).copied().unwrap_or((0, ""));
    let scaled = value / 10f64.powi(e);
    format!("{} {p}{unit}", fmt_sig(scaled, 10))
}

struct Compiled {
    residual: Expr,
    names: Vec<String>,
}

fn compile(f: &Formula) -> Result<Compiled, String> {
    let names: Vec<String> = f.vars.iter().map(|v| v.s.clone()).collect();
    let (lhs, rhs) = f.equation.split_once('=').ok_or("equation needs an =")?;
    let l = expr::parse(lhs, &names)?;
    let r = expr::parse(rhs, &names)?;
    Ok(Compiled { residual: Expr::Bin('-', Box::new(l), Box::new(r)), names })
}

fn grid(v: &Var) -> Vec<f64> {
    let mut pts = Vec::new();
    let lo = v.min.unwrap_or(if v.signed { f64::NEG_INFINITY } else { 0.0 });
    let hi = v.max.unwrap_or(f64::INFINITY);
    let mut e = -30.0;
    while e <= 30.0 {
        let m = 10f64.powf(e);
        pts.push(m);
        if v.signed || lo < 0.0 {
            pts.push(-m);
        }
        e += 0.125;
    }
    if let (Some(a), Some(b)) = (v.min, v.max) {
        for i in 0..=400 {
            pts.push(a + (b - a) * i as f64 / 400.0);
        }
    }
    pts.push(0.0);
    pts.retain(|p| *p >= lo && *p <= hi);
    pts.sort_by(|a, b| a.partial_cmp(b).unwrap());
    pts.dedup();
    pts
}

fn bisect(f: &dyn Fn(f64) -> f64, mut a: f64, mut b: f64) -> f64 {
    let mut fa = f(a);
    for _ in 0..300 {
        let m = if a > 0.0 && b > 0.0 && b / a > 4.0 { (a * b).sqrt() } else { 0.5 * (a + b) };
        if m == a || m == b {
            break;
        }
        let fm = f(m);
        if fm == 0.0 {
            return m;
        }
        if (fm < 0.0) == (fa < 0.0) {
            a = m;
            fa = fm;
        } else {
            b = m;
        }
    }
    0.5 * (a + b)
}

pub fn solve(f: &Formula, target: &str, known: &HashMap<String, f64>) -> Result<f64, String> {
    let c = compile(f)?;
    let ti = c.names.iter().position(|n| n == target).ok_or_else(|| format!("{} has no variable {target}", f.id))?;
    let mut values = vec![0.0; c.names.len()];
    for (i, n) in c.names.iter().enumerate() {
        if i != ti {
            values[i] = *known.get(n).ok_or_else(|| format!("missing value for {n}"))?;
        }
    }
    let var = &f.vars[ti];
    let eval = |x: f64| {
        let mut v = values.clone();
        v[ti] = x;
        c.residual.eval(&v)
    };
    let scale = |x: f64| {
        let mut v = values.clone();
        v[ti] = x;
        let mut s = 0.0f64;
        if let Expr::Bin(_, l, r) = &c.residual {
            s = l.eval(&v).abs().max(r.eval(&v).abs());
        }
        s
    };
    let pts = grid(var);
    let mut roots: Vec<f64> = Vec::new();
    let mut prev: Option<(f64, f64)> = None;
    for &p in &pts {
        let fp = eval(p);
        if fp == 0.0 {
            roots.push(p);
        }
        if fp.is_finite() {
            if let Some((q, fq)) = prev {
                if fq != 0.0 && fp != 0.0 && (fq < 0.0) != (fp < 0.0) {
                    let r = bisect(&eval, q, p);
                    let fr = eval(r);
                    if fr.abs() <= 1e-6 * scale(r).max(1e-300) {
                        roots.push(r);
                    }
                }
            }
            prev = Some((p, fp));
        } else {
            prev = None;
        }
    }
    if roots.is_empty() {
        return Err(format!("no solution for {target} with those values"));
    }
    let log_dist = |r: f64| {
        if r > 0.0 && var.x > 0.0 { (r / var.x).ln().abs() } else { (r - var.x).abs() + 1e3 }
    };
    roots.sort_by(|a, b| log_dist(*a).partial_cmp(&log_dist(*b)).unwrap());
    Ok(roots[0])
}

pub fn solve_with_units(
    calc: &mut crate::engine::Calc,
    f: &Formula,
    inputs: &[(String, String)],
) -> Result<Solution, String> {
    let mut known = HashMap::new();
    for (sym, text) in inputs {
        let var = f.vars.iter().find(|v| v.s == *sym).ok_or_else(|| format!("{} has no variable {sym}", f.id))?;
        let value = if let Ok(n) = text.trim().parse::<f64>() {
            n
        } else {
            let target = if var.u.is_empty() { "" } else { var.u.as_str() };
            calc.value_in(text, target).map_err(|e| format!("{sym}: {e}"))?
        };
        known.insert(sym.clone(), value);
    }
    let missing: Vec<&Var> = f.vars.iter().filter(|v| !known.contains_key(&v.s)).collect();
    match missing.as_slice() {
        [one] => {
            let value = solve(f, &one.s, &known)?;
            Ok(Solution { symbol: one.s.clone(), name: one.n.clone(), value, unit: one.u.clone() })
        }
        [] => Err("every variable is given: leave one out to solve for it".into()),
        many => Err(format!(
            "give {} more value(s): missing {}",
            many.len() - 1,
            many.iter().map(|v| v.s.as_str()).collect::<Vec<_>>().join(", ")
        )),
    }
}

pub fn pretty(src: &str) -> String {
    const GREEK: &[(&str, &str)] = &[
        ("omega", "ω"), ("Omega", "Ω"), ("theta", "θ"), ("lambda", "λ"), ("tau", "τ"), ("rho", "ρ"), ("mu0", "μ0"), ("mu", "μ"),
        ("eps0", "ε0"), ("epsr", "εr"), ("epsilon", "ε"), ("sigma", "σ"), ("delta", "δ"), ("phi", "φ"), ("alpha", "α"), ("beta", "β"),
        ("gamma", "γ"), ("eta", "η"), ("nu", "ν"), ("pi", "π"),
    ];
    let mut out = String::new();
    let chars: Vec<char> = src.chars().collect();
    let mut i = 0;
    while i < chars.len() {
        let c = chars[i];
        if c.is_alphabetic() || c == '_' {
            let start = i;
            while i < chars.len() && (chars[i].is_alphanumeric() || chars[i] == '_') {
                i += 1;
            }
            let word: String = chars[start..i].iter().collect();
            let mut rendered = word.clone();
            for (k, v) in GREEK {
                if let Some(rest) = word.strip_prefix(k) {
                    if rest.is_empty() || rest.starts_with('_') || rest.chars().all(|c| c.is_ascii_digit()) {
                        rendered = format!("{v}{}", rest.replace('_', ""));
                        break;
                    }
                }
            }
            if let Some((a, b)) = rendered.split_once('_') {
                rendered = format!("{a}{}", subscript(b));
            }
            out.push_str(&rendered);
            continue;
        }
        match c {
            '*' => out.push('·'),
            '^' => {
                let mut j = i + 1;
                let mut exp = String::new();
                if chars.get(j) == Some(&'(') {
                    j += 1;
                    while j < chars.len() && chars[j] != ')' {
                        exp.push(chars[j]);
                        j += 1;
                    }
                    j += 1;
                } else {
                    if chars.get(j) == Some(&'-') {
                        exp.push('-');
                        j += 1;
                    }
                    while j < chars.len() && (chars[j].is_ascii_digit() || chars[j] == '.') {
                        exp.push(chars[j]);
                        j += 1;
                    }
                }
                if exp.replace('/', "").chars().all(|c| c.is_ascii_digit() || c == '-') && !exp.is_empty() {
                    out.push_str(&superscript(&exp));
                } else {
                    out.push('^');
                    out.push_str(&exp);
                }
                i = j;
                continue;
            }
            _ => out.push(c),
        }
        i += 1;
    }
    out.replace("sqrt(", "√(").replace("  ", " ")
}

fn superscript(s: &str) -> String {
    s.chars()
        .map(|c| match c {
            '0' => '⁰', '1' => '¹', '2' => '²', '3' => '³', '4' => '⁴', '5' => '⁵', '6' => '⁶', '7' => '⁷', '8' => '⁸', '9' => '⁹',
            '-' => '⁻', '/' => '⁄', c => c,
        })
        .collect()
}

fn subscript(s: &str) -> String {
    s.chars()
        .map(|c| match c {
            '0' => '₀', '1' => '₁', '2' => '₂', '3' => '₃', '4' => '₄', '5' => '₅', '6' => '₆', '7' => '₇', '8' => '₈', '9' => '₉',
            c => c,
        })
        .collect()
}
