const E3: &[f64] = &[1.0, 2.2, 4.7];
const E6: &[f64] = &[1.0, 1.5, 2.2, 3.3, 4.7, 6.8];
const E12: &[f64] = &[1.0, 1.2, 1.5, 1.8, 2.2, 2.7, 3.3, 3.9, 4.7, 5.6, 6.8, 8.2];
const E24: &[f64] = &[
    1.0, 1.1, 1.2, 1.3, 1.5, 1.6, 1.8, 2.0, 2.2, 2.4, 2.7, 3.0, 3.3, 3.6, 3.9, 4.3, 4.7, 5.1, 5.6, 6.2, 6.8, 7.5, 8.2, 9.1,
];
const E48: &[f64] = &[
    1.00, 1.05, 1.10, 1.15, 1.21, 1.27, 1.33, 1.40, 1.47, 1.54, 1.62, 1.69, 1.78, 1.87, 1.96, 2.05, 2.15, 2.26, 2.37, 2.49, 2.61, 2.74,
    2.87, 3.01, 3.16, 3.32, 3.48, 3.65, 3.83, 4.02, 4.22, 4.42, 4.64, 4.87, 5.11, 5.36, 5.62, 5.90, 6.19, 6.49, 6.81, 7.15, 7.50, 7.87,
    8.25, 8.66, 9.09, 9.53,
];
const E96: &[f64] = &[
    1.00, 1.02, 1.05, 1.07, 1.10, 1.13, 1.15, 1.18, 1.21, 1.24, 1.27, 1.30, 1.33, 1.37, 1.40, 1.43, 1.47, 1.50, 1.54, 1.58, 1.62, 1.65,
    1.69, 1.74, 1.78, 1.82, 1.87, 1.91, 1.96, 2.00, 2.05, 2.10, 2.15, 2.21, 2.26, 2.32, 2.37, 2.43, 2.49, 2.55, 2.61, 2.67, 2.74, 2.80,
    2.87, 2.94, 3.01, 3.09, 3.16, 3.24, 3.32, 3.40, 3.48, 3.57, 3.65, 3.74, 3.83, 3.92, 4.02, 4.12, 4.22, 4.32, 4.42, 4.53, 4.64, 4.75,
    4.87, 4.99, 5.11, 5.23, 5.36, 5.49, 5.62, 5.76, 5.90, 6.04, 6.19, 6.34, 6.49, 6.65, 6.81, 6.98, 7.15, 7.32, 7.50, 7.68, 7.87, 8.06,
    8.25, 8.45, 8.66, 8.87, 9.09, 9.31, 9.53, 9.76,
];

const E192: &[f64] = &[
    1.00, 1.01, 1.02, 1.04, 1.05, 1.06, 1.07, 1.09, 1.10, 1.11, 1.13, 1.14, 1.15, 1.17, 1.18, 1.20, 1.21, 1.23, 1.24, 1.26, 1.27, 1.29, 1.30, 1.32, 1.33, 1.35, 1.37, 1.38, 1.40, 1.42, 1.43, 1.45, 1.47, 1.49, 1.50, 1.52, 1.54, 1.56, 1.58, 1.60, 1.62, 1.64, 1.65, 1.67, 1.69, 1.72, 1.74, 1.76, 1.78, 1.80, 1.82, 1.84, 1.87, 1.89, 1.91, 1.93, 1.96, 1.98, 2.00, 2.03, 2.05, 2.08, 2.10, 2.13, 2.15, 2.18, 2.21, 2.23, 2.26, 2.29, 2.32, 2.34, 2.37, 2.40, 2.43, 2.46, 2.49, 2.52, 2.55, 2.58, 2.61, 2.64, 2.67, 2.71, 2.74, 2.77, 2.80, 2.84, 2.87, 2.91, 2.94, 2.98, 3.01, 3.05, 3.09, 3.12, 3.16, 3.20, 3.24, 3.28, 3.32, 3.36, 3.40, 3.44, 3.48, 3.52, 3.57, 3.61, 3.65, 3.70, 3.74, 3.79, 3.83, 3.88, 3.92, 3.97, 4.02, 4.07, 4.12, 4.17, 4.22, 4.27, 4.32, 4.37, 4.42, 4.48, 4.53, 4.59, 4.64, 4.70, 4.75, 4.81, 4.87, 4.93, 4.99, 5.05, 5.11, 5.17, 5.23, 5.30, 5.36, 5.42, 5.49, 5.56, 5.62, 5.69, 5.76, 5.83, 5.90, 5.97, 6.04, 6.12, 6.19, 6.26, 6.34, 6.42, 6.49, 6.57, 6.65, 6.73, 6.81, 6.90, 6.98, 7.06, 7.15, 7.23, 7.32, 7.41, 7.50, 7.59, 7.68, 7.77, 7.87, 7.96, 8.06, 8.16, 8.25, 8.35, 8.45, 8.56, 8.66, 8.76, 8.87, 8.98, 9.09, 9.20, 9.31, 9.42, 9.53, 9.65, 9.76, 9.88,
];

pub const SERIES: &[&str] = &["E3", "E6", "E12", "E24", "E48", "E96", "E192"];

pub fn series(name: &str) -> Option<Vec<f64>> {
    Some(match name.to_ascii_uppercase().as_str() {
        "E3" => E3.to_vec(),
        "E6" => E6.to_vec(),
        "E12" => E12.to_vec(),
        "E24" => E24.to_vec(),
        "E48" => E48.to_vec(),
        "E96" => E96.to_vec(),
        "E192" => E192.to_vec(),
        _ => return None,
    })
}

pub fn nearest(series_name: &str, value: f64) -> Option<f64> {
    let table = series(series_name)?;
    if value <= 0.0 || !value.is_finite() {
        return None;
    }
    let decade = value.log10().floor();
    let mut best = f64::NAN;
    let mut best_err = f64::INFINITY;
    for d in [decade - 1.0, decade, decade + 1.0] {
        for m in &table {
            let candidate = m * 10f64.powf(d);
            let err = (candidate / value).ln().abs();
            if err < best_err {
                best_err = err;
                best = (candidate * 1e9).round() / 1e9;
            }
        }
    }
    Some(best)
}

pub struct DividerPair {
    pub r1: f64,
    pub r2: f64,
    pub ratio: f64,
    pub error: f64,
}

pub fn divider_pairs(series_name: &str, ratio: f64, low: f64, high: f64, count: usize) -> Vec<DividerPair> {
    let Some(table) = series(series_name) else { return Vec::new() };
    if !(ratio > 0.0 && ratio < 1.0) {
        return Vec::new();
    }
    let mut values = Vec::new();
    let mut d = low.log10().floor();
    while d <= high.log10().ceil() {
        for m in &table {
            let v = m * 10f64.powf(d);
            if v >= low * 0.999 && v <= high * 1.001 {
                values.push(v);
            }
        }
        d += 1.0;
    }
    let mut out = Vec::new();
    for &r1 in &values {
        for &r2 in &values {
            let got = r2 / (r1 + r2);
            out.push(DividerPair { r1, r2, ratio: got, error: (got - ratio) / ratio });
        }
    }
    out.sort_by(|a, b| a.error.abs().partial_cmp(&b.error.abs()).unwrap().then(a.r1.partial_cmp(&b.r1).unwrap()));
    out.truncate(count);
    out
}

pub const COLOURS: &[&str] = &["black", "brown", "red", "orange", "yellow", "green", "blue", "violet", "grey", "white"];

pub fn tolerance_colour(tol: f64) -> Option<&'static str> {
    Some(match (tol * 100.0).round() as i64 {
        100 => "brown",
        200 => "red",
        50 => "green",
        25 => "blue",
        10 => "violet",
        500 => "gold",
        1000 => "silver",
        _ => return None,
    })
}

pub fn colour_bands(ohms: f64, tolerance: f64, bands: usize) -> Result<Vec<&'static str>, String> {
    if !(4..=5).contains(&bands) {
        return Err("only 4 and 5 band codes are supported".into());
    }
    if ohms <= 0.0 {
        return Err("resistance must be positive".into());
    }
    let sig = bands - 2;
    let mut exp = ohms.log10().floor() as i32 - (sig as i32 - 1);
    let mut digits = (ohms / 10f64.powi(exp)).round() as i64;
    if digits >= 10i64.pow(sig as u32) {
        digits /= 10;
        exp += 1;
    }
    if (digits as f64 * 10f64.powi(exp) - ohms).abs() > ohms * 1e-9 {
        return Err(format!("{ohms} ohm needs more than {sig} significant digits"));
    }
    if !(-2..=9).contains(&exp) {
        return Err("multiplier out of range".into());
    }
    let mut out: Vec<&'static str> = digits
        .to_string()
        .chars()
        .map(|c| COLOURS[c.to_digit(10).unwrap() as usize])
        .collect();
    out.push(match exp {
        -1 => "gold",
        -2 => "silver",
        e => COLOURS[e as usize],
    });
    out.push(tolerance_colour(tolerance).ok_or("tolerance must be 0.1, 0.25, 0.5, 1, 2, 5 or 10 percent")?);
    Ok(out)
}

pub fn colour_value(names: &[&str]) -> Result<(f64, f64), String> {
    if !(4..=5).contains(&names.len()) {
        return Err("give 4 or 5 colours".into());
    }
    let sig = names.len() - 2;
    let digit = |n: &str| COLOURS.iter().position(|c| *c == n).map(|i| i as f64).ok_or_else(|| format!("{n} is not a digit colour"));
    let mut value = 0.0;
    for n in &names[..sig] {
        value = value * 10.0 + digit(n)?;
    }
    let exp = match names[sig] {
        "gold" => -1.0,
        "silver" => -2.0,
        n => digit(n)?,
    };
    let tol = match names[sig + 1] {
        "brown" => 1.0,
        "red" => 2.0,
        "green" => 0.5,
        "blue" => 0.25,
        "violet" => 0.1,
        "gold" => 5.0,
        "silver" => 10.0,
        n => return Err(format!("{n} is not a tolerance colour")),
    };
    Ok((value * 10f64.powf(exp), tol))
}

pub fn capacitor_code(code: &str) -> Result<f64, String> {
    let c = code.trim();
    if c.len() == 3 && c.chars().all(|c| c.is_ascii_digit()) {
        let mantissa: f64 = c[..2].parse().unwrap();
        let exp: i32 = c[2..].parse().unwrap();
        let e = if exp == 8 { -2 } else if exp == 9 { -1 } else { exp };
        return Ok(mantissa * 10f64.powi(e) * 1e-12);
    }
    if let Some(i) = c.find(|ch: char| matches!(ch, 'p' | 'n' | 'u' | 'P' | 'N' | 'U')) {
        let scale = match c.as_bytes()[i].to_ascii_lowercase() {
            b'p' => 1e-12,
            b'n' => 1e-9,
            _ => 1e-6,
        };
        let text = format!("{}.{}", &c[..i], &c[i + 1..]);
        let text = text.trim_end_matches('.');
        return text.parse::<f64>().map(|v| v * scale).map_err(|_| format!("cannot read {code}"));
    }
    Err(format!("cannot read {code}: use a 3 digit code like 104 or 4n7"))
}

pub fn capacitor_to_code(farads: f64) -> Result<String, String> {
    if farads <= 0.0 {
        return Err("capacitance must be positive".into());
    }
    let pf = farads * 1e12;
    if pf < 10.0 {
        return Err("below 10 pF the code is the value itself".into());
    }
    let exp = pf.log10().floor() as i32 - 1;
    let mantissa = (pf / 10f64.powi(exp)).round() as i64;
    if (mantissa as f64 * 10f64.powi(exp) - pf).abs() > pf * 1e-6 || exp > 5 {
        return Err("not expressible as a two digit code".into());
    }
    Ok(format!("{mantissa}{exp}"))
}

pub fn parse_int(text: &str) -> Result<i128, String> {
    let t = text.trim().replace('_', "");
    let (neg, t) = match t.strip_prefix('-') {
        Some(r) => (true, r.to_string()),
        None => (false, t),
    };
    let lower = t.to_lowercase();
    let v = if let Some(h) = lower.strip_prefix("0x") {
        i128::from_str_radix(h, 16)
    } else if let Some(b) = lower.strip_prefix("0b") {
        i128::from_str_radix(b, 2)
    } else if let Some(o) = lower.strip_prefix("0o") {
        i128::from_str_radix(o, 8)
    } else {
        lower.parse()
    }
    .map_err(|_| format!("cannot read {text} as an integer"))?;
    Ok(if neg { -v } else { v })
}

pub struct Bases {
    pub decimal: String,
    pub hex: String,
    pub octal: String,
    pub binary: String,
}

pub fn bases(value: i128, bits: u32) -> Bases {
    let mask: u128 = if bits >= 128 { u128::MAX } else { (1u128 << bits) - 1 };
    let u = (value as u128) & mask;
    let width = bits as usize;
    let bin = format!("{u:0width$b}");
    let groups: Vec<String> = bin.as_bytes().rchunks(4).rev().map(|c| String::from_utf8_lossy(c).to_string()).collect();
    Bases {
        decimal: value.to_string(),
        hex: format!("{u:X}"),
        octal: format!("{u:o}"),
        binary: groups.join(" "),
    }
}

pub fn twos_complement(value: i128, bits: u32) -> Result<String, String> {
    let min = -(1i128 << (bits - 1));
    let max = (1i128 << (bits - 1)) - 1;
    if value < min || value > max {
        return Err(format!("{value} does not fit in {bits} bits signed ({min} to {max})"));
    }
    let mask: u128 = (1u128 << bits) - 1;
    let u = (value as u128) & mask;
    Ok(format!("{u:0width$b}", width = bits as usize))
}

pub fn from_twos_complement(bits_text: &str) -> Result<i128, String> {
    let t: String = bits_text.chars().filter(|c| !c.is_whitespace() && *c != '_').collect();
    let n = t.len() as u32;
    if n == 0 || n > 127 || !t.chars().all(|c| c == '0' || c == '1') {
        return Err("give a string of 0 and 1".into());
    }
    let u = u128::from_str_radix(&t, 2).unwrap() as i128;
    Ok(if u >> (n - 1) & 1 == 1 { u - (1i128 << n) } else { u })
}

pub struct Float {
    pub sign: u8,
    pub exponent: String,
    pub mantissa: String,
    pub hex: String,
    pub class: &'static str,
    pub exact: String,
}

pub fn float32(v: f32) -> Float {
    let b = v.to_bits();
    let class = if v.is_nan() {
        "nan"
    } else if v.is_infinite() {
        "infinity"
    } else if v == 0.0 {
        "zero"
    } else if v.is_subnormal() {
        "subnormal"
    } else {
        "normal"
    };
    Float {
        sign: (b >> 31) as u8,
        exponent: format!("{:08b}", (b >> 23) & 0xFF),
        mantissa: format!("{:023b}", b & 0x7F_FFFF),
        hex: format!("{b:08X}"),
        class,
        exact: format!("{}", v as f64),
    }
}

pub fn float64(v: f64) -> Float {
    let b = v.to_bits();
    let class = if v.is_nan() {
        "nan"
    } else if v.is_infinite() {
        "infinity"
    } else if v == 0.0 {
        "zero"
    } else if v.is_subnormal() {
        "subnormal"
    } else {
        "normal"
    };
    Float {
        sign: (b >> 63) as u8,
        exponent: format!("{:011b}", (b >> 52) & 0x7FF),
        mantissa: format!("{:052b}", b & 0xF_FFFF_FFFF_FFFF),
        hex: format!("{b:016X}"),
        class,
        exact: format!("{v}"),
    }
}

pub fn float32_from_hex(hex: &str) -> Result<f32, String> {
    u32::from_str_radix(hex.trim().trim_start_matches("0x"), 16).map(f32::from_bits).map_err(|_| "give up to 8 hex digits".to_string())
}

pub fn float64_from_hex(hex: &str) -> Result<f64, String> {
    u64::from_str_radix(hex.trim().trim_start_matches("0x"), 16).map(f64::from_bits).map_err(|_| "give up to 16 hex digits".to_string())
}

pub fn bitwise(op: &str, a: i128, b: i128, bits: u32) -> Result<i128, String> {
    let mask: i128 = if bits >= 127 { i128::MAX } else { (1i128 << bits) - 1 };
    let r = match op {
        "and" | "&" => a & b,
        "or" | "|" => a | b,
        "xor" | "^" => a ^ b,
        "nand" => !(a & b) & mask,
        "nor" => !(a | b) & mask,
        "not" | "~" => !a & mask,
        "shl" | "<<" => a.checked_shl(b as u32).ok_or("shift too large")? & mask,
        "shr" | ">>" => a >> b,
        "rol" => {
            let b = (b as u32) % bits;
            ((a << b) | ((a & mask) >> (bits - b))) & mask
        }
        "ror" => {
            let b = (b as u32) % bits;
            (((a & mask) >> b) | (a << (bits - b))) & mask
        }
        other => return Err(format!("unknown operation {other}")),
    };
    Ok(r)
}

pub struct Stats {
    pub count: usize,
    pub mean: f64,
    pub median: f64,
    pub min: f64,
    pub max: f64,
    pub variance_sample: f64,
    pub variance_population: f64,
    pub stdev_sample: f64,
    pub stdev_population: f64,
}

pub fn stats(values: &[f64]) -> Result<Stats, String> {
    if values.is_empty() {
        return Err("no values".into());
    }
    let n = values.len();
    let mean = values.iter().sum::<f64>() / n as f64;
    let ss: f64 = values.iter().map(|v| (v - mean).powi(2)).sum();
    let mut sorted = values.to_vec();
    sorted.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let median = if n % 2 == 1 { sorted[n / 2] } else { (sorted[n / 2 - 1] + sorted[n / 2]) / 2.0 };
    let vp = ss / n as f64;
    let vs = if n > 1 { ss / (n - 1) as f64 } else { f64::NAN };
    Ok(Stats { count: n, mean, median, min: sorted[0], max: sorted[n - 1], variance_sample: vs, variance_population: vp, stdev_sample: vs.sqrt(), stdev_population: vp.sqrt() })
}
