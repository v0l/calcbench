use crate::{constants, engine::Calc, formula, tables};

pub const HELP: &str = "calcbench: units, constants and formulas

usage:
  calcbench \"60 mph to m/s\"            evaluate an expression with units
  calcbench calc <expression>         the same, explicitly
  calcbench formula                   list every formula
  calcbench formula <id> [SYM=value ...]   show a formula, or solve for the one left out
  calcbench formula search <words>    find formulas
  calcbench const <name or symbol>    show constants, with value, uncertainty and source
  calcbench const list [category]     list constants
  calcbench eseries <E24> <value>     nearest standard value
  calcbench divider <E24> <ratio> [min max]   resistor pairs for Vout/Vin
  calcbench colour <ohms> [tolerance%] [bands]   resistor colour code
  calcbench colour <band> <band> ...  colours to resistance
  calcbench cap <104|4n7|100nF>       capacitor codes
  calcbench base <value> [bits]       bases and two's complement
  calcbench float <value|0xHEX> [32|64]   IEEE-754 fields
  calcbench bits <and|or|xor|shl|shr|not> <a> [b] [bits]
  calcbench stats <numbers...>        mean, median and deviations

options: --json for machine output on formula and const";

pub fn run(args: &[String]) -> Result<String, String> {
    let json = args.iter().any(|a| a == "--json");
    let args: Vec<String> = args.iter().filter(|a| *a != "--json").cloned().collect();
    let Some(first) = args.first() else { return Ok(HELP.to_string()) };
    let rest = &args[1..];
    match first.as_str() {
        "-h" | "--help" | "help" => Ok(HELP.to_string()),
        "-V" | "--version" | "version" => Ok(format!("calcbench {}", env!("CARGO_PKG_VERSION"))),
        "calc" => calc(&rest.join(" ")),
        "formula" | "formulas" => formulas(rest, json),
        "const" | "constant" | "constants" => consts(rest, json),
        "eseries" => eseries(rest),
        "divider" => divider(rest),
        "colour" | "color" => colour(rest),
        "cap" | "capacitor" => cap(rest),
        "base" | "bases" => base(rest),
        "float" => float(rest),
        "bits" | "bit" => bits(rest),
        "stats" => stats(rest),
        _ => calc(&args.join(" ")),
    }
}

fn calc(expr: &str) -> Result<String, String> {
    Calc::new().eval(expr)
}

fn quote(s: &str) -> String {
    let mut out = String::from("\"");
    for c in s.chars() {
        match c {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\n' => out.push_str("\\n"),
            c => out.push(c),
        }
    }
    out.push('"');
    out
}

fn formulas(args: &[String], json: bool) -> Result<String, String> {
    let Some(first) = args.first() else {
        return Ok(formula::categories()
            .into_iter()
            .map(|cat| {
                let rows: Vec<String> = formula::all()
                    .iter()
                    .filter(|f| f.category == cat)
                    .map(|f| format!("  {:<28} {}", f.id, f.name))
                    .collect();
                format!("{cat}\n{}", rows.join("\n"))
            })
            .collect::<Vec<_>>()
            .join("\n\n"));
    };
    if first == "search" {
        let hits = formula::search(&args[1..].join(" "));
        if hits.is_empty() {
            return Err("no formula matches".into());
        }
        return Ok(hits.iter().map(|f| format!("{:<28} {}", f.id, f.name)).collect::<Vec<_>>().join("\n"));
    }
    let f = formula::get(first).ok_or_else(|| {
        let mut near = formula::search(first);
        near.extend(formula::all().iter().filter(|f| first.contains(f.id.as_str()) || f.id.starts_with(first.as_str())));
        if near.is_empty() {
            format!("no formula {first}: try `calcbench formula search <words>`")
        } else {
            format!("no formula {first}: did you mean {}", near.iter().take(5).map(|f| f.id.as_str()).collect::<Vec<_>>().join(", "))
        }
    })?;
    let inputs: Vec<(String, String)> = args[1..]
        .iter()
        .map(|a| {
            a.split_once('=')
                .map(|(k, v)| (k.trim().to_string(), v.trim().to_string()))
                .ok_or_else(|| format!("{a}: write inputs as SYMBOL=value"))
        })
        .collect::<Result<_, _>>()?;
    if inputs.is_empty() {
        return Ok(describe(f, json));
    }
    let mut calc = Calc::new();
    let s = formula::solve_with_units(&mut calc, f, &inputs)?;
    if json {
        Ok(format!(
            "{{\"formula\":{},\"solved\":{},\"name\":{},\"value\":{},\"unit\":{},\"text\":{}}}",
            quote(&f.id),
            quote(&s.symbol),
            quote(&s.name),
            s.value,
            quote(&s.unit),
            quote(&s.engineering())
        ))
    } else {
        Ok(format!("{} = {}  ({})", s.symbol, s.engineering(), s.name))
    }
}

fn describe(f: &formula::Formula, json: bool) -> String {
    if json {
        let vars: Vec<String> = f
            .vars
            .iter()
            .map(|v| format!("{{\"symbol\":{},\"name\":{},\"unit\":{}}}", quote(&v.s), quote(&v.n), quote(&v.u)))
            .collect();
        return format!(
            "{{\"id\":{},\"name\":{},\"equation\":{},\"source\":{},\"variables\":[{}]}}",
            quote(&f.id),
            quote(&f.name),
            quote(&f.equation),
            quote(&f.source),
            vars.join(",")
        );
    }
    let mut out = format!("{} ({})\n  {}\n", f.name, f.id, formula::pretty(&f.equation));
    for v in &f.vars {
        out.push_str(&format!("  {:<6} {}{}\n", v.s, v.n, if v.u.is_empty() { String::new() } else { format!(" [{}]", v.u) }));
    }
    if !f.notes.is_empty() {
        out.push_str(&format!("{}\n", f.notes));
    }
    out.push_str(&format!("source: {}\nsolve: calcbench formula {} {}", f.source, f.id, example(f)));
    out
}

fn example(f: &formula::Formula) -> String {
    f.vars.iter().skip(1).map(|v| format!("{}={}", v.s, crate::engine::fmt_sig(v.x, 4))).collect::<Vec<_>>().join(" ")
}

fn consts(args: &[String], json: bool) -> Result<String, String> {
    if args.is_empty() {
        return Err("name a constant, or use `const list`".into());
    }
    if args[0] == "list" {
        let cat = args.get(1).map(|s| s.as_str());
        if let Some(c) = cat {
            if !constants::categories().contains(&c) {
                return Err(format!("categories: {}", constants::categories().join(", ")));
            }
        } else {
            return Ok(format!("categories: {}", constants::categories().join(", ")));
        }
        return Ok(constants::all().iter().filter(|c| Some(c.category.as_str()) == cat).map(|c| c.line()).collect::<Vec<_>>().join("\n"));
    }
    let hits = constants::find(&args.join(" "));
    if hits.is_empty() {
        return Err(format!("no constant matches {}", args.join(" ")));
    }
    let shown: Vec<_> = hits.iter().take(25).collect();
    if json {
        let rows: Vec<String> = shown
            .iter()
            .map(|c| {
                format!(
                    "{{\"name\":{},\"symbol\":{},\"value\":{:e},\"uncertainty\":{:e},\"exact\":{},\"unit\":{},\"source\":{}}}",
                    quote(&c.name),
                    quote(c.symbol.as_deref().unwrap_or("")),
                    c.value,
                    c.uncertainty,
                    c.exact,
                    quote(&c.unit),
                    quote(&c.source)
                )
            })
            .collect();
        return Ok(format!("[{}]", rows.join(",")));
    }
    Ok(shown.iter().map(|c| c.line()).collect::<Vec<_>>().join("\n"))
}

fn number(s: &str) -> Result<f64, String> {
    Calc::new().value_in(s, "").map_err(|e| format!("{s}: {e}"))
}

fn eseries(args: &[String]) -> Result<String, String> {
    let [name, value] = args else { return Err("usage: eseries <E24> <value>".into()) };
    let v = number(value)?;
    let n = tables::nearest(name, v).ok_or_else(|| format!("series: {}", tables::SERIES.join(", ")))?;
    Ok(format!("{} {} ({:+.2}%)", name.to_uppercase(), crate::engine::fmt_sig(n, 6), (n / v - 1.0) * 100.0))
}

fn divider(args: &[String]) -> Result<String, String> {
    if args.len() < 2 {
        return Err("usage: divider <E24> <Vout/Vin> [min max]".into());
    }
    let ratio = number(&args[1])?;
    let low = args.get(2).map(|s| number(s)).transpose()?.unwrap_or(1e3);
    let high = args.get(3).map(|s| number(s)).transpose()?.unwrap_or(1e6);
    let pairs = tables::divider_pairs(&args[0], ratio, low, high, 5);
    if pairs.is_empty() {
        return Err("ratio must be between 0 and 1 and the series known".into());
    }
    Ok(pairs
        .iter()
        .map(|p| {
            format!(
                "R1 {} R2 {} ratio {:.5} ({:+.3}%)",
                crate::formula::engineering(p.r1, "ohm"),
                crate::formula::engineering(p.r2, "ohm"),
                p.ratio,
                p.error * 100.0
            )
        })
        .collect::<Vec<_>>()
        .join("\n"))
}

fn colour(args: &[String]) -> Result<String, String> {
    let first = args.first().ok_or("usage: colour <ohms> [tolerance%] [bands] or colour <band> <band> ...")?;
    if first.chars().next().is_some_and(|c| c.is_alphabetic()) && tables::COLOURS.iter().any(|c| first.eq_ignore_ascii_case(c)) {
        let names: Vec<String> = args.iter().map(|a| a.to_lowercase()).collect();
        let refs: Vec<&str> = names.iter().map(String::as_str).collect();
        let (ohms, tol) = tables::colour_value(&refs)?;
        return Ok(format!("{} {}%", crate::formula::engineering(ohms, "ohm"), crate::engine::fmt_sig(tol, 3)));
    }
    let ohms = match first.parse::<f64>() {
        Ok(v) => v,
        Err(_) => Calc::new().value_in(first, "ohm").map_err(|e| format!("{first}: {e}"))?,
    };
    let tol = args.get(1).map(|t| t.trim_end_matches('%').parse::<f64>().map_err(|_| "bad tolerance".to_string())).transpose()?.unwrap_or(5.0);
    let bands = args.get(2).map(|b| b.parse::<usize>().map_err(|_| "bad band count".to_string())).transpose()?.unwrap_or(4);
    Ok(tables::colour_bands(ohms, tol, bands)?.join(" "))
}

fn cap(args: &[String]) -> Result<String, String> {
    let first = args.first().ok_or("usage: cap <104|4n7|100nF>")?;
    let plain = first.len() == 3 && first.chars().all(|c| c.is_ascii_digit()) || (first.len() <= 5 && first.chars().any(|c| matches!(c, 'p' | 'n' | 'u')) && !first.to_lowercase().ends_with('f'));
    if plain {
        let f = tables::capacitor_code(first)?;
        return Ok(crate::formula::engineering(f, "F"));
    }
    let f = Calc::new().value_in(&args.join(" "), "F")?;
    tables::capacitor_to_code(f)
}

fn bits_arg(args: &[String], at: usize) -> Result<u32, String> {
    args.get(at).map(|b| b.parse::<u32>().map_err(|_| "bad bit width".to_string())).transpose().map(|b| b.unwrap_or(32))
}

fn base(args: &[String]) -> Result<String, String> {
    let v = tables::parse_int(args.first().ok_or("usage: base <value> [bits]")?)?;
    let bits = bits_arg(args, 1)?;
    if !(1..=127).contains(&bits) {
        return Err("bits must be 1 to 127".into());
    }
    let b = tables::bases(v, bits);
    let twos = tables::twos_complement(v, bits).map(|t| format!("\ntwos complement {t}")).unwrap_or_default();
    Ok(format!("dec {}\nhex {}\noct {}\nbin {}{twos}", b.decimal, b.hex, b.octal, b.binary))
}

fn float(args: &[String]) -> Result<String, String> {
    let text = args.first().ok_or("usage: float <value|0xHEX> [32|64]")?;
    let width = args.get(1).map(|s| s.as_str()).unwrap_or("32");
    let describe = |f: tables::Float| {
        format!("sign {}\nexponent {}\nmantissa {}\nhex {}\nclass {}\nvalue {}", f.sign, f.exponent, f.mantissa, f.hex, f.class, f.exact)
    };
    match (width, text.to_lowercase().starts_with("0x")) {
        ("32", true) => Ok(describe(tables::float32(tables::float32_from_hex(text)?))),
        ("64", true) => Ok(describe(tables::float64(tables::float64_from_hex(text)?))),
        ("32", false) => Ok(describe(tables::float32(number(text)? as f32))),
        ("64", false) => Ok(describe(tables::float64(number(text)?))),
        _ => Err("width is 32 or 64".into()),
    }
}

fn bits(args: &[String]) -> Result<String, String> {
    let op = args.first().ok_or("usage: bits <op> <a> [b] [bits]")?.to_lowercase();
    let a = tables::parse_int(args.get(1).ok_or("missing a")?)?;
    let unary = matches!(op.as_str(), "not" | "~");
    let (b, width_at) = if unary { (0, 2) } else { (tables::parse_int(args.get(2).ok_or("missing b")?)?, 3) };
    let width = bits_arg(args, width_at)?;
    let r = tables::bitwise(&op, a, b, width)?;
    let shown = tables::bases(r, width);
    Ok(format!("{} (hex {}, bin {})", r, shown.hex, shown.binary))
}

fn stats(args: &[String]) -> Result<String, String> {
    let values: Vec<f64> = args.iter().flat_map(|a| a.split(',')).filter(|s| !s.is_empty()).map(number).collect::<Result<_, _>>()?;
    let s = tables::stats(&values)?;
    let f = |v: f64| crate::engine::fmt_sig(v, 10);
    Ok(format!(
        "n {}\nmean {}\nmedian {}\nmin {}\nmax {}\nstdev (sample) {}\nstdev (population) {}\nvariance (sample) {}",
        s.count,
        f(s.mean),
        f(s.median),
        f(s.min),
        f(s.max),
        f(s.stdev_sample),
        f(s.stdev_population),
        f(s.variance_sample)
    ))
}
