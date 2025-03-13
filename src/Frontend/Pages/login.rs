use crate::Frontend::app::{Page, PageState};
use Rusty_egui::egui::UiBuilder;
use Rusty_egui::egui;
use Rusty_egui::eframe;
use crate::Frontend::Utility::ui_styles::{ContextStyle, WidgetStyle, UiStyle};


pub struct MakeAccountPage{
    _name: String,
    id_field: String,  // 이렇게 필드 추가
    password_field: String,
    password_check_field: String,
    name_field: String,
    reason_field: String,
    job_field: String,
    email_field: String,
    phone_field: String,
    region_field: String,
    religion_field : String,
}
impl MakeAccountPage {
    pub fn new(name: &str) -> Self {
        Self {
            _name: name.to_string(),
            id_field: String::new(),
            password_field: String::new(),
            password_check_field: String::new(),
            name_field: String::new(),
            reason_field: String::new(),
            job_field: String::new(),
            email_field: String::new(),
            phone_field: String::new(),
            region_field: String::new(),
            religion_field: String::new(),
        }
    }
}
impl Page for MakeAccountPage {
    fn run(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) -> PageState {
        let mut return_v = PageState::MAKEACCOUNT;
        let _ = &ctx.apply_style(&UiStyle::deep_navy(2));

        egui::CentralPanel::default()
            .frame(egui::Frame {
                outer_margin: Rusty_egui::egui::Margin::same(0.0),
                inner_margin: Rusty_egui::egui::Margin::same(0.0),
            ..Default::default()
            })
            .apply_style(&UiStyle::dark_blue()) // 필요시
            .show(ctx, |ui| {
                // 전체 패널의 위치/크기
                let full_rect = ui.max_rect();
                let width = full_rect.width();
                let height = full_rect.height();

                // ─────────────────────────────────────────────────────────
                // 1) 상단 10% rect
                let top_h = height * 0.20; // 10%
                let top_rect = egui::Rect::from_min_size(
                    full_rect.min,
                    egui::vec2(width, top_h),
                );

                // 2) 하단 10% rect
                let bottom_h = height * 0.20; // 10%
                let bottom_rect = egui::Rect::from_min_size(
                    // y좌표는 전체 하단에서 bottom_h만큼 뺀 위치
                    egui::pos2(full_rect.min.x, full_rect.max.y - bottom_h),
                    egui::vec2(width, bottom_h),
                );
                let small_rect = egui::Rect::from_min_max(
                    egui::pos2(full_rect.min.x,  full_rect.min.y + top_h ),
                    egui::pos2( full_rect.min.x + width * 0.2, full_rect.max.y - bottom_h),
                );
                // 3) 중간 80% rect (상단/하단 사이)
                //    좌우로도 15%씩 제외하여 가운데 70%만 사용
                let middle_rect = egui::Rect::from_min_max(
                    egui::pos2(
                        full_rect.min.x + width * 0.2,         // 왼쪽 15%
                        full_rect.min.y + top_h                 // 위에서 10% 내려온 지점
                    ),
                    egui::pos2(
                        full_rect.max.x ,         
                        full_rect.max.y - bottom_h              // 아래에서 10% 위 지점
                    ),
                );

                // ─────────────────────────────────────────────────────────
                // (A) 상단 10%: 제목/설명
                ui.allocate_new_ui(UiBuilder::new().max_rect(top_rect), |ui| {
                    ui.add_space(20.0);
                    ui.vertical_centered(|ui| {
                        ui.heading("Sign Up");
                        ui.add_space(10.0);
                        ui.label("Create an account to manage your servers efficiently");
                    });
                });

                // (B) 하단 10%: 버튼
                ui.allocate_new_ui(UiBuilder::new().max_rect(bottom_rect), |ui| {
                    ui.vertical_centered(|ui| {
                        if ui
                            .add(egui::Button::new("Sign Up!!").apply_style(&UiStyle::bright_blue()).min_size(egui::vec2(500.0, 30.0)))
                            .clicked()
                        {
                            // 가입 버튼 클릭 시 처리
                            // ex) return_v = PageState::LOGIN;
                            return_v = PageState::LOGIN;
                        }
                    });
                });
                ui.allocate_new_ui(UiBuilder::new().max_rect(small_rect), |ui| {
                    ui.add_space(30.0);
                    egui::Grid::new("small_grid")
                    .num_columns(2)
                    .spacing([10.0, 30.0])
                    .show(ui, |ui| {


                        ui.label("");
                        ui.label("ID");
                        ui.end_row();
                        ui.label("");
                        ui.label("password");
                        ui.end_row();
                        ui.label("");
                        ui.label("이름");
                        ui.end_row();
                        ui.label("");
                        ui.label("직업");
                        ui.end_row();
                        ui.label("");
                        ui.label("거주지");

                    });
                });
                // (C) 중간 80%: 좌우 15% 빼고 남은 영역에서
                ui.allocate_new_ui(UiBuilder::new().max_rect(middle_rect), |ui| {
                    // 1) 첫 줄: 라벨 5개
                    ui.horizontal(|ui| {

                    });

                    ui.add_space(20.0);

                    // 2) 3열 Grid
                    egui::Grid::new("signup_grid")
                        .num_columns(3)
                        .min_col_width(210.0)
                        .spacing([10.0, 30.0])
                        .show(ui, |ui| {
                            // 1행
                            ui.add(
                                egui::TextEdit::singleline(&mut self.id_field)
                                    .hint_text("ID를 입력하세요"),
                            );
                            ui.label("가입 사유");
                            ui.add(
                                egui::TextEdit::singleline(&mut self.reason_field)
                                    .hint_text("가입 사유?"),
                            );
                            ui.end_row();

                            // 2행 (password)
                            let password_input = egui::TextEdit::singleline(&mut self.password_field)
                                .hint_text("Enter your password")
                                .password(true)
                                .desired_width(300.0);
                            ui.add(password_input);

                            ui.label("retype password");
                            let password_check_input =
                                egui::TextEdit::singleline(&mut self.password_check_field)
                                    .hint_text("Retype password")
                                    .password(true)
                                    .desired_width(300.0);
                            ui.add(password_check_input);
                            ui.end_row();

                            // 3행 (이름)
                            ui.add(
                                egui::TextEdit::singleline(&mut self.name_field)
                                    .hint_text("실명을 입력하세요"),
                            );
                            ui.label("이메일");
                            ui.add(
                                egui::TextEdit::singleline(&mut self.email_field)
                                    .hint_text("이메일 주소"),
                            );
                            ui.end_row();

                            // 4행 (직업)
                            ui.add(
                                egui::TextEdit::singleline(&mut self.job_field)
                                    .hint_text("직업을 입력하세요"),
                            );
                            ui.label("전화번호");
                            ui.add(
                                egui::TextEdit::singleline(&mut self.phone_field)
                                    .hint_text("전화번호"),
                            );
                            ui.end_row();

                            // 5행 (거주지)
                            ui.add(
                                egui::TextEdit::singleline(&mut self.region_field)
                                    .hint_text("지역을 입력하세요"),
                            );
                            ui.label("종교");
                            ui.add(
                                egui::TextEdit::singleline(&mut self.religion_field)
                                    .hint_text("종교를 입력하세요"),
                            );
                            ui.end_row();
                        });
                });
            });

        return_v
    }
    
    
    fn _name(&self) -> &str {
        &self._name
    }
    fn _initialize(&mut self) {
        println!("{} 페이지에 진입했습니다.", self._name);
    }
    fn _cleanup(&mut self) {
        println!("{} 페이지를 나갑니다.", self._name);
    }   


}





pub struct LoginPage{
    _name: String,
    _id_field: String,  // 이렇게 필드 추가
    password_field: String,

}
impl LoginPage {
    pub fn new(name: &str) -> Self {
        Self {
            _name: name.to_string(),
            _id_field: String::new(),
            password_field: String::new(),
           
        }
    }
}

impl Page  for LoginPage {

    fn run(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame)->PageState {
        let _ = &ctx.apply_style(&UiStyle::deep_navy(2));
        let mut returnV=PageState::LOGIN;
        egui::CentralPanel::default().apply_style(&UiStyle::dark_blue()).show(ctx, |ui| {
            ui.vertical_centered(|ui| {
            ui.add_space(20.0);
            ui.heading(egui::RichText::new("Access Account").size(32.0));
            ui.label(
                egui::RichText::new("Log in to manage your servers efficiently")
                    .size(16.0)
                    .color(egui::Color32::from_rgba_unmultiplied(180, 180, 180, 200))
            );
            ui.add_space(30.0);
            ui.add_space(20.0);
            let ID_input = egui::TextEdit::singleline(&mut self._id_field)
                .hint_text("Your ID") // 흐릿한 placeholder 텍스트
                .desired_width(300.0);
            ui.add(ID_input);
            ui.add_space(20.0);
            let password_input = egui::TextEdit::singleline(&mut self.password_field)
                .hint_text("Enter your password")
                .password(true) // 비밀번호 마스킹 처리
                .desired_width(300.0);
            ui.add(password_input);
            ui.add_space(20.0);
            if ui.add(egui::Button::new("Log In").apply_style(&UiStyle::bright_blue()).min_size(egui::vec2(300.0, 40.0))).clicked() {
                // 로그인 버튼 클릭 시 실행할 코드
                returnV=PageState::MAIN;
                
            }
            ui.add_space(5.0);
        // 컨테이너 중앙 정렬 시도
        egui::Frame::none()
            .inner_margin(egui::vec2(0.0, 5.0))
            .show(ui, |ui| {
                ui.with_layout(egui::Layout::top_down(egui::Align::Center), |ui| {
                    ui.horizontal(|ui| {
                        ui.add_space(20.0);
                        ui.label("Need to create an account?");
                        let signup_text = egui::RichText::new("Sign Up")
                        .color(egui::Color32::from_rgb(100, 100, 255))
                        .underline();
                    
                    if ui.button(signup_text).clicked() {
                        returnV=PageState::MAKEACCOUNT;
                       // println!("Sign Up 클릭됨!");
                    }
                    });
                });
            });
        });
    }
    );
    returnV
}    
    
    
    fn _name(&self) -> &str {
        &self._name
    }
    fn _initialize(&mut self) {
        println!("{} 페이지에 진입했습니다.", self._name);
    }
    fn _cleanup(&mut self) {
        println!("{} 페이지를 나갑니다.", self._name);
    }
    
}
