use Rusty_egui::egui::{self, Response};
use usvg;
use resvg;
use egui::ColorImage;
use Rusty_egui::image::{ImageBuffer, Rgba};
use tiny_skia;
use crate::Frontend::Utility::ui_styles::UiStyle;
use crate::Frontend::Utility::area_slicer::FileVec;
use std::rc::Rc;
use std::cell::RefCell;






#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Icon {
    SKIP_NEXT,
    CLOUD_WITH_BK,
    FILE,
    DATABASE,
    DOC,
    STOP_MUSIC,
    DATABASE3,
    SPEAKER,
    FOLDER2,
    PEOPLES,
    DATABASEWITHBACKGROUND,
    FOLDER,
    SETTING,
    READING_GLASSES,
    FILE_WITH_BACKGROUND,
    AVI,
    MANY_PEOPLES,
    SKIP_PREVIOUS,
    HOME,
    PLUS,
    BELL,
    LOCK,
    PLAY_ARROW,
    BACK,
    FORWARD,
    CONTROLBAR,
    ZIP,
    MAIL,
    MUSICFILE,
    HOURGLASS,
    QUESTION,
    BELL2,
    FILE2,
    DOCKER,
}

const skip_next : &[u8]= include_bytes!("icon/Skip Next.svg");
const cloud_with_bk : &[u8]= include_bytes!("icon/cloud with bk.svg");
const file : &[u8]= include_bytes!("icon/file.svg");
const database : &[u8]= include_bytes!("icon/database.svg");
const doc : &[u8]= include_bytes!("icon/doc.svg");
const stop_music : &[u8]= include_bytes!("icon/Stop_music.svg");
const database3 : &[u8]= include_bytes!("icon/database3.svg");
const speaker : &[u8]= include_bytes!("icon/speaker.svg");
const folder2 : &[u8]= include_bytes!("icon/folder2.svg");
const peoples : &[u8]= include_bytes!("icon/peoples.svg");
const databasewithbackground : &[u8]= include_bytes!("icon/databasewithbackground.svg");
const folder : &[u8]= include_bytes!("icon/folder.svg");
const setting : &[u8]= include_bytes!("icon/setting.svg");
const reading_glasses : &[u8]= include_bytes!("icon/reading_glasses.svg");
const file_with_background : &[u8]= include_bytes!("icon/file with background.svg");
const avi : &[u8]= include_bytes!("icon/avi.svg");
const many_peoples : &[u8]= include_bytes!("icon/many peoples.svg");
const skip_previous : &[u8]= include_bytes!("icon/Skip Previous.svg");
const home : &[u8]= include_bytes!("icon/home.svg");
const plus : &[u8]= include_bytes!("icon/plus.svg");
const bell : &[u8]= include_bytes!("icon/bell.svg");
const lock : &[u8]= include_bytes!("icon/lock.svg");
const play_arrow : &[u8]= include_bytes!("icon/Play Arrow.svg");
const back : &[u8]= include_bytes!("icon/back.svg");
const forward : &[u8]= include_bytes!("icon/Forward.svg");
const controlbar : &[u8]= include_bytes!("icon/controlbar.svg");
const zip : &[u8]= include_bytes!("icon/zip.svg");
const mail : &[u8]= include_bytes!("icon/mail.svg");
const musicfile : &[u8]= include_bytes!("icon/musicfile.svg");
const hourglass : &[u8]= include_bytes!("icon/hourglass.svg");
const question : &[u8]= include_bytes!("icon/question.svg");
const bell2 : &[u8]= include_bytes!("icon/bell2.svg");
const file2 : &[u8]= include_bytes!("icon/file2.svg");
const docker : &[u8]= include_bytes!("icon/docker.svg");

impl Icon {
    pub fn data(&self) -> &'static [u8] {
        match self {
            Icon::SKIP_NEXT => skip_next,
            Icon::CLOUD_WITH_BK => cloud_with_bk,
            Icon::FILE => file,
            Icon::DATABASE => database,
            Icon::DOC => doc,
            Icon::STOP_MUSIC => stop_music,
            Icon::DATABASE3 => database3,
            Icon::SPEAKER => speaker,
            Icon::FOLDER2 => folder2,
            Icon::PEOPLES => peoples,
            Icon::DATABASEWITHBACKGROUND => databasewithbackground,
            Icon::FOLDER => folder,
            Icon::SETTING => setting,
            Icon::READING_GLASSES => reading_glasses,
            Icon::FILE_WITH_BACKGROUND => file_with_background,
            Icon::AVI => avi,
            Icon::MANY_PEOPLES => many_peoples,
            Icon::SKIP_PREVIOUS => skip_previous,
            Icon::HOME => home,
            Icon::PLUS => plus,
            Icon::BELL => bell,
            Icon::LOCK => lock,
            Icon::PLAY_ARROW => play_arrow,
            Icon::BACK => back,
            Icon::FORWARD => forward,
            Icon::CONTROLBAR => controlbar,
            Icon::ZIP => zip,
            Icon::MAIL => mail,
            Icon::MUSICFILE => musicfile,
            Icon::HOURGLASS => hourglass,
            Icon::QUESTION => question,
            Icon::BELL2 => bell2,
            Icon::FILE2 => file2,
            Icon::DOCKER => docker,
        }
    }
}


fn apply_interactive_styles(
    cpdata:IconButton,ui: &mut egui::Ui, response: &egui::Response
) {
        // 상태에 따라 적절한 배경색 선택
        // 상태에 따라 적절한 배경색 선택

        let background_color = if response.clicked() {
            // 클릭 스타일 
            cpdata.click_style.map_or(egui::Color32::from_rgb(37, 99, 235), |s| s.accent)
        } else if response.hovered() {
            if matches!(cpdata.style, ButtonStyle::Explorer) {
                return;
            }
            // 호버 스타일
            cpdata.hover_style.map_or(egui::Color32::from_rgb(80, 140, 230), |s| s.hover)


        } else {
            return; // 호버나 클릭 상태가 아니면 아무것도 하지 않음
        };

        // 배경 다시 그리기
        ui.painter().rect_filled(
            response.rect,
            egui::Rounding::same(2.0), // 약간의 라운딩
            background_color
        );
        
        // 필요시 아이콘 다시 그리기 (호버/클릭 시 아이콘 색상도 변경하려면)
        if cpdata.tint.is_some() || response.hovered() || response.clicked() {
            let icon_pos = egui::pos2(
                response.rect.min.x + 10.0,
                response.rect.center().y - cpdata.size.y / 2.0
            );
            
            let tint = if response.clicked() && cpdata.click_style.is_some() {
                cpdata.click_style.map_or(egui::Color32::WHITE, |s| s.text)
            } else if response.hovered() && cpdata.hover_style.is_some() {
                cpdata.hover_style.map_or(egui::Color32::WHITE, |s| s.text)
            } else {
                cpdata.tint.unwrap_or(egui::Color32::WHITE)
            };
            
            ui.painter().image(
                cpdata.texture.id(),
                egui::Rect::from_min_size(icon_pos, cpdata.size),
                egui::Rect::from_min_max(egui::pos2(0.0, 0.0), egui::pos2(1.0, 1.0)),
                tint
            );
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
    let (w, h) = (pixmap_size.width(), pixmap_size.height());

    let mut pixmap = tiny_skia::Pixmap::new(w, h).unwrap();

    // pixmap.as_mut() : Option<PixmapMut<'_>>
    // 잘못된 예 (직접 as_mut() 결과를 넣거나...)
    resvg::render(&rtree, tiny_skia::Transform::default(),&mut pixmap.as_mut());
    // ↑ 여기서 타입 불일치: 
    //   expected `&mut PixmapMut`, found `Option<PixmapMut>`

    let rgba_data = pixmap.data();
    egui::ColorImage::from_rgba_unmultiplied([w as usize, h as usize], rgba_data)
}


pub struct IconLoader {
    icons: egui::TextureHandle,
    data : &'static [u8],
    ctx: egui::Context,
}

impl IconLoader {
    pub fn new(ctx: & egui::Context,icon_type:Icon) -> Self {
        let data =icon_type.data();
        let icons = load_svg_icon(ctx, data);
        Self {
            icons,
            data,
            ctx: ctx.clone(),
        }
    }
    fn load_icon(&mut self, icon_type: Icon) {
        self.data = icon_type.data();
        self.icons = load_svg_icon(&self.ctx, self.data);
    }
}
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ButtonStyle {
    Plain,     // 프레임 없음
    Framed,    // 프레임 있음
    Menu,      // 메뉴 항목 스타일
    Primary,   // 주요 액션 버튼
    Secondary, // 보조 액션 버튼
    Explorer,
    // 필요한 다른 스타일들...
}
#[derive(Clone)]
pub struct IconButton {
    icon: Icon,
    texture: egui::TextureHandle,
    style: ButtonStyle,
    size: egui::Vec2,
    tint: Option<egui::Color32>,
    selected: bool,
    tooltip: Option<String>,
    // 상태별 스타일 추가
    default_style: Option<UiStyle>,
    hover_style: Option<UiStyle>,
    click_style: Option<UiStyle>,
    id:usize,
    is_drag:bool,
}

impl IconButton {
    pub fn new(ctx: &egui::Context, icon: Icon, button_style: ButtonStyle) -> Self {
        let size = match button_style {
            ButtonStyle::Menu => egui::vec2(16.0, 16.0),
            _ => egui::vec2(20.0, 20.0),
        };
        
        let texture = load_svg_icon(ctx, icon.data());
        
        Self {
            icon,
            texture,
            style: button_style,
            size,
            tint: None,
            selected: false,
            tooltip: None,
            default_style: None,
            hover_style: None,
            click_style: None,
            id:0,
            is_drag:false,
        }
    }
    
    // 기존 메서드들...
    
    // 상태별 스타일 설정 메서드 추가
    pub fn with_style(mut self, style: &UiStyle) -> Self {
        self.default_style = Some(*style);
        self
    }
    
    pub fn with_hover_style(mut self, style: &UiStyle) -> Self {
        self.hover_style = Some(*style);
        self
    }
    
    pub fn with_click_style(mut self, style: &UiStyle) -> Self {
        self.click_style = Some(*style);
        self
    }
    fn framd_taplate(&self, ui: &mut egui::Ui)->Response{
        // 이미지 버튼 처리
        let mut button = egui::ImageButton::new(
            egui::load::SizedTexture::new(self.texture.id(), self.size)
        ).frame(self.style == ButtonStyle::Framed);
        
        if let Some(tint) = self.tint {
            button = button.tint(tint);
        }
        
        ui.add(button)
    }
    fn explorer_taplate(&self, ui: &mut egui::Ui)->Response{
        let text = match &self.tooltip {
            Some(text) => text.as_str(),
            None => "No tooltip"
        };
        
        let (rect, response) = ui.allocate_at_least(self.size, egui::Sense::click());
        
        // 아이콘 그리기
        let icon_pos = egui::pos2(
            rect.center().x - (self.size.x * 0.5),
            rect.center().y - (self.size.y * 0.5),
        );
        ui.painter().image(
            self.texture.id(),
            egui::Rect::from_min_size(icon_pos, self.size),
            egui::Rect::from_min_max(egui::pos2(0.0, 0.0), egui::pos2(1.0, 1.0)),
            egui::Color32::WHITE,
        );
        
        // 텍스트 처리
        // 텍스트 처리
        let max_width = 80.0;  // 최대 너비 제한

        // 먼저 최대 너비로 텍스트 레이아웃 생성
        let text_galley = ui.painter().layout(
            text.to_string(),
            egui::FontId::proportional(10.0),
            egui::Color32::WHITE,
            max_width
        );

        // 행 수 확인 및 제한
        let display_text = if text_galley.rows.len() > 2 {
            // 첫 두 줄의 텍스트 추출을 시도
            // Row 구조체의 public 메서드들만 사용
            
            // 대안: 전체 텍스트를 2줄 높이에 맞게 자르기
            let font_height = 10.0; // 대략적인 행 높이
            let display_galley = ui.painter().layout(
                text.to_string(),
                egui::FontId::proportional(10.0),
                egui::Color32::WHITE,
                max_width
            );
            
            // 2줄까지만 표시
            if display_galley.rows.len() > 2 {
                let result = String::new();
                let mut chars_added = 0;
                
                // 첫 2줄에 해당하는 글자 수 계산
                for (i, row) in display_galley.rows.iter().take(2).enumerate() {
                    // Row에서 직접 텍스트를 추출할 수 없으므로
                    // 글자 위치를 추정
                    let row_chars = if i == 0 {
                        // 첫 번째 줄
                        row.glyphs.len()
                    } else {
                        // 두 번째 줄
                        row.glyphs.len() + chars_added
                    };
                    chars_added = row_chars;
                }
                
                // 추정한 글자 수만큼 원본 텍스트에서 가져오기
                if chars_added > 0 && chars_added < text.chars().count() {
                    let truncated: String = text.chars().take(chars_added).collect();
                    format!("{}...", truncated)
                } else {
                    text.to_string()
                }
            } else {
                text.to_string()
            }
        } else {
            text.to_string()
        };

        // 최종 텍스트 표시
        let final_galley = ui.painter().layout(
            display_text,
            egui::FontId::proportional(10.0),
            egui::Color32::WHITE,
            max_width
        );

        // 텍스트 위치 (중앙 정렬)
        let text_pos = egui::pos2(
            rect.center().x - final_galley.rect.width() / 2.0,
            rect.center().y + (self.size.y * 0.6)
        );

        // 텍스트 그리기
        ui.painter().galley(text_pos, final_galley, egui::Color32::WHITE);
                        
        if response.clicked() {
            println!("Explorer style icon clicked: {}", text);
        }
        
        response
    }
    fn menu_taplate(&self, ui: &mut egui::Ui)->Response{
               // 기본 스타일 적용 (제공된 경우)
               let mut visuals = ui.style().visuals.clone();
               if let Some(default_style) = &self.default_style {
                   visuals.widgets.inactive.bg_fill = default_style.background;
                   visuals.widgets.inactive.fg_stroke = egui::Stroke::new(1.0, default_style.text);
                   visuals.widgets.inactive.bg_stroke = egui::Stroke::new(1.0, default_style.border);
                   
                   visuals.widgets.hovered.bg_fill = default_style.hover;
                   visuals.widgets.hovered.fg_stroke = egui::Stroke::new(1.0, default_style.text);
                   visuals.widgets.hovered.bg_stroke = egui::Stroke::new(1.0, default_style.border);
                   
                   visuals.widgets.active.bg_fill = default_style.accent;
                   visuals.widgets.active.fg_stroke = egui::Stroke::new(1.0, default_style.text);
                   visuals.widgets.active.bg_stroke = egui::Stroke::new(1.0, default_style.border);
               }
               
               // 호버 스타일이 제공된 경우 오버라이드
               if let Some(hover_style) = &self.hover_style {
                   //visuals.widgets.hovered.bg_fill = hover_style.background;
                   visuals.widgets.hovered.bg_fill = hover_style.hover;
                   visuals.widgets.hovered.fg_stroke = egui::Stroke::new(1.0, hover_style.text);//default_style
                   visuals.widgets.hovered.bg_stroke = egui::Stroke::new(1.0, hover_style.border);
               }
               
               // 클릭 스타일이 제공된 경우 오버라이드
               if let Some(click_style) = &self.click_style {
                   //visuals.widgets.active.bg_fill = click_style.background;
                   visuals.widgets.active.bg_fill = click_style.accent;
                   visuals.widgets.active.fg_stroke = egui::Stroke::new(1.0, click_style.text);
                   visuals.widgets.active.bg_stroke = egui::Stroke::new(1.0, click_style.border);
               }
               
               let old_visuals = ui.style().visuals.clone();
               ui.style_mut().visuals = visuals.clone();
               
               let button = egui::Button::new({
               let text = match &self.tooltip {
                   Some(text) => {
                          //println!("{}", text);
                           text.as_str() // 또는 &text[..]
                   },
                   None => {
                           //println!("No tooltip");
                           "No tooltip"
                   }
                   };
                   let mut rich_text = egui::RichText::new(text);
                   if self.selected {
                       rich_text = rich_text.strong();
                   }
                   rich_text
               })
               .fill(if self.selected {
                   self.default_style.map_or(egui::Color32::from_rgb(45, 55, 65), |s| s.selected)
               } else {
                   self.default_style.map_or(egui::Color32::TRANSPARENT, |s| s.background)
               })
               .frame(true);
               
               let response = ui.add_sized([ui.available_width(), 32.0], button);
               
               // 아이콘 그리기
               let rect = response.rect;
               let icon_pos = egui::pos2(
                   rect.min.x + 10.0,
                   rect.center().y - self.size.y / 2.0
               );
               
               ui.painter().image(
                   self.texture.id(),
                   egui::Rect::from_min_size(icon_pos, self.size),
                   egui::Rect::from_min_max(egui::pos2(0.0, 0.0), egui::pos2(1.0, 1.0)),
                   self.tint.unwrap_or(egui::Color32::WHITE)
               );
               
               // 선택된 항목의 왼쪽에 표시기 추가
               if self.selected {
                   let indicator_width = 3.0;
                   let accent_color = self.default_style.map_or(
                       egui::Color32::from_rgb(59, 130, 246), 
                       |s| s.accent
                   );
                   
                   let indicator_rect = egui::Rect::from_min_size(
                       egui::pos2(rect.min.x, rect.min.y),
                       egui::vec2(indicator_width, rect.height())
                   );
                   ui.painter().rect_filled(
                       indicator_rect,
                       0.0,
                       accent_color
                   );
               }
               
               // 원래 스타일로 복원
               ui.style_mut().visuals = old_visuals;
               apply_interactive_styles(self.clone(),ui, &response);
                   response
               
    }
    fn primary_taplate(&self, ui: &mut egui::Ui)->Response{
        // Primary 버튼 스타일도 유사하게 처리
        let accent_color = self.default_style.map_or(
            egui::Color32::from_rgb(37, 99, 235), 
            |s| s.accent
        );
        
        let button = egui::Button::new({
            let text = format!("{:?}", self.icon);
            egui::RichText::new(text).strong()
        })
        .fill(accent_color);
        
        ui.add(button)
    }    
    fn secondary_taplate(&self, ui: &mut egui::Ui)->Response{
        // Secondary 버튼 스타일도 유사하게 처리
        let bg_color = self.default_style.map_or(
            egui::Color32::from_rgb(75, 85, 99), 
            |s| s.selected
        );
        
        let button = egui::Button::new(format!("{:?}", self.icon))
        .fill(bg_color);
        
        ui.add(button)
    }  
    
    fn render_dragged_icon(
        ui: &mut egui::Ui, 
        texture_id: egui::TextureId, 
        position: egui::Pos2, 
        size: egui::Vec2, 
        tint: egui::Color32
    ) {
        // 아이콘을 반투명하게 렌더링
        let dragged_tint = egui::Color32::from_rgba_unmultiplied(
            tint.r(), tint.g(), tint.b(), tint.a() / 2  // 반투명 효과
        );
        
        ui.painter().image(
            texture_id,
            egui::Rect::from_min_size(position, size),
            egui::Rect::from_min_max(egui::pos2(0.0, 0.0), egui::pos2(1.0, 1.0)),
            dragged_tint
        );
    }

    pub fn show(self, ui: &mut egui::Ui) -> egui::Response {
        let mut response = match self.style {
            ButtonStyle::Plain | ButtonStyle::Framed => {
                self.framd_taplate(ui)
            },
            ButtonStyle::Explorer => {
                self.explorer_taplate(ui)
            }
            ButtonStyle::Menu => {
                self.menu_taplate(ui)
            },
            ButtonStyle::Primary => {
                self.primary_taplate(ui)
            },
            ButtonStyle::Secondary => {
                self.secondary_taplate(ui)
            },
        };
        let input=self.clone();
        // 툴팁 추가
        if let Some(tooltip_text) = self.tooltip {
            response = response.on_hover_text(tooltip_text);
        }
    apply_interactive_styles(input,ui, &response);    
        response
    }
    pub fn size(mut self, size: egui::Vec2) -> Self {
        self.size = size;
        self
    }
    
    pub fn tint(mut self, color: egui::Color32) -> Self {
        self.tint = Some(color);
        self
    }
    
    pub fn selected(mut self, selected: bool) -> Self {
        self.selected = selected;
        self
    }
    
    pub fn tooltip(mut self, text: impl Into<String>) -> Self {
        self.tooltip = Some(text.into());
        self
    }
    // 호버/클릭 상태에 따라 스타일 적용하는 헬퍼 메서드

        
}
pub trait TapPage {
    fn new(title: &str) -> Self where Self: Sized;
    fn add(&mut self, item: &str);
    fn render(&mut self, ui: &mut egui::Ui,ctx: &mut egui::Context)->Option<FileVec>;
    fn clone_page(&self) -> Box<dyn TapPage>; // Clone 대신 사용
    fn activate(&mut self);
    fn deactivate(&mut self);
    fn get_draw(&self)->bool;
}

pub struct ToggleController {
    selected: Vec<bool>,
    buttons: Vec<IconButton>,
    callbacks: Vec<Option<Box<dyn Fn()+ 'static>>>, 
    pages: Vec<Option<Rc<RefCell<Box<dyn TapPage>>>>>, // TapPage 트레잇 객체 저장
    index:usize,
    removed:Vec<usize>,
} 
impl ToggleController {
    pub fn new() -> Self {
        let size=0;
        Self {
            selected: vec![false; size],
            buttons: vec![],
            callbacks: Vec::new(),
            pages: vec![], // 페이지 벡터 초기화
            index:0,
            removed:vec![],
        }
    }
    pub fn add<F,T>(&mut self, newbutton: IconButton,callback: Option<F>, page: Option<Rc<RefCell<Box<dyn TapPage>>>>)->usize 
    where 
    F: Fn() + 'static,
    T: TapPage + 'static
    {
        let boxed_callback = callback.map(|f| Box::new(f) as Box<dyn Fn()+ 'static>);
       
        if self.buttons.len()>self.index{
            self.removed.retain(|&x| x != self.index);
            self.selected[self.index]=false;
            self.buttons[self.index]=newbutton;
            self.callbacks[self.index]=boxed_callback;
            self.pages[self.index] = page; // 페이지 업데이트
        }else{
            self.buttons.push(newbutton);
            self.selected.push(false);
            self.callbacks.push(boxed_callback);
            self.pages.push(page); // 새 페이지 추가
            self.index=self.buttons.len();
            
        }
        self.index
    }
    pub fn remove(&mut self, index:usize) {
        self.removed.push(index);
        self.index=index;
    }
    fn call_toggle(&mut self,id:usize,ui: &mut egui::Ui,ctx: &egui::Context){
        
        
        
        if !self.removed.contains(&id) && self.buttons.len()>id {
        for i in 0..self.selected.len() {
            let page_option = &self.pages[i];
            self.selected[i] = false;
            self.buttons[i]=self.buttons[i].clone().with_style(&UiStyle::deep_navy(1));
            if let Some(page_rc) = page_option {
                if let Ok(mut page_ref) = page_rc.try_borrow_mut() {
                    page_ref.deactivate();
                }
                else{
                    print!("Page is already borrowed mutably");
                }
            }       
        }
        self.selected[id]=true;
        self.buttons[id]=self.buttons[id].clone().with_style(&UiStyle::bright_blue());
        let page_option = &self.pages[id];
        if let Some(page_rc) = page_option {
            if let Ok(mut page_ref) = page_rc.try_borrow_mut() {
                page_ref.activate();
            }
            else{
                print!("Page is already borrowed mutably");
            }
        } 
    }
    }
    pub fn update_page(&mut self, ui: &mut egui::Ui, ctx: &egui::Context) {
        if let Some(selected_idx) = self.selected.iter().position(|&selected| selected) {
            if !self.removed.contains(&selected_idx) {
                if let Some(page_rc) = &self.pages[selected_idx] {
                    // try_borrow_mut는 Result를 반환
                    if let Ok(mut page) = page_rc.try_borrow_mut() {
                        page.activate();
                    } else {
                        // 이미 가변 대여 중인 경우 처리
                        println!("페이지가 이미 가변 대여 중입니다.");
                    }
                }
            }
        }
    }


    
    fn show_button(&mut self, id: usize, ui: &mut egui::Ui,ctx:&egui::Context ) -> Option<egui::Response> 
    {
        // 유효한 ID인지 먼저 확인

        if id < self.buttons.len() && !self.removed.contains(&id) {
            // 현재 선택 상태를 버튼에 반영
            let button = self.buttons[id].clone().selected(self.selected[id]);
            // 버튼 렌더링 및 응답 가져오기
            let response = button.show(ui);
            
            // 클릭 이벤트 처리
            if response.clicked() {
                self.call_toggle(id,ui,ctx);
               // println!("{:?}",self.selected);
                            // 외부에서 제공된 콜백 실행
                //let callback=&self.callbacks[id];
            if let Some(callback) = &self.callbacks[id] {
                callback(); 
            }
            }
            
            Some(response)
        } else {
            // 유효하지 않은 ID에 대해 None 반환
            None
        }
    }
    pub fn show(&mut self,  ui: &mut egui::Ui,ctx:&egui::Context){
        for index in 0 .. self.buttons.len(){
            if !self.removed.contains(&index){
                let _ = self.show_button(index,ui,ctx);
            }
        }
    }
}


#[derive(Clone)]
pub struct ExplorerIcon{
    pub file : IconButton
}
impl ExplorerIcon{
    pub fn new(ctx:&mut Rusty_egui::egui::Context,text:String)->ExplorerIcon{
        let data= IconButton::new(ctx, Icon::FOLDER2, ButtonStyle::Explorer)
                .size(egui::vec2(40.0, 40.0))
                .tooltip(text);
        Self{
        file:data
        }
    }
}