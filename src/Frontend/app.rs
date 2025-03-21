use Rusty_egui::eframe;
use Rusty_egui::egui;
use Rusty_egui::eframe::{App, CreationContext};
use crate::Frontend::Pages::login::{LoginPage,MakeAccountPage};
use crate::Frontend::Pages::dashboard::MainPage;
use crate::Frontend::Utility::event_manager::PointScanner;
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
    login :Box<dyn Page>,
    main :Box<dyn Page>,
    file :Box<dyn Page>,
    moniter :Box<dyn Page>,
    make_account_page : Box<dyn Page>,
    state : PageState,
    mouse_interactor : PointScanner,

    

}

impl MyApp  {
    pub fn new(_cc: &CreationContext) -> Self {
        Rusty_egui::replace_fonts(&_cc.egui_ctx);
        let mouse = PointScanner::new();
        Self {
            login: Box::new(LoginPage::new("Login")),
            main: Box::new(MainPage::new(&_cc.egui_ctx,"Main")),
            file: Box::new(LoginPage::new("File")),
            moniter: Box::new(LoginPage::new("Moniter")),
            make_account_page: Box::new(MakeAccountPage::new("MakeAccount")),
            state: PageState::LOGIN,
            mouse_interactor:mouse,
        }
    }
}


//run(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame)
// 애플리케이션 업데이트 구현
impl App for MyApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        let drag_info=self.mouse_interactor.check(ctx);       
        if let Some(prdata)=drag_info{
        println!("드래그앤 드롭 인식 디버그{:?}",prdata);
        }
        match self.state {
            PageState::LOGIN => {
                ctx.send_viewport_cmd(egui::ViewportCommand::InnerSize(egui::vec2(500.0, 350.0)));
                let nstate=self.login.run( ctx, frame);
                self.state = nstate;
            }
            PageState::MAIN => {
                let setx=1500.0;
                let sety=900.0;
                ctx.send_viewport_cmd(egui::ViewportCommand::InnerSize(egui::vec2(setx, sety)));
                let screen_rect = ctx.screen_rect();
                //let screen_width = screen_rect.width();
                //let screen_height = screen_rect.height();
                
                //if screen_width==setx && screen_height==sety{
                let nstate: PageState=self.main.run( ctx, frame);
                self.state = nstate;
                //}
                //else{
                   // println!("Screen rect: {:?}", ctx.screen_rect());
                   // println!("Available rect: {:?}", ctx.available_rect());    
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
                let nstate= self.make_account_page.run( ctx, frame);
                self.state = nstate;
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