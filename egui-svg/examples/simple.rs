#![expect(missing_docs)] // it's an example
use eframe::{App, NativeOptions};
use egui::{CentralPanel, Pos2, Sense, TextStyle, TextWrapMode, Ui, WidgetText, epaint::TextShape};
use egui_svg::EguiSvg;

fn draw(ui: &mut Ui) {
    CentralPanel::default().show(ui, |ui| {
        ui.heading("egui-svg");
        ui.separator();
        ui.label("This is the simple example for egui-svg.");
        ui.separator();

        let galley = WidgetText::Text("Woah...".into()).into_galley(
            ui,
            Some(TextWrapMode::Extend),
            f32::INFINITY,
            TextStyle::Body,
        );
        let (response, painter) = ui.allocate_painter(ui.available_size(), Sense::hover());

        let text_pos = {
            let pos = response.rect.left_center();
            Pos2 {
                x: pos.x,
                y: pos.y + galley.size().x * 0.5,
            }
        };
        let angle = -std::f32::consts::FRAC_PI_2;
        painter.add(TextShape::new(text_pos, galley, ui.visuals().text_color()).with_angle(angle));
    });
}

struct MyEguiApp;
impl MyEguiApp {
    fn new() -> Self {
        Self
    }
}

impl App for MyEguiApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        draw(ui)
    }
}

fn main() -> Result<(), eframe::Error> {
    let res_x = 300;
    let res_y = 100;
    let svg = EguiSvg::new(res_x, res_y).run(draw);

    if option_env!("DRY_RUN").is_none() {
        std::fs::write("egui-svg/assets/egui.svg", svg).unwrap();
    } else {
        println!("{svg}");
    }

    eframe::run_native(
        "simple",
        NativeOptions {
            viewport: egui::ViewportBuilder::default()
                .with_inner_size([res_x as f32, res_y as f32]),
            ..Default::default()
        },
        Box::new(|_cc| Ok(Box::new(MyEguiApp::new()))),
    )
}
