#![expect(missing_docs)] // it's an example
use egui::CentralPanel;
use egui_svg::EguiSvg;

fn main() {
    let svg = EguiSvg::new(300, 100).run(|ui| {
        CentralPanel::default().show(ui, |ui| {
            ui.heading("egui-svg");
            ui.label("This is the simple example for egui-svg.");
        });
    });

    if option_env!("DRY_RUN").is_none() {
        std::fs::write("egui-svg/assets/egui.svg", svg).unwrap();
    } else {
        println!("{svg}");
    }
}
