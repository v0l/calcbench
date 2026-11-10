use crate::engine::{Calc, fmt_number, fmt_sig};
use crate::{constants, formula, quantities, tables};
use egui::{Align, Layout, Pos2, Rect, Stroke, Ui, Vec2};
use egui_bench::prelude::*;
use std::collections::HashMap;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Tab {
    Convert,
    Constants,
    Formulas,
    Calculator,
    Plot,
    Tools,
}

pub const TABS: &[(Tab, &str)] = &[
    (Tab::Convert, "convert"),
    (Tab::Constants, "constants"),
    (Tab::Formulas, "formulas"),
    (Tab::Calculator, "calculator"),
    (Tab::Plot, "plot"),
    (Tab::Tools, "tools"),
];

const SHOWN: usize = 60;

#[cfg(target_os = "android")]
const TOP_INSET: i8 = 40;
#[cfg(not(target_os = "android"))]
const TOP_INSET: i8 = 8;

pub struct Card {
    pub id: String,
    pub inputs: HashMap<String, String>,
}

pub struct App {
    installed: bool,
    pub tab: Tab,
    calc: Calc,
    pub quantity: usize,
    pub amount: String,
    pub from: usize,
    pub to: usize,
    pub line: String,
    pub line_result: Option<Result<String, String>>,
    pub const_query: String,
    pub const_category: Option<&'static str>,
    pub formula_query: String,
    pub formula_category: Option<&'static str>,
    pub open_card: Option<Card>,
    pub history: Vec<(String, Result<String, String>)>,
    pub expression: String,
    pub plot_expr: String,
    pub plot_lo: String,
    pub plot_hi: String,
    pub plot_x_unit: String,
    pub plot_y_unit: String,
    pub base_input: String,
    pub base_bits: u32,
    pub float_input: String,
    pub bit_a: String,
    pub bit_b: String,
    pub bit_op: &'static str,
    pub series: &'static str,
    pub series_value: String,
    pub divider_ratio: String,
    pub colour_input: String,
    pub cap_input: String,
    pub status: Option<(String, bool)>,
}

impl App {
    pub fn new(ctx: &egui::Context) -> Self {
        install(ctx);
        let mut app = App {
            installed: false,
            tab: Tab::Convert,
            calc: Calc::new(),
            quantity: 0,
            amount: "1".into(),
            from: 0,
            to: 1,
            line: "3 ft 2 in to cm".into(),
            line_result: None,
            const_query: String::new(),
            const_category: None,
            formula_query: String::new(),
            formula_category: None,
            open_card: None,
            history: Vec::new(),
            expression: String::new(),
            plot_expr: "sin(x)/x".into(),
            plot_lo: "-20".into(),
            plot_hi: "20".into(),
            plot_x_unit: "s".into(),
            plot_y_unit: "V".into(),
            base_input: "0xCAFE".into(),
            base_bits: 16,
            float_input: "3.14159".into(),
            bit_a: "0xF0".into(),
            bit_b: "0x3C".into(),
            bit_op: "xor",
            series: "E24",
            series_value: "4800".into(),
            divider_ratio: "0.66".into(),
            colour_input: "470".into(),
            cap_input: "104".into(),
            status: None,
        };
        app.quantity = quantities::QUANTITIES.iter().position(|q| q.name == "length").unwrap_or(0);
        app.to = 1;
        app.line_result = Some(app.calc.eval(&app.line.clone()));
        app
    }

    pub fn opened(ctx: &egui::Context) -> Self {
        let mut app = App::new(ctx);
        if let Ok(tab) = std::env::var("CALCBENCH_TAB") {
            if let Some((t, _)) = TABS.iter().find(|(_, l)| *l == tab) {
                app.tab = *t;
            }
        }
        if let Ok(id) = std::env::var("CALCBENCH_FORMULA") {
            app.open_formula(&id);
        }
        app
    }

    pub fn open_formula(&mut self, id: &str) {
        let Some(f) = formula::get(id) else { return };
        let inputs = f
            .vars
            .iter()
            .enumerate()
            .map(|(i, v)| (v.s.clone(), if i == 0 { String::new() } else { fmt_sig(v.x, 6) }))
            .collect();
        self.open_card = Some(Card { id: id.to_string(), inputs });
        self.tab = Tab::Formulas;
    }

    pub fn evaluate(&mut self) {
        let text = self.expression.trim().to_string();
        if text.is_empty() {
            return;
        }
        let r = self.calc.eval(&text);
        self.history.push((text, r));
        self.expression.clear();
    }

    pub fn show(&mut self, ui: &mut Ui) {
        if !self.installed {
            install(ui.ctx());
            self.installed = true;
        }
        egui::Panel::top("header")
            .frame(egui::Frame::NONE.fill(CHASSIS).inner_margin(egui::Margin { left: 12, right: 12, top: TOP_INSET, bottom: 6 }))
            .show(ui, |ui| {
                ui.label(egui::RichText::new("CALCBENCH").font(legend_font(16.0)).color(VALUE).extra_letter_spacing(2.0));
                ui.add_space(4.0);
                let options: Vec<(Tab, &str, Option<usize>)> = TABS.iter().map(|(t, l)| (*t, *l, None)).collect();
                segments(ui, &mut self.tab, &options);
            });
        if let Some((msg, fault)) = self.status.clone() {
            let tone = if fault { Tone::Fault } else { Tone::Ok };
            if toast(ui.ctx(), "status", &msg, tone) {
                self.status = None;
            }
        }
        egui::CentralPanel::no_frame().show(ui, |ui| {
            let r = ui.max_rect();
            ui.painter().rect_filled(r, 0.0, CHASSIS);
            ui.painter().line_segment([r.left_top(), r.right_top()], Stroke::new(1.0, ETCH));
            egui::Frame::NONE.inner_margin(egui::Margin::same(10)).show(ui, |ui| {
                egui::ScrollArea::vertical().auto_shrink([false, false]).show(ui, |ui| {
                    ui.spacing_mut().item_spacing.y = 8.0;
                    let width = ui.available_width().min(980.0);
                    ui.set_max_width(width);
                    match self.tab {
                        Tab::Convert => self.convert(ui),
                        Tab::Constants => self.constants(ui),
                        Tab::Formulas => self.formulas(ui),
                        Tab::Calculator => self.calculator(ui),
                        Tab::Plot => self.plot(ui),
                        Tab::Tools => self.tools(ui),
                    }
                });
            });
        });
    }

    fn copy(&mut self, ui: &Ui, text: String) {
        ui.ctx().copy_text(text.clone());
        self.status = Some((format!("copied {text}"), false));
    }

    fn convert(&mut self, ui: &mut Ui) {
        let mut copy = None;
        section(ui, "expression", "units, prefixes and arithmetic: 60 mph to m/s", |ui| {
            let mut go = false;
            field_then(ui, &mut self.line, "3 ft 2 in to cm", 100.0, |ui| {
                go = ui.add(Key::new("convert").solid(READOUT).min_width(90.0)).clicked();
            });
            if ui.input(|i| i.key_pressed(egui::Key::Enter)) && ui.memory(|m| m.focused().is_some()) {
                go = true;
            }
            if go {
                self.line_result = Some(self.calc.eval(&self.line.clone()));
            }
            match &self.line_result {
                Some(Ok(r)) => {
                    let r = r.clone();
                    hero_text(ui, &r, TRACE);
                    if ui.add(Key::new("copy").outline(VALUE)).clicked() {
                        copy = Some(r);
                    }
                }
                Some(Err(e)) => status(ui, false, e),
                None => {}
            }
        });
        section(ui, "by quantity", "pick what you are measuring", |ui| {
            let before = self.quantity;
            row(ui, "quantity", |ui| {
                choice(ui, "quantity", &mut self.quantity, quantities::QUANTITIES.iter().enumerate().map(|(i, q)| (i, q.name.to_string())));
            });
            if before != self.quantity {
                self.from = 0;
                self.to = 1.min(quantities::QUANTITIES[self.quantity].units.len() - 1);
            }
            let q = &quantities::QUANTITIES[self.quantity];
            row(ui, "value", |ui| {
                field(ui, &mut self.amount, "1");
            });
            row(ui, "from", |ui| {
                choice(ui, "from", &mut self.from, q.units.iter().enumerate().map(|(i, u)| (i, (*u).to_string())));
            });
            row(ui, "to", |ui| {
                choice(ui, "to", &mut self.to, q.units.iter().enumerate().map(|(i, u)| (i, (*u).to_string())));
            });
            ui.horizontal(|ui| {
                if ui.add(Key::new("swap").outline(VALUE)).clicked() {
                    std::mem::swap(&mut self.from, &mut self.to);
                }
            });
            let from = q.units[self.from.min(q.units.len() - 1)];
            let to = q.units[self.to.min(q.units.len() - 1)];
            match self.calc.eval(&format!("{} {from} to {to}", self.amount)) {
                Ok(r) => {
                    hero_text(ui, &r, TRACE);
                    if ui.add(Key::new("copy").outline(VALUE)).clicked() {
                        copy = Some(r);
                    }
                }
                Err(e) => status(ui, false, &e),
            }
        });
        let q = &quantities::QUANTITIES[self.quantity];
        section(ui, &format!("{} in every unit", q.name), "", |ui| {
            let from = q.units[self.from.min(q.units.len() - 1)];
            for u in q.units {
                let text = self.calc.eval(&format!("{} {from} to {u}", self.amount)).unwrap_or_else(|e| e);
                Line::new().legend(u).column(ui, 90.0).value(text).show(ui);
            }
        });
        if let Some(c) = copy {
            self.copy(ui, c);
        }
    }

    fn constants(&mut self, ui: &mut Ui) {
        section(ui, "find", "name, symbol or a word from it", |ui| {
            field(ui, &mut self.const_query, "electron mass, kB, copper");
            let cats = constants::categories();
            let mut options: Vec<(Option<&'static str>, &str, Option<usize>)> = vec![(None, "all", Some(constants::all().len()))];
            for c in &cats {
                options.push((Some(*c), c, Some(constants::all().iter().filter(|k| k.category == *c).count())));
            }
            segments(ui, &mut self.const_category, &options);
        });
        let hits: Vec<&constants::Constant> = if self.const_query.trim().is_empty() {
            constants::all().iter().collect()
        } else {
            constants::find(&self.const_query)
        };
        let hits: Vec<_> = hits.into_iter().filter(|c| self.const_category.is_none_or(|k| c.category == k)).collect();
        let total = hits.len();
        let mut copy = None;
        for c in hits.into_iter().take(SHOWN) {
            card(
                ui,
                None,
                |ui| {
                    Line::new().legend(&c.name).elided(ui);
                },
                |ui| {
                    ui.horizontal_wrapped(|ui| {
                        ui.label(egui::RichText::new(c.with_unit()).font(figure(15.0)).color(VALUE));
                    });
                    if let Some(s) = &c.symbol {
                        Line::new().legend("symbol").column(ui, 90.0).value(s.clone()).show(ui);
                    }
                    Line::new().legend("uncertainty").column(ui, 90.0).value(c.uncertainty_text()).show(ui);
                    Line::new().legend("source").column(ui, 90.0).note(format!("{}, {}", c.source, c.category)).show(ui);
                    ui.horizontal(|ui| {
                        if ui.add(Key::new("copy value").outline(VALUE)).clicked() {
                            copy = Some(fmt_sig(c.value, 17));
                        }
                        if ui.add(Key::new("copy with unit").outline(VALUE)).clicked() {
                            copy = Some(c.with_unit());
                        }
                    });
                },
            );
        }
        if total > SHOWN {
            hint(ui, &format!("{} more: narrow the search", total - SHOWN));
        }
        if total == 0 {
            note(ui, "no constant matches", LEGEND);
        }
        if let Some(c) = copy {
            self.copy(ui, c);
        }
    }

    fn formulas(&mut self, ui: &mut Ui) {
        section(ui, "find", "name, topic or variable", |ui| {
            field(ui, &mut self.formula_query, "divider, filter, orbit, loan");
            let cats = formula::categories();
            let mut options: Vec<(Option<&'static str>, &str, Option<usize>)> = vec![(None, "all", Some(formula::all().len()))];
            for c in &cats {
                options.push((Some(*c), c, Some(formula::all().iter().filter(|f| f.category == *c).count())));
            }
            segments(ui, &mut self.formula_category, &options);
        });
        let hits: Vec<&formula::Formula> = if self.formula_query.trim().is_empty() {
            formula::all().iter().collect()
        } else {
            formula::search(&self.formula_query)
        };
        let hits: Vec<_> = hits.into_iter().filter(|f| self.formula_category.is_none_or(|k| f.category == k)).collect();
        if hits.is_empty() {
            note(ui, "no formula matches", LEGEND);
        }
        let mut open = None;
        let mut close = false;
        let mut copy = None;
        for f in hits.iter().take(SHOWN) {
            let is_open = self.open_card.as_ref().is_some_and(|c| c.id == f.id);
            card(
                ui,
                if is_open { Some(READOUT) } else { None },
                |ui| {
                    ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                        let label = if is_open { "close" } else { "open" };
                        if ui.add(Key::new(label).outline(VALUE).id_salt(&f.id)).clicked() {
                            if is_open {
                                close = true;
                            } else {
                                open = Some(f.id.clone());
                            }
                        }
                        ui.with_layout(Layout::left_to_right(Align::Center), |ui| {
                            Line::new().legend(&f.name).elided(ui);
                        });
                    });
                },
                |ui| {
                    ui.label(egui::RichText::new(formula::pretty(&f.equation)).font(figure(16.0)).color(VALUE));
                    if is_open {
                        if let Some(c) = copy_card(ui, &mut self.calc, f, self.open_card.as_mut().unwrap()) {
                            copy = Some(c);
                        }
                    } else {
                        Line::new().note(&f.tags).size(10.5).elided(ui);
                    }
                },
            );
        }
        if hits.len() > SHOWN {
            hint(ui, &format!("{} more: narrow the search", hits.len() - SHOWN));
        }
        if close {
            self.open_card = None;
        }
        if let Some(id) = open {
            self.open_formula(&id);
        }
        if let Some(c) = copy {
            self.copy(ui, c);
        }
    }

    fn calculator(&mut self, ui: &mut Ui) {
        section(ui, "calculator", "units, variables (x = 5), functions, complex numbers (1 + 2i)", |ui| {
            let mut go = false;
            let resp = field_then(ui, &mut self.expression, "(1+2i)*(3-i), sqrt(2) m, x = 5", 100.0, |ui| {
                go = ui.add(Key::new("evaluate").solid(READOUT).min_width(90.0)).clicked();
            });
            if resp.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter)) {
                go = true;
                resp.request_focus();
            }
            if go {
                self.evaluate();
            }
            ui.horizontal(|ui| {
                if ui.add(Key::new("clear history").outline(VALUE)).clicked() {
                    self.history.clear();
                    self.calc.clear();
                }
            });
        });
        let mut reuse = None;
        let mut copy = None;
        for (i, (src, res)) in self.history.iter().enumerate().rev() {
            card(
                ui,
                Some(if res.is_ok() { TRACE } else { FAULT }),
                |ui| {
                    Line::new().value(src.clone()).elided(ui);
                },
                |ui| match res {
                    Ok(r) => {
                        ui.label(egui::RichText::new(r).font(figure(16.0)).color(TRACE));
                        ui.horizontal(|ui| {
                            if ui.add(Key::new("copy").outline(VALUE).id_salt(("copy", i))).clicked() {
                                copy = Some(r.clone());
                            }
                            if ui.add(Key::new("reuse").outline(VALUE).id_salt(("reuse", i))).clicked() {
                                reuse = Some(src.clone());
                            }
                        });
                    }
                    Err(e) => status(ui, false, e),
                },
            );
        }
        if let Some(r) = reuse {
            self.expression = r;
        }
        if let Some(c) = copy {
            self.copy(ui, c);
        }
    }

    fn plot(&mut self, ui: &mut Ui) {
        section(ui, "plot y = f(x)", "x is the variable, units label the axes", |ui| {
            row(ui, "y =", |ui| {
                field(ui, &mut self.plot_expr, "sin(x)/x");
            });
            row(ui, "x from", |ui| {
                field(ui, &mut self.plot_lo, "-20");
            });
            row(ui, "x to", |ui| {
                field(ui, &mut self.plot_hi, "20");
            });
            row(ui, "x unit", |ui| {
                field(ui, &mut self.plot_x_unit, "s");
            });
            row(ui, "y unit", |ui| {
                field(ui, &mut self.plot_y_unit, "V");
            });
        });
        let lo = self.calc.value_in(&self.plot_lo, &self.plot_x_unit).or_else(|_| self.calc.value_in(&self.plot_lo, ""));
        let hi = self.calc.value_in(&self.plot_hi, &self.plot_x_unit).or_else(|_| self.calc.value_in(&self.plot_hi, ""));
        let (Ok(lo), Ok(hi)) = (lo, hi) else {
            status(ui, false, "the x range needs numbers, optionally with the x unit");
            return;
        };
        if hi <= lo {
            status(ui, false, "x to must be larger than x from");
            return;
        }
        let parsed = crate::expr::parse(&self.plot_expr, &["x".to_string()]);
        match parsed {
            Err(e) => status(ui, false, &e),
            Ok(e) => {
                let pts = sample(&e, lo, hi, 800);
                draw_plot(ui, &pts, lo, hi, &self.plot_x_unit, &self.plot_y_unit);
            }
        }
    }

    fn tools(&mut self, ui: &mut Ui) {
        section(ui, "number bases", "dec, hex 0x, bin 0b, oct 0o", |ui| {
            row(ui, "value", |ui| {
                field(ui, &mut self.base_input, "255, 0xFF, -5");
            });
            row(ui, "bits", |ui| {
                choice(ui, "bits", &mut self.base_bits, [8u32, 16, 32, 64].into_iter().map(|b| (b, b.to_string())));
            });
            match tables::parse_int(&self.base_input) {
                Ok(v) => {
                    let b = tables::bases(v, self.base_bits);
                    reading(ui, "decimal", b.decimal);
                    reading(ui, "hex", b.hex);
                    reading(ui, "octal", b.octal);
                    reading(ui, "binary", b.binary);
                    if v < 0 {
                        match tables::twos_complement(v, self.base_bits) {
                            Ok(t) => reading(ui, "twos complement", t),
                            Err(e) => status(ui, false, &e),
                        }
                    }
                }
                Err(e) => status(ui, false, &e),
            }
        });
        section(ui, "ieee-754 float", "a number, or 0x bits", |ui| {
            row(ui, "value", |ui| {
                field(ui, &mut self.float_input, "3.14159 or 0x40490FDB");
            });
            let text = self.float_input.trim().to_string();
            let f32v = if text.to_lowercase().starts_with("0x") { tables::float32_from_hex(&text) } else { self.calc.value_in(&text, "").map(|v| v as f32) };
            match f32v {
                Ok(v) => {
                    let f = tables::float32(v);
                    reading(ui, "sign", f.sign.to_string());
                    reading(ui, "exponent", f.exponent);
                    reading(ui, "mantissa", f.mantissa);
                    reading(ui, "hex", f.hex);
                    reading(ui, "class", f.class.to_string());
                    reading(ui, "f32 value", f.exact);
                    let d = tables::float64(v as f64);
                    reading(ui, "f64 hex", d.hex);
                }
                Err(e) => status(ui, false, &e),
            }
        });
        section(ui, "bitwise", "and, or, xor, nand, nor, shl, shr, rol, ror", |ui| {
            row(ui, "a", |ui| {
                field(ui, &mut self.bit_a, "0xF0");
            });
            row(ui, "b", |ui| {
                field(ui, &mut self.bit_b, "0x3C");
            });
            row(ui, "operation", |ui| {
                choice(ui, "bitop", &mut self.bit_op, ["and", "or", "xor", "nand", "nor", "shl", "shr", "rol", "ror"].into_iter().map(|o| (o, o.to_string())));
            });
            let r = tables::parse_int(&self.bit_a).and_then(|a| tables::parse_int(&self.bit_b).and_then(|b| tables::bitwise(self.bit_op, a, b, 8)));
            match r {
                Ok(v) => {
                    let b = tables::bases(v, 8);
                    reading(ui, "result", format!("{v}  hex {}  bin {}", b.hex, b.binary));
                }
                Err(e) => status(ui, false, &e),
            }
        });
        section(ui, "e-series", "nearest standard value and divider pairs", |ui| {
            row(ui, "series", |ui| {
                choice(ui, "series", &mut self.series, tables::SERIES.iter().map(|s| (*s, (*s).to_string())));
            });
            row(ui, "value", |ui| {
                field(ui, &mut self.series_value, "4800");
            });
            match self.calc.value_in(&self.series_value, "").ok().and_then(|v| tables::nearest(self.series, v).map(|n| (v, n))) {
                Some((v, n)) => reading(ui, "nearest", format!("{} ({:+.2}%)", formula::engineering(n, "ohm"), (n / v - 1.0) * 100.0)),
                None => status(ui, false, "enter a positive number"),
            }
            row(ui, "divider ratio", |ui| {
                field(ui, &mut self.divider_ratio, "0.66");
            });
            match self.calc.value_in(&self.divider_ratio, "") {
                Ok(r) => {
                    for p in tables::divider_pairs(self.series, r, 1e3, 1e5, 3) {
                        reading(
                            ui,
                            "r1 / r2",
                            format!("{} / {}  {:.4} ({:+.2}%)", formula::engineering(p.r1, "ohm"), formula::engineering(p.r2, "ohm"), p.ratio, p.error * 100.0),
                        );
                    }
                }
                Err(e) => status(ui, false, &e),
            }
        });
        section(ui, "resistor colour code", "ohms to bands", |ui| {
            row(ui, "resistance", |ui| {
                field(ui, &mut self.colour_input, "470 or 4.7 kohm");
            });
            let ohms = self.colour_input.trim().parse::<f64>().or_else(|_| self.calc.value_in(&self.colour_input, "ohm"));
            match ohms.map_err(|e| e).and_then(|o| tables::colour_bands(o, 5.0, 4)) {
                Ok(b) => {
                    ui.horizontal_wrapped(|ui| {
                        for name in b {
                            chip(ui, name, band_tint(name), true);
                        }
                    });
                }
                Err(e) => status(ui, false, &e),
            }
        });
        section(ui, "capacitor code", "104, 472, 4n7", |ui| {
            row(ui, "code", |ui| {
                field(ui, &mut self.cap_input, "104");
            });
            match tables::capacitor_code(&self.cap_input) {
                Ok(f) => reading(ui, "capacitance", formula::engineering(f, "F")),
                Err(e) => status(ui, false, &e),
            }
        });
    }
}

fn band_tint(name: &str) -> egui::Color32 {
    match name {
        "brown" | "gold" => WARN,
        "red" => FAULT,
        "green" => OK,
        "blue" => TRACE,
        "orange" | "yellow" => READOUT,
        _ => VALUE,
    }
}

fn hero_text(ui: &mut Ui, text: &str, tint: egui::Color32) {
    let (num, unit) = text.split_once(' ').unwrap_or((text, ""));
    hero(ui, "result", num, unit, tint);
}

fn copy_card(ui: &mut Ui, calc: &mut Calc, f: &formula::Formula, card: &mut Card) -> Option<String> {
    let mut copy = None;
    if !f.notes.is_empty() {
        hint(ui, &f.notes);
    }
    for v in &f.vars {
        let hint_text = if v.u.is_empty() { v.n.clone() } else { format!("{} [{}]", v.n, v.u) };
        let text = card.inputs.entry(v.s.clone()).or_default();
        row(ui, &v.s, |ui| {
            field(ui, text, &hint_text);
        });
        Line::new().note(hint_text).size(10.5).show(ui);
    }
    let given: Vec<(String, String)> = card.inputs.iter().filter(|(_, t)| !t.trim().is_empty()).map(|(k, t)| (k.clone(), t.clone())).collect();
    match formula::solve_with_units(calc, f, &given) {
        Ok(s) => {
            let text = s.engineering();
            let (n, u) = text.split_once(' ').unwrap_or((&text, ""));
            hero(ui, &format!("{} ({})", s.symbol, s.name), n, u, TRACE);
            if ui.add(Key::new("copy result").outline(VALUE)).clicked() {
                copy = Some(text);
            }
        }
        Err(e) => status(ui, false, &e),
    }
    Line::new().legend("source").column(ui, 90.0).note(f.source.clone()).show(ui);
    copy
}

pub fn sample(e: &crate::expr::Expr, lo: f64, hi: f64, n: usize) -> Vec<(f64, f64)> {
    (0..=n)
        .map(|i| {
            let x = lo + (hi - lo) * i as f64 / n as f64;
            (x, e.eval(&[x]))
        })
        .collect()
}

fn nice_step(span: f64) -> f64 {
    let raw = span / 6.0;
    let mag = 10f64.powf(raw.log10().floor());
    let r = raw / mag;
    mag * if r < 1.5 { 1.0 } else if r < 3.5 { 2.0 } else if r < 7.5 { 5.0 } else { 10.0 }
}

pub fn draw_plot(ui: &mut Ui, pts: &[(f64, f64)], lo: f64, hi: f64, x_unit: &str, y_unit: &str) {
    let finite: Vec<f64> = pts.iter().map(|p| p.1).filter(|y| y.is_finite()).collect();
    if finite.is_empty() {
        status(ui, false, "the expression gives no finite values in that range");
        return;
    }
    let mut ylo = finite.iter().cloned().fold(f64::INFINITY, f64::min);
    let mut yhi = finite.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
    if yhi - ylo < 1e-12 {
        ylo -= 1.0;
        yhi += 1.0;
    }
    let pad = (yhi - ylo) * 0.08;
    ylo -= pad;
    yhi += pad;
    let height = (ui.available_width() * 0.55).clamp(220.0, 420.0);
    let (rect, _) = ui.allocate_exact_size(Vec2::new(ui.available_width(), height), egui::Sense::hover());
    let p = ui.painter_at(rect);
    p.rect_filled(rect, 2.0, WELL);
    let plot = Rect::from_min_max(Pos2::new(rect.left() + 64.0, rect.top() + 14.0), Pos2::new(rect.right() - 14.0, rect.bottom() - 38.0));
    let px = |x: f64| plot.left() + ((x - lo) / (hi - lo)) as f32 * plot.width();
    let py = |y: f64| plot.bottom() - ((y - ylo) / (yhi - ylo)) as f32 * plot.height();
    let grid = Stroke::new(1.0, ETCH);
    let step = nice_step(hi - lo);
    let mut x = (lo / step).ceil() * step;
    while x <= hi + step * 1e-9 {
        let at = px(x);
        p.line_segment([Pos2::new(at, plot.top()), Pos2::new(at, plot.bottom())], grid);
        p.text(Pos2::new(at, plot.bottom() + 4.0), egui::Align2::CENTER_TOP, fmt_number(round_tick(x, step)), figure(11.0), LEGEND);
        x += step;
    }
    let ystep = nice_step(yhi - ylo);
    let mut y = (ylo / ystep).ceil() * ystep;
    while y <= yhi + ystep * 1e-9 {
        let at = py(y);
        p.line_segment([Pos2::new(plot.left(), at), Pos2::new(plot.right(), at)], grid);
        p.text(Pos2::new(plot.left() - 6.0, at), egui::Align2::RIGHT_CENTER, fmt_number(round_tick(y, ystep)), figure(11.0), LEGEND);
        y += ystep;
    }
    if lo < 0.0 && hi > 0.0 {
        p.line_segment([Pos2::new(px(0.0), plot.top()), Pos2::new(px(0.0), plot.bottom())], Stroke::new(1.0, LEGEND));
    }
    if ylo < 0.0 && yhi > 0.0 {
        p.line_segment([Pos2::new(plot.left(), py(0.0)), Pos2::new(plot.right(), py(0.0))], Stroke::new(1.0, LEGEND));
    }
    let mut run: Vec<Pos2> = Vec::new();
    let trace = Stroke::new(1.8, READOUT);
    for (x, y) in pts {
        if y.is_finite() {
            run.push(Pos2::new(px(*x), py(*y).clamp(plot.top(), plot.bottom())));
        } else if run.len() > 1 {
            p.add(egui::Shape::line(std::mem::take(&mut run), trace));
        } else {
            run.clear();
        }
    }
    if run.len() > 1 {
        p.add(egui::Shape::line(run, trace));
    }
    p.rect_stroke(plot, 0.0, Stroke::new(1.0, ETCH), egui::StrokeKind::Inside);
    let xl = if x_unit.is_empty() { "x".to_string() } else { format!("x [{x_unit}]") };
    let yl = if y_unit.is_empty() { "y".to_string() } else { format!("y [{y_unit}]") };
    p.text(Pos2::new(plot.center().x, rect.bottom() - 4.0), egui::Align2::CENTER_BOTTOM, xl, legend_font(11.0), LEGEND);
    p.text(Pos2::new(rect.left() + 4.0, rect.top() + 2.0), egui::Align2::LEFT_TOP, yl, legend_font(11.0), LEGEND);
}

fn round_tick(v: f64, step: f64) -> f64 {
    let d = (-step.log10().floor()).max(0.0) as i32 + 1;
    let m = 10f64.powi(d);
    (v * m).round() / m
}

#[cfg(not(target_os = "android"))]
pub fn run() -> eframe::Result<()> {
    let viewport = egui::ViewportBuilder::default()
        .with_inner_size([1000.0, 800.0])
        .with_min_inner_size([360.0, 480.0])
        .with_title("calcbench")
        .with_app_id("calcbench");
    eframe::run_native("calcbench", eframe::NativeOptions { viewport, ..Default::default() }, Box::new(|cc| Ok(Box::new(Shell(App::opened(&cc.egui_ctx))))))
}

pub struct Shell(pub App);

impl eframe::App for Shell {
    fn ui(&mut self, ui: &mut Ui, _frame: &mut eframe::Frame) {
        self.0.show(ui);
    }
}

#[cfg(target_os = "android")]
#[unsafe(no_mangle)]
fn android_main(app: android_activity::AndroidApp) {
    android_logger::init_once(android_logger::Config::default().with_max_level(log::LevelFilter::Info));
    let opts = eframe::NativeOptions { android_app: Some(app), renderer: eframe::Renderer::Glow, ..Default::default() };
    let _ = eframe::run_native("calcbench", opts, Box::new(|cc| Ok(Box::new(Shell(App::new(&cc.egui_ctx))))));
}
