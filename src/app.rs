use Rusty_egui::eframe;
use Rusty_egui::egui;
use Rusty_egui::eframe::{App, CreationContext};
use crate::login::{LoginPage,MakeAccountPage};
use crate::dashboard::MainPage;
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
                ctx.send_viewport_cmd(egui::ViewportCommand::InnerSize(egui::vec2(1500.0, 900.0)));
                let nstate: PageState=self.Main.run( ctx, frame);
                self.State = nstate;
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