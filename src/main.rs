use Rusty_egui::egui::{ Context, Frame};
use Rusty_egui::eframe;
use Rusty_egui::egui;
use Rusty_egui::eframe::{App, CreationContext};
use Rusty_egui::{replace_fonts};
use Rusty_egui::fix_vul_lib;
mod ui_styles;
mod app;
use app::MyApp;




fn main() -> Result<(), Rusty_egui::eframe::Error> {
    Rusty_egui::fix_vul_lib();
    // 로거 설정 (선택 사항)
    env_logger::init();

    // 네이티브 옵션 설정
    let options = eframe::NativeOptions {
        viewport : egui::ViewportBuilder::default(),
        
        vsync :true,
        centered: true,
        ..Default::default()
    };

    // 애플리케이션 실행
    eframe::run_native(
        "test",
        options,
        Box::new(|cc| Ok(Box::new(app::MyApp::new(cc)))),
    )
}

