use Rusty_egui::egui::{ Context, Frame};
use Rusty_egui::eframe;
use Rusty_egui::egui;
use Rusty_egui::eframe::{App, CreationContext};
use Rusty_egui::{replace_fonts};
use Rusty_egui::fix_vul_lib;
mod ui_styles;
use ui_styles::UiStyle;
use ui_styles::{ContextStyle, WidgetStyle};





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
        "내 첫 번째 egui 앱",
        options,
        Box::new(|cc| Ok(Box::new(MyApp::new(cc)))),
    )
}

// 애플리케이션 상태를 저장할 구조체
struct MyApp {
    name: String,
}

impl MyApp {
    // 생성자
    fn new(_cc: &CreationContext) -> Self {
        Rusty_egui::replace_fonts(&_cc.egui_ctx);
        Self {
            name: "World".to_string(),
        }
    }
}

// 애플리케이션 업데이트 구현
impl App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // 중앙 패널 생성
        let _ = &ctx.apply_style(&UiStyle::deep_navy(1));
        egui::CentralPanel::default().apply_style(&UiStyle::dark_blue()).show(ctx, |ui| {
            ui.heading("Hello egui!");
            ui.horizontal(|ui| {
                ui.label("이름: ");
                ui.text_edit_singleline(&mut self.name);
            });
            ui.label(format!("안녕하세요, {}님!", self.name));
            
            if ui.button("Click me!").clicked() {
                println!("버튼이 클릭되었습니다!");
            }
        });
    }
}