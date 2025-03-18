use Rusty_egui::eframe;
use Rusty_egui::egui;
use Rusty_egui::eframe::{App, CreationContext};
use crate::Frontend::Pages::login::{LoginPage,MakeAccountPage};
use crate::Frontend::Pages::dashboard::MainPage;
pub enum PageState {
    LOGIN,
    MAIN,
    FILE,
    MONITER,
    MAKEACCOUNT,
}

pub trait Page{
    fn run(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame)->PageState;
    // 추가로 유용한 메서드들:
    fn _name(&self) -> &str;  // 페이지 이름 반환
    fn _initialize(&mut self);  // 페이지 진입 시 초기화
    fn _cleanup(&mut self);  // 페이지 나갈 때 정리
}


pub struct MyApp {
    Login :Box<dyn Page>,
    Main :Box<dyn Page>,
    File :Box<dyn Page>,
    Moniter :Box<dyn Page>,
    MakeAccountPage : Box<dyn Page>,
    State : PageState,

    

}

impl MyApp  {
    pub fn new(_cc: &CreationContext) -> Self {
        Rusty_egui::replace_fonts(&_cc.egui_ctx);
        Self {
            Login: Box::new(LoginPage::new("Login")),
            Main: Box::new(MainPage::new(&_cc.egui_ctx,"Main")),
            File: Box::new(LoginPage::new("File")),
            Moniter: Box::new(LoginPage::new("Moniter")),
            MakeAccountPage: Box::new(MakeAccountPage::new("MakeAccount")),
            State: PageState::LOGIN,
        }
    }
}


//run(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame)
// 애플리케이션 업데이트 구현
impl App for MyApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {


        match self.State {
            PageState::LOGIN => {
                ctx.send_viewport_cmd(egui::ViewportCommand::InnerSize(egui::vec2(500.0, 350.0)));
                let nstate=self.Login.run( ctx, frame);
                self.State = nstate;
            }
            PageState::MAIN => {
                let setx=1500.0;
                let sety=900.0;
                ctx.send_viewport_cmd(egui::ViewportCommand::InnerSize(egui::vec2(setx, sety)));
                let screen_rect = ctx.screen_rect();
                //let screen_width = screen_rect.width();
                //let screen_height = screen_rect.height();
                
                //if screen_width==setx && screen_height==sety{
                let nstate: PageState=self.Main.run( ctx, frame);
                self.State = nstate;
                //}
                //else{
                    println!("Screen rect: {:?}", ctx.screen_rect());
                    println!("Available rect: {:?}", ctx.available_rect());    
                //    println!("Screen size is not set to 1500x900");
                //}

            }
            PageState::FILE => {
                //self.File.run();
            }
            PageState::MONITER => {
                //self.Moniter.run();
            }
            PageState::MAKEACCOUNT => {
                ctx.send_viewport_cmd(egui::ViewportCommand::InnerSize(egui::vec2(850.0, 450.0)));
                let nstate= self.MakeAccountPage.run( ctx, frame);
                self.State = nstate;
            }
        }


    }
}

pub fn run_ui(){
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
    let _ = eframe::run_native(
        "test",
        options,
        Box::new(|cc| Ok(Box::new(MyApp::new(cc)))),
    );
}