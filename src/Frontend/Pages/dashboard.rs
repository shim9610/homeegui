use crate::Frontend::app::{Page, PageState};
use Rusty_egui::egui::UiBuilder;
use crate::Frontend::Utility::ui_styles::{UiStyle,EmptyRenderer};
use crate::Frontend::Utility::area_slicer::{AreaSlicer,DefaultAreaSlicer,FileSlicer};
use crate::Frontend::Utility::area_slicer::SliceDirection;
use Rusty_egui::egui;
use Rusty_egui::eframe;
use crate::Frontend::Utility::ui_styles::{ContextStyle, WidgetStyle};
use Rusty_egui::egui::Rect;
use usvg;
use resvg;
use egui::ColorImage;
use crate::Frontend::Utility::icon_loader::{IconButton,Icon,ButtonStyle,ToggleController,TapPage};
use Rusty_egui::image::{ImageBuffer, Rgba};
use tiny_skia;
use std::rc::Rc;
use std::cell::RefCell;
const LOCK_ICON: &[u8] = include_bytes!("icon/lock.svg");
const SETTINGS_ICON: &[u8] = include_bytes!("icon/setting.svg");
const PLAY_ICON: &[u8] = include_bytes!("icon/Play Arrow.svg");
const BACK_ICON: &[u8] = include_bytes!("icon/back.svg");
const FORWARD_ICON: &[u8] = include_bytes!("icon/Forward.svg");
use std::collections::HashMap;


struct FILEINFO{
    name: String,
    size: String,
    date: String,
    owner: String,
    status: String,
}
struct FileData{
    files: FILEINFO,
    index: usize,
}
struct Filesystem{
    file_list: Vec<FileData>,
    current_index: usize,
}
impl Filesystem{
    fn new()->Self{
        Self{
            file_list: Vec::new(),
            current_index: 0,
        }
    }
    fn add_file(&mut self,mut file: FileData){
        self.current_index=self.current_index+1;
        file.index=self.current_index;
        self.file_list.push(file);
    }
    fn get_file(&self, index: usize)->Option<&FileData>{
        if index>=self.current_index{
            print!("Index out of range");
            None

        }else{
            Some(&self.file_list[index])}
    }
    fn get_current_file(&self)->&FileData{
        &self.file_list[self.current_index]
    }
    fn get_current_index(&self)->usize{
        self.current_index
    }
}


fn load_svg_icon(ctx: &egui::Context, svg_bytes: &[u8]) -> egui::TextureHandle {
    // SVG를 ColorImage로 변환
    let image = load_svg_as_color_image(svg_bytes);
    
    // ColorImage는 ImageData로 변환 가능
    ctx.load_texture(
        "icon",
        image,
        egui::TextureOptions::default()
    )
}
fn _debug_save_color_image_as_png(color_image: &ColorImage, path: &str) {
    let width = color_image.width();
    let height = color_image.height();

    let mut img_buf = ImageBuffer::new(width as u32, height as u32);

    for y in 0..height {
        for x in 0..width {
            // ColorImage는 (r, g, b, a) 각 8비트
            let c = color_image[(x, y)];
            let pixel = Rgba([c.r(), c.g(), c.b(), c.a()]);
            img_buf.put_pixel(x as u32, y as u32, pixel);
        }
    }

    // 실제로 PNG로 저장
    img_buf
        .save(path)
        .unwrap_or_else(|e| eprintln!("PNG 파일 저장 실패: {}", e));
}
fn load_svg_as_color_image(svg_bytes: &[u8]) -> egui::ColorImage {
    use usvg::{Tree, Options};
    use resvg;

    let options = Options::default();
    let rtree = Tree::from_data(svg_bytes, &options).expect("SVG 파싱 실패");

    let pixmap_size = rtree.size().to_int_size();
    let (w, h) = (pixmap_size.width() as u32, pixmap_size.height() as u32);

    let mut pixmap = tiny_skia::Pixmap::new(w, h).unwrap();

    // pixmap.as_mut() : Option<PixmapMut<'_>>
    // 잘못된 예 (직접 as_mut() 결과를 넣거나...)
    resvg::render(&rtree, tiny_skia::Transform::default(),&mut pixmap.as_mut());
    // ↑ 여기서 타입 불일치: 
    //   expected `&mut PixmapMut`, found `Option<PixmapMut>`

    let rgba_data = pixmap.data();
    egui::ColorImage::from_rgba_unmultiplied([w as usize, h as usize], rgba_data)
}
#[derive(Clone)]
struct ExplorerPage{
    _name: String,
    _id_field: String,  // 이렇게 필드 추가
    draw : bool,
}
impl TapPage for ExplorerPage{
    fn new(input:&str) -> Self {
        Self {
            _name: input.to_string(),
            _id_field: String::new(),
            draw:false,

        }
    }
    fn add(&mut self,item: &str){

    }

    fn render(&mut self, ui: &mut egui::Ui,ctx: &egui::Context) {
        if self.draw{
            if IconButton::new(ctx, Icon::FOLDER2, ButtonStyle::Explorer)
            .size(egui::vec2(40.0, 40.0))
            .tooltip("그런데 파일 이름이 길어지면 어디서 줄이 바뀌지?>.,.,.ㄹ이아루니아룬이라누이라ㅜ")
            .show(ui).clicked(){

            }
        }
    }
    fn clone_page(&self) -> Box<dyn TapPage> {
        Box::new(Self::clone(self))
    }
    fn activate(&mut self) {
        self.draw=true;
    }
    fn deactivate(&mut self) {
        self.draw=false;
    }

    
}
impl ExplorerPage{
    fn file(&mut self,ctx:&egui::Context,ui: &mut egui::Ui){
        if self.draw{
            if IconButton::new(ctx, Icon::FOLDER2, ButtonStyle::Explorer)
            .size(egui::vec2(40.0, 40.0))
            .tooltip("file1")
            .show(ui).clicked(){
                println!("File1 Clicked");

            }
            self.draw=false;
        }
    }
}
#[derive(Clone)]
struct Basepage{
    _name: String,
    _id_field: String,  // 이렇게 필드 추가
    draw : bool,
}
impl TapPage for Basepage{
    fn new(input:&str) -> Self {
        Self {
            _name: input.to_string(),
            _id_field: String::new(),
            draw:false,
        }
    }
    fn add(&mut self,item: &str){

    }

    fn render(&mut self, ui: &mut egui::Ui,ctx: &egui::Context) {
        if self.draw{
            ui.label("Base Page");
            self.draw=false;
        }
        
    }
    fn clone_page(&self) -> Box<dyn TapPage> {
        Box::new(Self::clone(self))
    }
    fn activate(&mut self) {
        self.draw=true;
    }
    fn deactivate(&mut self) {
        self.draw=false;
    }
}

struct AreaStructure {
    window: Rect,
    top_layer: UiBuilder,
    left_top: UiBuilder,  // 이렇게 필드 추가
    left_bottom: UiBuilder,
    right_top: UiBuilder,
    right_bottom: UiBuilder,
    bottom_layer: UiBuilder,
}

#[derive(PartialEq, Clone, Copy)]
enum LeftTabState {
    Files,
    Favorites,
    Recent,
    None,
}

impl AreaStructure {
    pub fn new() -> Self {
        Self {
            window: Rect::from_min_size(egui::Pos2::new(0.0, 0.0), egui::vec2(500.0, 350.0)),
            top_layer: UiBuilder::default(),
            left_top: UiBuilder::default(),
            left_bottom: UiBuilder::default(),
            right_top: UiBuilder::default(),
            right_bottom: UiBuilder::default(),
            bottom_layer: UiBuilder::default(),
        }
    }
    pub fn _initialize(&mut self, window_rect_no_margin: Rect) {

        let margin:f32 = 5.0;
        let width =window_rect_no_margin.width();
        let height = window_rect_no_margin.height();

        let  window_rect = Rect::from_min_size(
            egui::pos2(window_rect_no_margin.min.x + margin, window_rect_no_margin.min.y + margin),
            egui::vec2(width - margin , height - margin )
        );
        // 비율 상수들
        let width_debug =window_rect.width();
        let height_debug = window_rect.height();
        println!("Window rect width: {:?}", width_debug);
        println!("Window rect height: {:?}", height_debug);
        let top_layer_ratio = 0.05;      // 상단 영역 높이 비율
        let bottom_layer_ratio = 0.1;   // 하단 영역 높이 비율
        let left_side_ratio = 0.2;      // 좌측 영역 너비 비율
        
        let left_top_ratio = 0.5;       // 좌측 상단 높이 비율 (좌측 영역 내에서)
        let left_bottom_ratio = 0.5;    // 좌측 하단 높이 비율
        
        let right_top_ratio = 0.2;      // 우측 상단 높이 비율 (우측 영역 내에서)
        let right_bottom_ratio = 0.9;   // 우측 하단 높이 비율
        
        // 전체 창 크기 저장
        self.window = window_rect;
        
        let width = window_rect.width();
        let height = window_rect.height();
        
        // 상단 레이어
        let top_height = height * top_layer_ratio;
        let top_rect = Rect::from_min_size(
            window_rect.min,
            egui::vec2(width, top_height)
        );
        self.top_layer = UiBuilder::new().max_rect(top_rect);
        
        // 하단 레이어
        let bottom_height = height * bottom_layer_ratio;
        let bottom_rect = Rect::from_min_size(
            egui::pos2(window_rect.min.x, window_rect.max.y - bottom_height),
            egui::vec2(width, bottom_height)
        );
        self.bottom_layer = UiBuilder::new().max_rect(bottom_rect);
        
        // 중앙 영역
        let middle_height = height - top_height - bottom_height;
        let middle_min_y = window_rect.min.y + top_height;
        
        // 좌우 분할
        let left_width = width * left_side_ratio;
        let right_width = width - left_width;
        
        // 좌측 상단
        let left_top_height = middle_height * left_top_ratio;
        let left_top_rect = Rect::from_min_size(
            egui::pos2(window_rect.min.x, middle_min_y),
            egui::vec2(left_width, left_top_height)
        );
        self.left_top = UiBuilder::new().max_rect(left_top_rect);
        
        // 좌측 하단
        let left_bottom_rect = Rect::from_min_size(
            egui::pos2(window_rect.min.x, middle_min_y + left_top_height),
            egui::vec2(left_width, middle_height * left_bottom_ratio)
        );
        self.left_bottom = UiBuilder::new().max_rect(left_bottom_rect);
        
        // 우측 상단
        let right_top_height = middle_height * right_top_ratio;
        let right_top_rect = Rect::from_min_size(
            egui::pos2(window_rect.min.x + left_width, middle_min_y),
            egui::vec2(right_width, right_top_height)
        );
        self.right_top = UiBuilder::new().max_rect(right_top_rect);
        
        // 우측 하단
        let right_bottom_rect = Rect::from_min_size(
            egui::pos2(window_rect.min.x + left_width, middle_min_y + right_top_height),
            egui::vec2(right_width, middle_height * right_bottom_ratio)
        );
        self.right_bottom = UiBuilder::new().max_rect(right_bottom_rect);
    }

}

pub struct MainPage <'a>{
    _name: String,
    _id_field: String,  // 이렇게 필드 추가
    area: AreaStructure,
    //lock.svg, setting.svg Play Arrow Skip Next Stop_music Skip Previous Forward back
    lock_icon: egui::TextureHandle,
    settings_icon: egui::TextureHandle,
    back_icon: egui::TextureHandle,
    play_icon: egui::TextureHandle,
    forward_icon: egui::TextureHandle,
    current_left_tab: LeftTabState,
    toggle_set:ToggleController,
    explorer: Option<Rc<RefCell<Box<dyn TapPage>>>>,
    slicer : Option<DefaultAreaSlicer<'a>>,
}
impl<'a> MainPage <'a>{
    pub fn new(ctx: &egui::Context, name: &str) -> Self 

    {
        // SVG 아이콘 로딩
        let lock_icon = load_svg_icon(ctx, LOCK_ICON);
        let settings_icon = load_svg_icon(ctx, SETTINGS_ICON);
        let play_icon = load_svg_icon(ctx, PLAY_ICON);
        let back_icon = load_svg_icon(ctx, BACK_ICON);
        let forward_icon = load_svg_icon(ctx, FORWARD_ICON);
        let current_left_tab = LeftTabState::None;
        let mut toggle=ToggleController::new();
        let slicer_n=None;
        toggle.add::<fn(),Basepage>(
            IconButton::new(ctx, Icon::CLOUD_WITH_BK, ButtonStyle::Menu)
            .size(egui::vec2(24.0, 24.0))
            .with_style(&UiStyle::deep_navy(1))  
            .with_hover_style(&UiStyle::deep_navy(1))
            .with_click_style(&UiStyle::bright_blue())
            .tooltip("Data Cloud"),
            None as Option<fn()>,
            Some(Rc::new(RefCell::new(Box::new(Basepage::new("Data Cloud")) as Box<dyn TapPage>)))

        );
        toggle.add::<fn(),Basepage>(IconButton::new(ctx, Icon::DOCKER, ButtonStyle::Menu)
        .size(egui::vec2(24.0, 24.0))
        .with_style(&UiStyle::deep_navy(1))  
        .with_hover_style(&UiStyle::deep_navy(1))
        .with_click_style(&UiStyle::bright_blue())
        .tooltip("Docker Management"),Some(|| println!("Docker Clicked")),Some(Rc::new(RefCell::new(Box::new(Basepage::new("Docker Management")) as Box<dyn TapPage>))));
        toggle.add::<fn(),Basepage>(IconButton::new(ctx, Icon::CONTROLBAR, ButtonStyle::Menu)
        .size(egui::vec2(24.0, 24.0))
        .with_style(&UiStyle::deep_navy(1))  
        .with_hover_style(&UiStyle::deep_navy(1))
        .with_click_style(&UiStyle::bright_blue())
        .tooltip("Control Panel"),None as Option<fn()>,Some(Rc::new(RefCell::new(Box::new(Basepage::new("Control Panel")) as Box<dyn TapPage>))));
        toggle.add::<fn(),Basepage>(IconButton::new(ctx, Icon::FILE, ButtonStyle::Menu)
        .size(egui::vec2(24.0, 24.0))
        .with_style(&UiStyle::deep_navy(1))  
        .with_hover_style(&UiStyle::deep_navy(1))
        .with_click_style(&UiStyle::bright_blue())
        .tooltip("Terminal"),None as Option<fn()>,Some(Rc::new(RefCell::new(Box::new(Basepage::new("Terminal")) as Box<dyn TapPage>))));
        let subpage=Some(Rc::new(RefCell::new(Box::new(ExplorerPage::new("Terminal")) as Box<dyn TapPage>)));
        toggle.add::<fn(),Basepage>(IconButton::new(ctx, Icon::FOLDER2, ButtonStyle::Menu)
        .size(egui::vec2(24.0, 24.0))
        .with_style(&UiStyle::deep_navy(1))  
        .with_hover_style(&UiStyle::deep_navy(1))
        .with_click_style(&UiStyle::bright_blue())
        .tooltip("Explorer"),None as Option<fn()>,subpage.clone());

        Self {
            _name: name.to_string(),
            _id_field: String::new(),
            area: AreaStructure::new(),
            lock_icon,
            settings_icon,
            play_icon,
            back_icon,
            forward_icon,
            current_left_tab,
            toggle_set:toggle,
            explorer: subpage.clone(), 
            slicer:slicer_n,         
        }
    }
    fn render_top_layer(&mut self, ui: &mut egui::Ui,returnV:&mut PageState) {
        *returnV=PageState::MAIN;
        ui.horizontal(|ui| {
            // 왼쪽 영역: 뒤로가기/앞으로가기 버튼
            let back_button = egui::ImageButton::new(
                egui::load::SizedTexture::new(self.back_icon.id(), egui::vec2(20.0, 20.0))
            ).frame(true);  // 프레임(배경) 추가
            if ui.add(back_button).clicked() {
                // 뒤로가기 기능
            }
            let forward_button = egui::ImageButton::new(
                egui::load::SizedTexture::new(self.forward_icon.id(), egui::vec2(20.0, 20.0))
            ).frame(true);  // 프레임(배경) 추가
            if ui.add(forward_button).clicked() {
                // 뒤로가기 기능
            }
            
            // 중앙 영역: 앱 제목
            ui.with_layout(egui::Layout::centered_and_justified(egui::Direction::LeftToRight), |ui| {
                
            });
            
            // 오른쪽 영역: 아이콘들
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                
                let lock_button = egui::ImageButton::new(
                    egui::load::SizedTexture::new(self.lock_icon.id(), egui::vec2(20.0, 20.0))
                ).frame(true);  // 프레임(배경) 추가
                if ui.add(lock_button).clicked() {
                    *returnV=PageState::LOGIN;
                    //잠금 기능
                }
                let settings_button = egui::ImageButton::new(
                    egui::load::SizedTexture::new(self.settings_icon.id(), egui::vec2(20.0, 20.0))
                ).frame(true);  // 프레임(배경) 추가
                if ui.add(settings_button).clicked() {
                    // 설정 기능
                }

            });
        });
    }
    //Plain,     // 프레임 없음
    //Framed,    // 프레임 있음
    //Menu,      // 메뉴 항목 스타일
    //Primary,   // 주요 액션 버튼
    //Secondary, // 보조 액션 버튼
    fn render_left_top(&mut self, ui : &mut egui::Ui,ctx: &egui::Context) {
        ui.vertical_centered(|ui| {
            ui.label("Memu");
            ui.separator();        
            ui.vertical_centered(|ui| {
            
            self.toggle_set.show(ui,ctx);
            //self.toggle_set.update_page(ui,ctx);
            });
        
        ui.vertical_centered(|ui| {
            
        });
        ui.separator();
            
            // 현재 선택된 탭 내용 표시
            match self.current_left_tab {
                LeftTabState::Files => {
                    ui.label("파일 목록");
                    // 파일 목록 UI 구현
                },
                LeftTabState::Favorites => {
                    ui.label("즐겨찾기 목록");
                    // 즐겨찾기 목록 UI 구현
                },
                LeftTabState::Recent => {
                    ui.label("최근 파일 목록");
                    // 최근 파일 목록 UI 구현
                },
                LeftTabState::None => {
                    
                }
            }
        });
    }
    
    fn render_left_bottom(&mut self, ui: &mut egui::Ui) {
        // 좌측 하단 UI 코드
        ui.label("좌측 하단");
    }

    fn render_bottom_layer(&mut self, ui: &mut egui::Ui) {
        // 상단 레이어 UI 코드
        ui.label("하단레이어");
    }
    
    fn render_right_top(&mut self, ui: &mut egui::Ui,ctx : &egui::Context) {
        // 우측 상단 UI 코드
        ui.label("우측 상단");
    }
    
    fn render_right_bottom(&mut self, ui: &mut egui::Ui,ctx : &egui::Context) {
            //let ctx_clone = ctx.clone();
            //println!("{:?}",ctx_clone.screen_rect());
            let right_bottom_rect = ui.max_rect(); // 현재 우하단 UI 영역
            let mut file = FileSlicer::new(40.0,80.0,50.0,20.0,right_bottom_rect);
            file.set_number_of_grid();
            let ctx_clone = ctx.clone();
            for _i in 0..40 {
                let ctx_inner_clone = ctx_clone.clone();
                if let Some(rc_page) = &self.explorer {
                    let rc_page_clone = rc_page.clone();  // Rc 복제
                    file.add_file(ui, move |ui_param| {
                        let mut page_ref = rc_page_clone.borrow_mut();  // 클로저 내부에서 borrow_mut
                        page_ref.render(ui_param, &ctx_inner_clone);
                    });
                }
            }
    }
}

impl<'a> Page  for MainPage<'a> {
    fn run(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame)->PageState {
        self.area=AreaStructure::new();
        let full_rect =ctx.screen_rect();
        self.area._initialize(full_rect);
        println!("Screen rect inside: {:?}", full_rect);
        let _ = &ctx.apply_style(&UiStyle::deep_navy(2));
        let mut returnV=PageState::MAIN;
        egui::CentralPanel::default()
        .frame(egui::Frame {
            outer_margin: Rusty_egui::egui::Margin::same(25.0),
            inner_margin: Rusty_egui::egui::Margin::same(25.0),
        ..Default::default()
        }).apply_style(&UiStyle::dark_blue()).show(ctx, |ui| {
            //let full_rect = ui.max_rect();
            //self.area._initialize(full_rect);

            ui.allocate_new_ui(self.area.top_layer.clone(), |ui| {
                self.render_top_layer(ui,&mut returnV);
            });
            ui.allocate_new_ui(self.area.left_top.clone(), |ui| {
                self.render_left_top(ui,ctx);
            });
            ui.allocate_new_ui(self.area.left_bottom.clone(), |ui| {
                self.render_left_bottom(ui);
            });
            ui.allocate_new_ui(self.area.right_top.clone(), |ui| {
                self.render_right_top(ui,ctx);
            });
            ui.allocate_new_ui(self.area.right_bottom.clone(), |ui| {
                self.render_right_bottom(ui,ctx);
            });
            ui.allocate_new_ui(self.area.bottom_layer.clone(), |ui| {
                self.render_bottom_layer(ui);
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
//lock.svg, setting.svg Play Arrow Skip Next Stop_music Skip Previous Forward back