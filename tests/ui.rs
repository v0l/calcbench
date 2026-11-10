use calcbench::app::{App, Tab};
use egui_kittest::Harness;
use egui_kittest::kittest::{NodeT, Queryable};

struct Screen {
    ctx: egui::Context,
    app: App,
    width: f32,
    texts: Vec<String>,
    right: f32,
    widest: String,
}

impl Screen {
    fn new(tab: Tab, width: f32) -> Screen {
        let ctx = egui::Context::default();
        let mut app = App::new(&ctx);
        app.tab = tab;
        let mut s = Screen { ctx, app, width, texts: Vec::new(), right: 0.0, widest: String::new() };
        s.settle();
        s
    }

    fn settle(&mut self) {
        let screen = egui::Rect::from_min_size(egui::Pos2::ZERO, egui::vec2(self.width, 4000.0));
        for _ in 0..3 {
            let raw = egui::RawInput { screen_rect: Some(screen), ..Default::default() };
            let app = &mut self.app;
            let mut out = self.ctx.run_ui(raw, |ui| app.show(ui));
            out.textures_delta.clear();
            self.texts.clear();
            self.right = 0.0;
            for c in &out.shapes {
                if let egui::Shape::Text(t) = &c.shape {
                    self.texts.push(t.galley.text().to_string());
                    let r = t.galley.rect.translate(t.pos.to_vec2()).right();
                    if r > self.right {
                        self.right = r;
                        self.widest = t.galley.text().to_string();
                    }
                }
            }
        }
    }

    fn text(&self) -> String {
        self.texts.join("\n")
    }
}

#[test]
fn converter_shows_the_expression_result() {
    let s = Screen::new(Tab::Convert, 900.0);
    assert!(s.text().contains("96.52"), "{}", s.text());
}

#[test]
fn clicking_convert_evaluates_the_typed_expression() {
    let mut h = Harness::builder()
        .with_size(egui::vec2(900.0, 760.0))
        .build_ui_state(|ui, app: &mut App| app.show(ui), App::new(&egui::Context::default()));
    h.run();
    h.state_mut().line = "60 mph to m/s".into();
    h.get_by_label("convert").click();
    h.run();
    assert_eq!(h.state().line_result, Some(Ok("26.8224 m / s".to_string())));
    h.state_mut().line = "1 m + 1 s".into();
    h.get_by_label("convert").click();
    h.run();
    assert!(h.state().line_result.as_ref().unwrap().as_ref().unwrap_err().contains("incompatible"));
}

#[test]
fn bad_dimensions_show_an_error_on_screen() {
    let mut s = Screen::new(Tab::Convert, 900.0);
    s.app.line = "1 m + 1 s".into();
    s.app.line_result = Some(Err("cannot convert from s to m: units 'second' and 'meter' are incompatible".into()));
    s.settle();
    assert!(s.text().contains("incompatible"), "{}", s.text());
}

#[test]
fn quantity_picker_converts_between_chosen_units() {
    let mut s = Screen::new(Tab::Convert, 900.0);
    s.app.amount = "2.54".into();
    s.settle();
    assert!(s.text().contains("2540"), "{}", s.text());
}

#[test]
fn constants_search_lists_matching_cards() {
    let mut s = Screen::new(Tab::Constants, 900.0);
    s.app.const_query = "electron mass".into();
    s.settle();
    let t = s.text();
    assert!(t.contains("electron mass") && t.contains("CODATA 2022") && t.contains("9.1093837139e-31 kg"), "{t}");
    assert!(t.contains("copy value") || t.contains("COPY VALUE"), "{t}");
}

#[test]
fn formula_card_solves_for_the_empty_variable() {
    let mut s = Screen::new(Tab::Formulas, 900.0);
    s.app.open_formula("ohm");
    let card = s.app.open_card.as_mut().unwrap();
    card.inputs.insert("V".into(), "12 V".into());
    card.inputs.insert("I".into(), String::new());
    card.inputs.insert("R".into(), "470 ohm".into());
    s.settle();
    let t = s.text();
    assert!(t.contains("25.53191489"), "{t}");
    assert!(t.contains("V = I·R"), "{t}");
}

#[test]
fn formula_card_reports_units_that_do_not_fit() {
    let mut s = Screen::new(Tab::Formulas, 900.0);
    s.app.open_formula("ohm");
    let card = s.app.open_card.as_mut().unwrap();
    card.inputs.insert("V".into(), "12 m".into());
    card.inputs.insert("I".into(), String::new());
    card.inputs.insert("R".into(), "470 ohm".into());
    s.settle();
    assert!(s.text().contains("incompatible"), "{}", s.text());
}

#[test]
fn calculator_keeps_history_and_variables() {
    let mut s = Screen::new(Tab::Calculator, 900.0);
    for line in ["x = 5", "x * 2 m", "(1+2i)*(3-i)", "1 m + 1 s"] {
        s.app.expression = line.into();
        s.app.evaluate();
    }
    s.settle();
    let hist = &s.app.history;
    assert_eq!(hist[1].1, Ok("10 m".to_string()));
    assert_eq!(hist[2].1, Ok("5 + 5i".to_string()));
    assert!(hist[3].1.is_err());
    let t = s.text();
    assert!(t.contains("5 + 5i") && t.contains("10 m"), "{t}");
}

#[test]
fn plot_draws_with_units_on_the_axes() {
    let s = Screen::new(Tab::Plot, 900.0);
    let t = s.text();
    assert!(t.contains("x [s]") && t.contains("y [V]"), "{t}");
}

#[test]
fn plot_samples_the_expression() {
    let e = calcbench::expr::parse("x^2", &["x".to_string()]).unwrap();
    let pts = calcbench::app::sample(&e, -2.0, 2.0, 4);
    assert_eq!(pts, vec![(-2.0, 4.0), (-1.0, 1.0), (0.0, 0.0), (1.0, 1.0), (2.0, 4.0)]);
}

#[test]
fn tools_tab_shows_bases_and_codes() {
    let s = Screen::new(Tab::Tools, 900.0);
    let t = s.text();
    assert!(t.contains("CAFE") && t.contains("1100 1010 1111 1110"), "{t}");
    assert!(t.contains("100 nF"), "{t}");
    assert!(t.to_lowercase().contains("yellow"), "{t}");
}

#[test]
fn narrow_phone_width_has_no_horizontal_overflow() {
    for tab in [Tab::Convert, Tab::Constants, Tab::Formulas, Tab::Calculator, Tab::Plot, Tab::Tools] {
        let s = Screen::new(tab, 360.0);
        assert!(s.right <= 361.0, "{tab:?} reaches {} with {:?}", s.right, s.widest);
    }
}
