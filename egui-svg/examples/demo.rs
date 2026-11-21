#![expect(missing_docs)] // it's an example
use egui_svg::EguiSvg;

fn main() {
    let svg = EguiSvg::new(1000, 1000).run(|ui| {
        egui_demo_lib::DemoWindows::default().ui(ui);
    });

    if option_env!("DRY_RUN").is_none() {
        std::fs::write("egui-svg/assets/egui.svg", svg).unwrap();
    } else {
        println!("{svg}");
    }
}
