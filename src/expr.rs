use std::collections::HashMap;

#[derive(Debug, Clone)]
pub enum Expr {
    Num(f64),
    Var(usize),
    Neg(Box<Expr>),
    Bin(char, Box<Expr>, Box<Expr>),
    Call(Func, Vec<Expr>),
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Func {
    Sqrt,
    Cbrt,
    Exp,
    Ln,
    Log10,
    Log2,
    Sin,
    Cos,
    Tan,
    Asin,
    Acos,
    Atan,
    Atan2,
    Sinh,
    Cosh,
    Tanh,
    Acosh,
    Asinh,
    Abs,
    Min,
    Max,
    Erf,
    NormCdf,
    TCdf,
    Floor,
    Ceil,
}

fn func(name: &str) -> Option<(Func, usize)> {
    Some(match name {
        "sqrt" => (Func::Sqrt, 1),
        "cbrt" => (Func::Cbrt, 1),
        "exp" => (Func::Exp, 1),
        "ln" => (Func::Ln, 1),
        "log10" | "log" => (Func::Log10, 1),
        "log2" => (Func::Log2, 1),
        "sin" => (Func::Sin, 1),
        "cos" => (Func::Cos, 1),
        "tan" => (Func::Tan, 1),
        "asin" => (Func::Asin, 1),
        "acos" => (Func::Acos, 1),
        "atan" => (Func::Atan, 1),
        "atan2" => (Func::Atan2, 2),
        "sinh" => (Func::Sinh, 1),
        "cosh" => (Func::Cosh, 1),
        "tanh" => (Func::Tanh, 1),
        "acosh" => (Func::Acosh, 1),
        "asinh" => (Func::Asinh, 1),
        "abs" => (Func::Abs, 1),
        "min" => (Func::Min, 2),
        "max" => (Func::Max, 2),
        "erf" => (Func::Erf, 1),
        "normcdf" => (Func::NormCdf, 1),
        "tcdf" => (Func::TCdf, 2),
        "floor" => (Func::Floor, 1),
        "ceil" => (Func::Ceil, 1),
        _ => return None,
    })
}

pub const CONSTANTS: &[(&str, f64)] = &[
    ("pi", std::f64::consts::PI),
    ("euler", std::f64::consts::E),
    ("c0", 299_792_458.0),
    ("mu0", 1.256_637_061_27e-6),
    ("eps0", 8.854_187_8188e-12),
    ("Z0", 376.730_313_412),
    ("g0", 9.806_65),
    ("kB", 1.380_649e-23),
    ("qe", 1.602_176_634e-19),
    ("h_planck", 6.626_070_15e-34),
    ("sigma_sb", 5.670_374_419e-8),
    ("G_grav", 6.674_30e-11),
    ("N_A", 6.022_140_76e23),
    ("R_gas", 8.314_462_618),
    ("M_earth", 5.9722e24),
    ("R_earth", 6_371_008.8),
];

struct Parser<'a> {
    chars: Vec<char>,
    pos: usize,
    vars: &'a [String],
}

pub fn parse(src: &str, vars: &[String]) -> Result<Expr, String> {
    let mut p = Parser { chars: src.chars().collect(), pos: 0, vars };
    let e = p.sum()?;
    p.skip();
    if p.pos < p.chars.len() {
        return Err(format!("unexpected {:?} in {src:?}", p.chars[p.pos]));
    }
    Ok(e)
}

impl Parser<'_> {
    fn skip(&mut self) {
        while self.chars.get(self.pos).is_some_and(|c| c.is_whitespace()) {
            self.pos += 1;
        }
    }

    fn peek(&mut self) -> Option<char> {
        self.skip();
        self.chars.get(self.pos).copied()
    }

    fn sum(&mut self) -> Result<Expr, String> {
        let mut left = self.product()?;
        while let Some(c) = self.peek() {
            if c != '+' && c != '-' {
                break;
            }
            self.pos += 1;
            let right = self.product()?;
            left = Expr::Bin(c, Box::new(left), Box::new(right));
        }
        Ok(left)
    }

    fn product(&mut self) -> Result<Expr, String> {
        let mut left = self.unary()?;
        while let Some(c) = self.peek() {
            if c != '*' && c != '/' {
                break;
            }
            self.pos += 1;
            let right = self.unary()?;
            left = Expr::Bin(c, Box::new(left), Box::new(right));
        }
        Ok(left)
    }

    fn unary(&mut self) -> Result<Expr, String> {
        match self.peek() {
            Some('-') => {
                self.pos += 1;
                Ok(Expr::Neg(Box::new(self.unary()?)))
            }
            Some('+') => {
                self.pos += 1;
                self.unary()
            }
            _ => self.power(),
        }
    }

    fn power(&mut self) -> Result<Expr, String> {
        let base = self.atom()?;
        if self.peek() == Some('^') {
            self.pos += 1;
            let exp = self.unary()?;
            return Ok(Expr::Bin('^', Box::new(base), Box::new(exp)));
        }
        Ok(base)
    }

    fn atom(&mut self) -> Result<Expr, String> {
        let c = self.peek().ok_or("unexpected end")?;
        if c == '(' {
            self.pos += 1;
            let e = self.sum()?;
            if self.peek() != Some(')') {
                return Err("missing )".into());
            }
            self.pos += 1;
            return Ok(e);
        }
        if c.is_ascii_digit() || c == '.' {
            let start = self.pos;
            while self.chars.get(self.pos).is_some_and(|c| c.is_ascii_digit() || *c == '.') {
                self.pos += 1;
            }
            if matches!(self.chars.get(self.pos), Some('e' | 'E')) {
                let mut j = self.pos + 1;
                if matches!(self.chars.get(j), Some('+' | '-')) {
                    j += 1;
                }
                if self.chars.get(j).is_some_and(|c| c.is_ascii_digit()) {
                    while self.chars.get(j).is_some_and(|c| c.is_ascii_digit()) {
                        j += 1;
                    }
                    self.pos = j;
                }
            }
            let s: String = self.chars[start..self.pos].iter().collect();
            return s.parse().map(Expr::Num).map_err(|_| format!("bad number {s}"));
        }
        if c.is_alphabetic() || c == '_' {
            let start = self.pos;
            while self.chars.get(self.pos).is_some_and(|c| c.is_alphanumeric() || *c == '_') {
                self.pos += 1;
            }
            let name: String = self.chars[start..self.pos].iter().collect();
            if self.peek() == Some('(') {
                let (f, arity) = func(&name).ok_or_else(|| format!("unknown function {name}"))?;
                self.pos += 1;
                let mut args = vec![self.sum()?];
                while self.peek() == Some(',') {
                    self.pos += 1;
                    args.push(self.sum()?);
                }
                if self.peek() != Some(')') {
                    return Err("missing )".into());
                }
                self.pos += 1;
                if args.len() != arity {
                    return Err(format!("{name} takes {arity} arguments"));
                }
                return Ok(Expr::Call(f, args));
            }
            if let Some(i) = self.vars.iter().position(|v| *v == name) {
                return Ok(Expr::Var(i));
            }
            if let Some((_, v)) = CONSTANTS.iter().find(|(n, _)| *n == name) {
                return Ok(Expr::Num(*v));
            }
            return Err(format!("unknown name {name}"));
        }
        Err(format!("unexpected {c:?}"))
    }
}

pub fn erf(x: f64) -> f64 {
    if x.is_nan() {
        return x;
    }
    if x == 0.0 {
        return 0.0;
    }
    if x < 0.0 {
        return -erf(-x);
    }
    if x > 6.0 {
        return 1.0;
    }
    if x < 2.5 {
        let mut term = x;
        let mut sum = x;
        let mut n = 0.0;
        loop {
            n += 1.0;
            term *= -x * x / n;
            let add = term / (2.0 * n + 1.0);
            sum += add;
            if add.abs() <= 1e-17 * sum.abs() || n > 200.0 {
                break;
            }
        }
        return sum * 2.0 / std::f64::consts::PI.sqrt();
    }
    1.0 - erfc_cf(x)
}

fn erfc_cf(x: f64) -> f64 {
    let mut f = 0.0;
    for k in (1..=60).rev() {
        f = (k as f64 / 2.0) / (x + f);
    }
    (-x * x).exp() / std::f64::consts::PI.sqrt() / (x + f)
}

pub fn norm_cdf(x: f64) -> f64 {
    0.5 * (1.0 + erf(x / std::f64::consts::SQRT_2))
}

fn ln_gamma(x: f64) -> f64 {
    const C: [f64; 6] = [76.18009172947146, -86.50532032941677, 24.01409824083091, -1.231739572450155, 0.1208650973866179e-2, -0.5395239384953e-5];
    let mut y = x;
    let tmp = x + 5.5;
    let tmp = tmp - (x + 0.5) * tmp.ln();
    let mut ser = 1.000000000190015;
    for c in C {
        y += 1.0;
        ser += c / y;
    }
    -tmp + (2.5066282746310005 * ser / x).ln()
}

fn betacf(a: f64, b: f64, x: f64) -> f64 {
    let tiny = 1e-300;
    let (qab, qap, qam) = (a + b, a + 1.0, a - 1.0);
    let mut c = 1.0;
    let mut d = 1.0 - qab * x / qap;
    if d.abs() < tiny {
        d = tiny;
    }
    d = 1.0 / d;
    let mut h = d;
    for m in 1..400 {
        let m = m as f64;
        let m2 = 2.0 * m;
        let aa = m * (b - m) * x / ((qam + m2) * (a + m2));
        d = 1.0 + aa * d;
        if d.abs() < tiny {
            d = tiny;
        }
        c = 1.0 + aa / c;
        if c.abs() < tiny {
            c = tiny;
        }
        d = 1.0 / d;
        h *= d * c;
        let aa = -(a + m) * (qab + m) * x / ((a + m2) * (qap + m2));
        d = 1.0 + aa * d;
        if d.abs() < tiny {
            d = tiny;
        }
        c = 1.0 + aa / c;
        if c.abs() < tiny {
            c = tiny;
        }
        d = 1.0 / d;
        let del = d * c;
        h *= del;
        if (del - 1.0).abs() < 1e-16 {
            break;
        }
    }
    h
}

fn inc_beta(a: f64, b: f64, x: f64) -> f64 {
    if x <= 0.0 {
        return 0.0;
    }
    if x >= 1.0 {
        return 1.0;
    }
    let bt = (ln_gamma(a + b) - ln_gamma(a) - ln_gamma(b) + a * x.ln() + b * (1.0 - x).ln()).exp();
    if x < (a + 1.0) / (a + b + 2.0) {
        bt * betacf(a, b, x) / a
    } else {
        1.0 - bt * betacf(b, a, 1.0 - x) / b
    }
}

pub fn t_cdf(t: f64, nu: f64) -> f64 {
    let x = nu / (nu + t * t);
    let tail = 0.5 * inc_beta(nu / 2.0, 0.5, x);
    if t >= 0.0 { 1.0 - tail } else { tail }
}

impl Expr {
    pub fn eval(&self, vars: &[f64]) -> f64 {
        match self {
            Expr::Num(n) => *n,
            Expr::Var(i) => vars[*i],
            Expr::Neg(e) => -e.eval(vars),
            Expr::Bin(op, a, b) => {
                let (a, b) = (a.eval(vars), b.eval(vars));
                match op {
                    '+' => a + b,
                    '-' => a - b,
                    '*' => a * b,
                    '/' => a / b,
                    _ => a.powf(b),
                }
            }
            Expr::Call(f, args) => {
                let a = args[0].eval(vars);
                let b = args.get(1).map(|e| e.eval(vars)).unwrap_or(0.0);
                match f {
                    Func::Sqrt => a.sqrt(),
                    Func::Cbrt => a.cbrt(),
                    Func::Exp => a.exp(),
                    Func::Ln => a.ln(),
                    Func::Log10 => a.log10(),
                    Func::Log2 => a.log2(),
                    Func::Sin => a.sin(),
                    Func::Cos => a.cos(),
                    Func::Tan => a.tan(),
                    Func::Asin => a.asin(),
                    Func::Acos => a.acos(),
                    Func::Atan => a.atan(),
                    Func::Atan2 => a.atan2(b),
                    Func::Sinh => a.sinh(),
                    Func::Cosh => a.cosh(),
                    Func::Tanh => a.tanh(),
                    Func::Acosh => a.acosh(),
                    Func::Asinh => a.asinh(),
                    Func::Abs => a.abs(),
                    Func::Min => a.min(b),
                    Func::Max => a.max(b),
                    Func::Erf => erf(a),
                    Func::NormCdf => norm_cdf(a),
                    Func::TCdf => t_cdf(a, b),
                    Func::Floor => a.floor(),
                    Func::Ceil => a.ceil(),
                }
            }
        }
    }
}

pub fn eval_str(src: &str, vars: &HashMap<String, f64>) -> Result<f64, String> {
    let names: Vec<String> = vars.keys().cloned().collect();
    let values: Vec<f64> = names.iter().map(|n| vars[n]).collect();
    Ok(parse(src, &names)?.eval(&values))
}
