use serde::Deserialize;
use std::sync::OnceLock;

const FILES: &[&str] = &[include_str!("../data/codata.toml"), include_str!("../data/constants.toml")];

#[derive(Debug, Clone, Deserialize)]
pub struct Constant {
    pub name: String,
    #[serde(default)]
    pub symbol: Option<String>,
    pub value: f64,
    #[serde(default)]
    pub uncertainty: f64,
    #[serde(default)]
    pub exact: bool,
    pub unit: String,
    pub category: String,
    pub source: String,
}

#[derive(Deserialize)]
struct File {
    constant: Vec<Constant>,
}

pub fn all() -> &'static [Constant] {
    static ALL: OnceLock<Vec<Constant>> = OnceLock::new();
    ALL.get_or_init(|| {
        FILES
            .iter()
            .flat_map(|f| toml::from_str::<File>(f).expect("constants data parses").constant)
            .collect()
    })
}

pub fn categories() -> Vec<&'static str> {
    let mut out: Vec<&'static str> = Vec::new();
    for c in all() {
        if !out.contains(&c.category.as_str()) {
            out.push(&c.category);
        }
    }
    out
}

pub fn find(query: &str) -> Vec<&'static Constant> {
    let q = query.trim();
    let exact: Vec<_> = all()
        .iter()
        .filter(|c| c.symbol.as_deref() == Some(q) || c.name.eq_ignore_ascii_case(q))
        .collect();
    if !exact.is_empty() {
        return exact;
    }
    let words: Vec<String> = q.to_lowercase().split_whitespace().map(str::to_string).collect();
    all()
        .iter()
        .filter(|c| {
            let hay = format!("{} {}", c.name, c.symbol.as_deref().unwrap_or("")).to_lowercase();
            !words.is_empty() && words.iter().all(|w| hay.contains(w.as_str()))
        })
        .collect()
}

impl Constant {
    pub fn with_unit(&self) -> String {
        let v = crate::engine::fmt_sig(self.value, 12);
        if self.unit.is_empty() { v } else { format!("{v} {}", self.unit) }
    }

    pub fn uncertainty_text(&self) -> String {
        if self.exact {
            "exact".into()
        } else if self.uncertainty == 0.0 {
            "not given".into()
        } else {
            crate::engine::fmt_sig(self.uncertainty, 3)
        }
    }

    pub fn line(&self) -> String {
        let sym = self.symbol.as_deref().map(|s| format!("{s}: ")).unwrap_or_default();
        format!("{sym}{} = {} ({}, {})", self.name, self.with_unit(), self.uncertainty_text(), self.source)
    }
}
