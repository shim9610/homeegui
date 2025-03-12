use crate::app::{Page, PageState};
use Rusty_egui::egui::UiBuilder;
use crate::ui_styles::UiStyle;
use Rusty_egui::egui;
use Rusty_egui::eframe;
use crate::ui_styles::{ContextStyle, WidgetStyle};
use crate::egui::Rect;
use usvg;
use resvg;
use egui::ColorImage;
use Rusty_egui::image::{ImageBuffer, Rgba};
use tiny_skia;
const LOCK_ICON: &[u8] = include_bytes!("icon/lock.svg");
const SETTINGS_ICON: &[u8] = include_bytes!("icon/setting.svg");
const PLAY_ICON: &[u8] = include_bytes!("icon/Play Arrow.svg");
const BACK_ICON: &[u8] = include_bytes!("icon/back.svg");
const FORWARD_ICON: &[u8] = include_bytes!("icon/Forward.svg");


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

        let margin:f32 = 10.0;
        let  window_rect = Rect::from_min_size(
            egui::pos2(window_rect_no_margin.min.x + margin, window_rect_no_margin.min.y + margin),
            egui::vec2(window_rect_no_margin.width() - margin , window_rect_no_margin.height() - margin )
        );
        // 비율 상수들
        let top_layer_ratio = 0.15;      // 상단 영역 높이 비율
        let bottom_layer_ratio = 0.1;   // 하단 영역 높이 비율
        let left_side_ratio = 0.2;      // 좌측 영역 너비 비율
        
        let left_top_ratio = 0.5;       // 좌측 상단 높이 비율 (좌측 영역 내에서)
        let left_bottom_ratio = 0.5;    // 좌측 하단 높이 비율
        
        let right_top_ratio = 0.2;      // 우측 상단 높이 비율 (우측 영역 내에서)
        let right_bottom_ratio = 0.8;   // 우측 하단 높이 비율
        
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

pub struct MainPage {
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
}
impl MainPage {
    pub fn new(ctx: &egui::Context, name: &str) -> Self {
        // SVG 아이콘 로딩
        let lock_icon = load_svg_icon(ctx, LOCK_ICON);
        let settings_icon = load_svg_icon(ctx, SETTINGS_ICON);
        let play_icon = load_svg_icon(ctx, PLAY_ICON);
        let back_icon = load_svg_icon(ctx, BACK_ICON);
        let forward_icon = load_svg_icon(ctx, FORWARD_ICON);
        let current_left_tab = LeftTabState::None;
        
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

    fn render_left_top(&mut self, ui: &mut egui::Ui) {
        ui.vertical(|ui| {
            ui.heading("탐색기");
            ui.separator();
            
            // 탭 버튼 UI
            ui.horizontal(|ui| {
                if ui.selectable_label(self.current_left_tab == LeftTabState::Files, "파일").clicked() {
                    self.current_left_tab = LeftTabState::Files;
                }
                
                if ui.selectable_label(self.current_left_tab == LeftTabState::Favorites, "즐겨찾기").clicked() {
                    self.current_left_tab = LeftTabState::Favorites;
                }
                
                if ui.selectable_label(self.current_left_tab == LeftTabState::Recent, "최근").clicked() {
                    self.current_left_tab = LeftTabState::Recent;
                }
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
                    print!("init");
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
    
    fn render_right_top(&mut self, ui: &mut egui::Ui) {
        ui.label("우측 상단");
    }
    
    fn render_right_bottom(&mut self, ui: &mut egui::Ui) {
        // 우측 하단 UI 코드
        ui.label("우측 하단");
    }
}





impl Page  for MainPage {

    fn run(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame)->PageState {
        let _ = &ctx.apply_style(&UiStyle::deep_navy(2));
        let mut returnV=PageState::MAIN;
        egui::CentralPanel::default()
        .frame(egui::Frame {
            outer_margin: Rusty_egui::egui::Margin::same(25.0),
            inner_margin: Rusty_egui::egui::Margin::same(25.0),
        ..Default::default()
        }).apply_style(&UiStyle::dark_blue()).show(ctx, |ui| {
            let full_rect = ui.max_rect();
            self.area._initialize(full_rect);

            ui.allocate_new_ui(self.area.top_layer.clone(), |ui| {
                self.render_top_layer(ui,&mut returnV);
            });
            ui.allocate_new_ui(self.area.left_top.clone(), |ui| {
                self.render_left_top(ui);
            });
            ui.allocate_new_ui(self.area.left_bottom.clone(), |ui| {
                self.render_left_bottom(ui);
            });
            ui.allocate_new_ui(self.area.right_top.clone(), |ui| {
                self.render_right_top(ui);
            });
            ui.allocate_new_ui(self.area.right_bottom.clone(), |ui| {
                self.render_right_bottom(ui);
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