use fend_core::{Context, CustomUnitAttribute as Attr};

const KEYWORDS: &[&str] = &["to", "as", "in", "of", "sf", "dp", "mod", "per", "xor", "and", "or"];
const FUNCTIONS: &[&str] = &[
    "sin", "cos", "tan", "asin", "acos", "atan", "sinh", "cosh", "tanh", "ln", "log", "log2", "log10", "sqrt", "cbrt",
    "abs", "exp", "floor", "ceil", "round", "real", "imag", "conj", "arg", "sum", "min", "max",
];
const CONSTANTS: &[&str] = &["pi", "e", "i", "tau", "phi", "x", "y", "z", "t", "n"];

struct LogUnit {
    name: &'static str,
    reference: &'static str,
    factor: f64,
    divisor: f64,
}

const LOG_UNITS: &[LogUnit] = &[
    LogUnit { name: "dbm", reference: "mW", factor: 1.0, divisor: 10.0 },
    LogUnit { name: "dbw", reference: "W", factor: 1.0, divisor: 10.0 },
    LogUnit { name: "dbv", reference: "V", factor: 1.0, divisor: 20.0 },
    LogUnit { name: "dbmv", reference: "mV", factor: 1.0, divisor: 20.0 },
    LogUnit { name: "dbuv", reference: "uV", factor: 1.0, divisor: 20.0 },
    LogUnit { name: "dbu", reference: "V", factor: 0.774_596_669_241_483_4, divisor: 20.0 },
    LogUnit { name: "dbi", reference: "", factor: 1.0, divisor: 10.0 },
    LogUnit { name: "dbd", reference: "", factor: 1.640_590_4, divisor: 10.0 },
    LogUnit { name: "dbc", reference: "", factor: 1.0, divisor: 10.0 },
    LogUnit { name: "db", reference: "", factor: 1.0, divisor: 10.0 },
];

fn log_unit(name: &str) -> Option<&'static LogUnit> {
    let key = name.to_lowercase().replace(['µ', 'μ'], "u");
    LOG_UNITS.iter().find(|u| u.name == key)
}

#[derive(Debug, Clone, PartialEq)]
enum Tok {
    Num(String),
    Ident(String),
    Op(char),
}

#[derive(Debug, Clone)]
struct Token {
    tok: Tok,
    space: bool,
}

fn tokenize(src: &str) -> Vec<Token> {
    let chars: Vec<char> = src.chars().collect();
    let mut out = Vec::new();
    let mut i = 0;
    while i < chars.len() {
        let space = chars[i].is_whitespace();
        while i < chars.len() && chars[i].is_whitespace() {
            i += 1;
        }
        if i >= chars.len() {
            break;
        }
        let c = chars[i];
        let start = i;
        let tok = if c.is_ascii_digit() || (c == '.' && chars.get(i + 1).is_some_and(|d| d.is_ascii_digit())) {
            if c == '0' && matches!(chars.get(i + 1), Some('x' | 'X' | 'b' | 'B' | 'o' | 'O')) {
                i += 2;
                while i < chars.len() && (chars[i].is_ascii_hexdigit() || chars[i] == '_') {
                    i += 1;
                }
            } else {
                while i < chars.len() && (chars[i].is_ascii_digit() || chars[i] == '.' || chars[i] == '_') {
                    i += 1;
                }
                if i < chars.len() && (chars[i] == 'e' || chars[i] == 'E') {
                    let mut j = i + 1;
                    if j < chars.len() && (chars[j] == '+' || chars[j] == '-') {
                        j += 1;
                    }
                    if j < chars.len() && chars[j].is_ascii_digit() {
                        while j < chars.len() && chars[j].is_ascii_digit() {
                            j += 1;
                        }
                        i = j;
                    }
                }
            }
            Tok::Num(chars[start..i].iter().collect())
        } else if c.is_alphabetic() || matches!(c, '°' | '_' | '%' | 'Ω' | 'µ') {
            while i < chars.len() && (chars[i].is_alphanumeric() || matches!(chars[i], '°' | '_' | '%' | 'Ω' | 'µ')) {
                i += 1;
            }
            Tok::Ident(chars[start..i].iter().collect())
        } else {
            i += 1;
            Tok::Op(c)
        };
        out.push(Token { tok, space });
    }
    out
}

fn render(tokens: &[Token]) -> String {
    let mut s = String::new();
    for (i, t) in tokens.iter().enumerate() {
        if i > 0 && t.space {
            s.push(' ');
        }
        match &t.tok {
            Tok::Num(x) | Tok::Ident(x) => s.push_str(x),
            Tok::Op(c) => s.push(*c),
        }
    }
    s
}

fn is_unit_ident(t: &Tok) -> bool {
    match t {
        Tok::Ident(x) => {
            let l = x.to_lowercase();
            !KEYWORDS.contains(&l.as_str()) && !FUNCTIONS.contains(&l.as_str()) && !CONSTANTS.contains(&l.as_str())
        }
        _ => false,
    }
}

fn is_keyword(t: &Tok, words: &[&str]) -> bool {
    matches!(t, Tok::Ident(x) if words.contains(&x.to_lowercase().as_str()))
}

fn rename_idents(tokens: &mut Vec<Token>) {
    let mut i = 0;
    while i < tokens.len() {
        if let Tok::Ident(name) = tokens[i].tok.clone() {
            let mut name = name.replace(['µ', 'μ'], "u");
            let lower = name.to_lowercase();
            if let Some(prefix) = ["k", "m", "g", "t", "u", "n", "p"]
                .iter()
                .find(|p| lower == format!("{p}ohm") || lower == format!("{p}ohms"))
            {
                let p = if name.starts_with('M') { "M" } else if name.starts_with('G') { "G" } else { prefix };
                let p = if lower.starts_with('k') { "k" } else { p };
                name = format!("{p}Ω");
            }
            if let Some(long) = ["k:kilo", "m:milli", "u:micro", "n:nano", "p:pico"]
                .iter()
                .find_map(|p| name.strip_suffix('C').filter(|pre| pre.len() == 1 && p.starts_with(&format!("{pre}:"))).map(|_| &p[2..]))
            {
                name = format!("{long}coulomb");
            }
            if lower == "fl" && tokens.get(i + 1).is_some_and(|t| is_keyword_or_ident(&t.tok, "oz")) {
                tokens.remove(i + 1);
                name = "floz".into();
            }
            if lower == "in" {
                let prev_unit = i > 0 && (is_unit_ident(&tokens[i - 1].tok) || tokens[i - 1].tok == Tok::Op(')'));
                let next_unit = tokens.get(i + 1).is_some_and(|t| is_unit_ident(&t.tok));
                if !(prev_unit && next_unit) {
                    name = "inch".into();
                }
            }
            tokens[i].tok = Tok::Ident(name);
        }
        i += 1;
    }
}

fn mark_temperatures(tokens: &mut [Token]) {
    let scales = ["c", "f", "k", "°c", "°f", "celsius", "fahrenheit", "kelvin", "degc", "degf"];
    let count = tokens
        .iter()
        .filter(|t| matches!(&t.tok, Tok::Ident(x) if scales.contains(&x.to_lowercase().as_str())))
        .count();
    for t in tokens.iter_mut() {
        if let Tok::Ident(x) = &t.tok {
            match x.as_str() {
                "C" if count < 2 => t.tok = Tok::Ident("coulomb".into()),
                _ if count < 2 => {}
                "C" => t.tok = Tok::Ident("°C".into()),
                "F" => t.tok = Tok::Ident("°F".into()),
                _ => {}
            }
        }
    }
}

fn is_keyword_or_ident(t: &Tok, word: &str) -> bool {
    matches!(t, Tok::Ident(x) if x.eq_ignore_ascii_case(word))
}

fn join_compound(tokens: &mut Vec<Token>) {
    let mut i = 2;
    while i + 1 < tokens.len() {
        let shape = matches!(tokens[i].tok, Tok::Num(_))
            && is_unit_ident(&tokens[i - 1].tok)
            && matches!(tokens[i - 2].tok, Tok::Num(_))
            && is_unit_ident(&tokens[i + 1].tok);
        if shape {
            let space = tokens[i].space;
            tokens[i].space = true;
            tokens.insert(i, Token { tok: Tok::Op('+'), space });
            i += 1;
        }
        i += 1;
    }
}

fn wrap_divisors(tokens: &mut Vec<Token>) {
    let mut k = 0;
    while k + 2 < tokens.len() {
        let unit_before = tokens[..k].iter().any(|t| is_unit_ident(&t.tok));
        if tokens[k].tok == Tok::Op('/')
            && unit_before
            && matches!(tokens[k + 1].tok, Tok::Num(_))
            && is_unit_ident(&tokens[k + 2].tok)
        {
            let mut end = k + 2;
            while end + 1 < tokens.len() {
                let next = &tokens[end + 1].tok;
                let more = is_unit_ident(next)
                    || matches!(next, Tok::Num(_))
                    || *next == Tok::Op('^');
                if !more {
                    break;
                }
                end += 1;
            }
            tokens.insert(end + 1, Token { tok: Tok::Op(')'), space: false });
            let space = tokens[k + 1].space;
            tokens[k + 1].space = false;
            tokens.insert(k + 1, Token { tok: Tok::Op('('), space });
            k = end + 2;
        }
        k += 1;
    }
}

fn expand_fuel(tokens: &mut Vec<Token>) {
    let mut i = 0;
    while i + 3 < tokens.len() {
        let litre = ["l", "litre", "liter"].iter().any(|w| is_keyword_or_ident(&tokens[i].tok, w));
        let per_100 = tokens[i + 1].tok == Tok::Op('/') && matches!(&tokens[i + 2].tok, Tok::Num(n) if n == "100");
        if litre && per_100 && is_keyword_or_ident(&tokens[i + 3].tok, "km") {
            tokens.drain(i..i + 4);
            let space = i > 0;
            let new = [Tok::Op('('), Tok::Ident("L".into()), Tok::Op('/'), Tok::Op('('), Tok::Num("100".into()), Tok::Ident("km".into()), Tok::Op(')'), Tok::Op(')')];
            for (n, tok) in new.into_iter().enumerate() {
                let space = (n == 0 && space) || n == 5;
                tokens.insert(i + n, Token { tok, space });
            }
            i += 8;
            continue;
        }
        i += 1;
    }
}

pub fn normalise(src: &str) -> String {
    let mut tokens = tokenize(src);
    rename_idents(&mut tokens);
    mark_temperatures(&mut tokens);
    expand_fuel(&mut tokens);
    join_compound(&mut tokens);
    wrap_divisors(&mut tokens);
    render(&tokens)
}

fn split_conversion(tokens: &[Token]) -> (usize, Option<usize>) {
    let mut depth = 0i32;
    for (i, t) in tokens.iter().enumerate() {
        match &t.tok {
            Tok::Op('(') => depth += 1,
            Tok::Op(')') => depth -= 1,
            Tok::Ident(_) if depth == 0 && i > 0 && is_keyword(&t.tok, &["to", "as", "in"]) => {
                return (i, Some(i));
            }
            _ => {}
        }
    }
    (tokens.len(), None)
}

pub fn fmt_sig(v: f64, digits: usize) -> String {
    if v == 0.0 || !v.is_finite() {
        return fmt_number(v);
    }
    let exp = v.abs().log10().floor() as i32;
    if (-3..=9).contains(&exp) {
        let decimals = (digits as i32 - 1 - exp).max(0) as usize;
        let s = format!("{v:.decimals$}");
        if s.contains('.') { s.trim_end_matches('0').trim_end_matches('.').to_string() } else { s }
    } else {
        let s = format!("{v:.prec$e}", prec = digits - 1);
        let (m, e) = s.split_once('e').unwrap();
        let m = if m.contains('.') { m.trim_end_matches('0').trim_end_matches('.') } else { m };
        format!("{m}e{e}")
    }
}

pub fn fmt_number(v: f64) -> String {
    if v == 0.0 {
        return "0".into();
    }
    if !v.is_finite() {
        return v.to_string();
    }
    let exp = v.abs().log10().floor() as i32;
    if (-4..12).contains(&exp) {
        let decimals = (9 - exp).max(0) as usize;
        let s = format!("{v:.decimals$}");
        if s.contains('.') { s.trim_end_matches('0').trim_end_matches('.').to_string() } else { s }
    } else {
        let s = format!("{v:.9e}");
        let (m, e) = s.split_once('e').unwrap();
        let m = m.trim_end_matches('0').trim_end_matches('.');
        format!("{m}e{e}")
    }
}

pub struct Calc {
    ctx: Context,
}

impl Default for Calc {
    fn default() -> Self {
        Self::new()
    }
}

impl Calc {
    pub fn new() -> Self {
        let mut ctx = Context::new();
        ctx.disable_rng();
        let prefixed = Attr::AllowShortPrefix;
        for (name, def, attr) in [
            ("Ω", "ohm", &prefixed),
            ("P", "poise", &prefixed),
            ("St", "stokes", &prefixed),
            ("rem", "0.01 sievert", &prefixed),
            ("floz", "29.5735295625 ml", &Attr::None),
            ("ukgal", "4.54609 L", &Attr::None),
            ("ukfloz", "28.4130625 ml", &Attr::None),
            ("ukpt", "568.26125 ml", &Attr::None),
            ("kbit", "1000 bit", &Attr::None),
            ("Mbit", "1000000 bit", &Attr::None),
            ("Gbit", "1000000000 bit", &Attr::None),
            ("Tbit", "1000000000000 bit", &Attr::None),
            ("kbps", "1000 bit/s", &Attr::None),
            ("Mbps", "1000000 bit/s", &Attr::None),
            ("Gbps", "1000000000 bit/s", &Attr::None),
            ("mil", "0.001 inch", &Attr::None),
            ("thou", "0.001 inch", &Attr::None),
            ("oersted", "79.57747154594767 A/m", &Attr::None),
            ("gauss", "0.0001 tesla", &prefixed),
            ("maxwell", "1e-8 weber", &Attr::None),
            ("footcandle", "10.763910416709722 lux", &Attr::None),
            ("nit", "cd/m^2", &Attr::None),
            ("cfm", "0.00047194745 m^3/s", &Attr::None),
            ("gpm", "6.30901964e-5 m^3/s", &Attr::None),
            ("lpm", "1.6666666666666667e-5 m^3/s", &Attr::None),
            ("barg", "100000 Pa", &Attr::None),
            ("inHg", "3386.389 Pa", &Attr::None),
            ("inH2O", "249.08891 Pa", &Attr::None),
            ("mmH2O", "9.80665 Pa", &Attr::None),
            ("torr", "133.32236842105263 Pa", &Attr::None),
            ("ppm", "1e-6", &Attr::None),
            ("ppb", "1e-9", &Attr::None),
            ("ft_lbf", "1.3558179483314004 J", &Attr::None),
        ] {
            ctx.define_custom_unit_v1(name, name, def, attr);
        }
        Self { ctx }
    }

    fn raw(&mut self, src: &str) -> Result<String, String> {
        let r = fend_core::evaluate(src, &mut self.ctx)?;
        Ok(r.get_main_result().trim_start_matches("approx. ").to_string())
    }

    fn number(&mut self, src: &str) -> Result<f64, String> {
        let out = self.raw(&format!("({src}) to 17 sf"))?;
        parse_leading_number(&out).ok_or_else(|| format!("not a number: {out}"))
    }

    pub fn value_in(&mut self, src: &str, unit: &str) -> Result<f64, String> {
        let joined = if unit.trim().is_empty() { src.to_string() } else { format!("{src} to {unit}") };
        let normalised = normalise(&joined);
        let tokens = tokenize(&normalised);
        let (split, kw) = split_conversion(&tokens);
        let left = render(&tokens[..split]);
        let unit = kw.map(|k| render(&tokens[k + 1..])).unwrap_or_default();
        let unit = unit.as_str();
        if let Some(v) = self.log_to_linear_value(&left, unit)? {
            return Ok(v);
        }
        let expr = if unit.trim().is_empty() { format!("({left})") } else { format!("(({left}) to {unit})") };
        let out = self.raw(&format!("{expr} to 17 sf"))?;
        parse_leading_number(&out).ok_or_else(|| format!("not a number: {out}"))
    }

    fn log_to_linear_value(&mut self, src: &str, unit: &str) -> Result<Option<f64>, String> {
        let tokens = tokenize(src);
        let Some((lu, value)) = log_value(&tokens) else { return Ok(None) };
        let linear = lu.factor * 10f64.powf(value / lu.divisor);
        if lu.reference.is_empty() {
            if unit.trim().is_empty() { Ok(Some(linear)) } else { Err(format!("a {} value is a ratio, not {unit}", lu.name)) }
        } else if unit.trim().is_empty() {
            Err(format!("a {} value is in {}", lu.name, lu.reference))
        } else {
            self.number(&format!("{} {} to {unit}", fmt_number(linear), lu.reference)).map(Some)
        }
    }

    pub fn eval(&mut self, input: &str) -> Result<String, String> {
        let src = normalise(input.trim());
        if src.is_empty() {
            return Err("empty expression".into());
        }
        let tokens = tokenize(&src);
        let (split, kw) = split_conversion(&tokens);
        let left = render(&tokens[..split]);
        let target = kw.map(|k| render(&tokens[k + 1..]));
        if let Some(out) = self.log_path(&tokens[..split], &left, target.as_deref())? {
            return Ok(out);
        }
        if let Some(t) = &target {
            if let Some(out) = self.fuel_path(&left, t)? {
                return Ok(out);
            }
        }
        match self.raw(&src) {
            Ok(out) => Ok(out),
            Err(e) if target.is_some() && e.contains("incompatible") => {
                let t = target.unwrap();
                match self.raw(&format!("(1 / ({left})) to {t}")) {
                    Ok(out) => Ok(out),
                    Err(_) => Err(e),
                }
            }
            Err(e) => Err(e),
        }
    }

    fn log_path(&mut self, left_tokens: &[Token], left: &str, target: Option<&str>) -> Result<Option<String>, String> {
        let source = log_value(left_tokens);
        let target_log = target.and_then(|t| {
            let ts = tokenize(t);
            match ts.as_slice() {
                [one] => match &one.tok {
                    Tok::Ident(x) => log_unit(x),
                    _ => None,
                },
                _ => None,
            }
        });
        let linear: Option<(f64, &str)> = match source {
            Some((lu, v)) => Some((lu.factor * 10f64.powf(v / lu.divisor), lu.reference)),
            None => None,
        };
        match (linear, target_log, target) {
            (None, None, _) => Ok(None),
            (Some((lin, reference)), None, None) => {
                Ok(Some(if reference.is_empty() { fmt_number(lin) } else { format!("{} {reference}", fmt_number(lin)) }))
            }
            (Some((lin, reference)), None, Some(t)) => {
                if reference.is_empty() {
                    if t.trim().is_empty() || matches!(t.trim(), "x" | "ratio") {
                        Ok(Some(fmt_number(lin)))
                    } else {
                        Err(format!("a dB ratio has no unit {t}"))
                    }
                } else {
                    self.raw(&format!("{} {reference} to {t}", fmt_number(lin))).map(Some)
                }
            }
            (src, Some(tl), _) => {
                let lin = match src {
                    Some((lin, reference)) => {
                        if reference != tl.reference {
                            if reference.is_empty() || tl.reference.is_empty() {
                                return Err(format!("cannot convert between a ratio and {}", tl.name));
                            }
                            self.number(&format!("{} {reference} to {}", fmt_number(lin), tl.reference))?
                        } else {
                            lin
                        }
                    }
                    None => {
                        if tl.reference.is_empty() {
                            self.number(left)?
                        } else {
                            self.number(&format!("({left}) to {}", tl.reference))?
                        }
                    }
                };
                if lin <= 0.0 {
                    return Err("a level needs a positive quantity".into());
                }
                let db = tl.divisor * (lin / tl.factor).log10();
                let name = target.unwrap().trim().to_string();
                Ok(Some(format!("{} {name}", fmt_number(db))))
            }
        }
    }

    fn fuel_path(&mut self, left: &str, target: &str) -> Result<Option<String>, String> {
        if target.replace(' ', "").to_lowercase() != "(l/(100km))" {
            return Ok(None);
        }
        let per_km = match self.number(&format!("({left}) to L/km")) {
            Ok(v) => v,
            Err(_) => self.number(&format!("(1/({left})) to L/km"))?,
        };
        Ok(Some(format!("{} L/100km", fmt_number(per_km * 100.0))))
    }

    pub fn clear(&mut self) {
        *self = Self::new();
    }
}

fn log_value(tokens: &[Token]) -> Option<(&'static LogUnit, f64)> {
    let mut idx = 0;
    let mut sign = 1.0;
    if matches!(tokens.first().map(|t| &t.tok), Some(Tok::Op('-'))) {
        sign = -1.0;
        idx = 1;
    }
    match (tokens.get(idx).map(|t| &t.tok), tokens.get(idx + 1).map(|t| &t.tok), tokens.len() == idx + 2) {
        (Some(Tok::Num(n)), Some(Tok::Ident(u)), true) => {
            let lu = log_unit(u)?;
            let v: f64 = n.replace('_', "").parse().ok()?;
            Some((lu, sign * v))
        }
        _ => None,
    }
}

fn parse_leading_number(s: &str) -> Option<f64> {
    let first = s.split_whitespace().next()?;
    let first = first.trim_end_matches(|c: char| !c.is_ascii_digit() && c != '.');
    first.parse().ok()
}
