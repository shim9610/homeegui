use Rusty_egui::egui::{self, Response};
use Rusty_egui::egui::{Color32,  Stroke, TextureHandle, Ui, Vec2};
use Rusty_egui::egui::style::Visuals;
use usvg;
use resvg;
use egui::ColorImage;
use Rusty_egui::image::{ImageBuffer, Rgba};
use tiny_skia;
use crate::Frontend::Utility::ui_styles::UiStyle;
use crate::Frontend::Utility::area_slicer::FileVec;
use std::rc::Rc;
use std::cell::RefCell;
use std::sync::LazyLock;






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
static DEFAULT_STYLE: LazyLock<ConstDummy> = LazyLock::new(|| {ConstDummy::new(Icon::FILE,ButtonStyle::Explorer)});
//static DEFAULT_BUTTON=LazyLock<StyleTemplate>=LazyLock::new(||{StyleTemplate::new()});
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
/// 예시: 버튼 스타일
#[derive(Debug, Clone)]
pub struct ButtonStruct {
    pub background: Color32,
    pub text: Color32,
    pub border: Color32,
    pub hover: Color32,
    pub accent: Color32,
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
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ButtonStte {
    Selected,     // 선택됨
    Idle,    // 활성화 대기
    Deactivate,      // 비활성화
    Dragging,   // 드래그 중
    // 필요한 다른 스타일들...
}

fn apply_interactive_styles(
    cpdata:Sample,ui: &mut egui::Ui, response: &egui::Response
) {
        // 상태에 따라 적절한 배경색 선택
        // 상태에 따라 적절한 배경색 선택

        let background_color = if response.clicked() {
            // 클릭 스타일 
            cpdata.default_button_style.click_style.map_or(egui::Color32::from_rgb(37, 99, 235), |s| s.accent)
        } else if response.hovered() {
            if matches!(cpdata.default_button_style.style, ButtonStyle::Explorer) {
                return;
            }
            // 호버 스타일
            cpdata.default_button_style.hover_style.map_or(egui::Color32::from_rgb(80, 140, 230), |s| s.hover)


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
        if cpdata.default_button_style.tint.is_some() || response.hovered() || response.clicked() {
            let icon_pos = egui::pos2(
                response.rect.min.x + 10.0,
                response.rect.center().y - cpdata.default_button_style.size.y / 2.0
            );
            
            let tint = if response.clicked() && cpdata.default_button_style.click_style.is_some() {
                cpdata.default_button_style.click_style.map_or(egui::Color32::WHITE, |s| s.text)
            } else if response.hovered() && cpdata.default_button_style.hover_style.is_some() {
                cpdata.default_button_style.hover_style.map_or(egui::Color32::WHITE, |s| s.text)
            } else {
                cpdata.default_button_style.tint.unwrap_or(egui::Color32::WHITE)
            };
            
            ui.painter().image(
                cpdata.texture.id(),
                egui::Rect::from_min_size(icon_pos, cpdata.default_button_style.size),
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
















trait IconButtonStyle:Sized{
    fn new(icon: Icon, button_style: ButtonStyle)->Self where Self : Sized;
    fn button_style_get(self)->Self where Self : Sized;
    fn style_split_get(self)->( Option<UiStyle>, Option<UiStyle>, Option<UiStyle>, Option<UiStyle>, Option<UiStyle>);
    fn with_style(self,input:ButtonStyle)->Self where Self: Sized;
    fn with_default_style(self,input:&UiStyle)->Self where Self: Sized;
    fn with_hover_style(self,input:&UiStyle)->Self where Self: Sized;
    fn with_click_style(self,input:&UiStyle)->Self where Self: Sized;
    fn with_selected_style(self,input:&UiStyle)->Self where Self: Sized;
    fn with_drag_style(self,input:&UiStyle)->Self where Self: Sized;
    fn with_tooltip(self,input:&String)->Self where Self: Sized;
    fn with_size(self,input:&Vec2)->Self where Self: Sized;
    fn with_id(self,input:usize)->Self where Self: Sized;
    fn with_is_drag(self,input:bool)->Self where Self: Sized;
    fn with_selected(self,input:bool)->Self where Self: Sized;
    fn with_tint(self,input:egui::Color32)->Self where Self: Sized;
}
trait RanderTemplateWrapper<T: IconButtonStyle>: Clone {//버튼 탬플릿 정의 구조체 디폴트 함수를 젇의하기 위해 IconButtonStyle트레잇을 사용함. 공통되는 설정은 IconButtonStyle인스턴스에서 설정하고, 버튼별로 다른 설정은 RanderTemplateWrapper 인스턴스에 추가 매서드를 만드는 방식으로 구현
    fn new (ctx: &egui::Context, icon: Icon, button_style: Option<ButtonStyle>)->Self where Self: Sized;
    fn style_get(&mut self)->( Option<UiStyle>, Option<UiStyle>, Option<UiStyle>, Option<UiStyle>, Option<UiStyle>){
        
        self.clone().button_coltroll().style_split_get()
    }
    fn button_coltroll(self)->T;
    fn set_coltrol(&mut self,input:T);
    fn with_style(&mut self,input:ButtonStyle){
        let data = self.clone();
        let result=data.button_coltroll().with_style(input);
        self.set_coltrol(result);
    }
    fn with_default_style(&mut self,input:&UiStyle){
        let data = self.clone();
        let result=data.button_coltroll().with_default_style(input);
        self.set_coltrol(result);
    }
    fn with_hover_style(&mut self,input:&UiStyle){
        let data = self.clone();
        let result=data.button_coltroll().with_hover_style(input);
        self.set_coltrol(result);
    }
    fn with_click_style(&mut self,input:&UiStyle){
        let data = self.clone();
        let result=data.button_coltroll().with_click_style(input);
        self.set_coltrol(result);
    }
    fn with_selected_style(&mut self,input:&UiStyle){
        let data = self.clone();
        let result=data.button_coltroll().with_selected_style(input);
        self.set_coltrol(result);
    }
    fn with_drag_style(&mut self,input:&UiStyle){
        let data = self.clone();
        let result=data.button_coltroll().with_drag_style(input);
        self.set_coltrol(result);
    }
    fn with_tooltip(&mut self,input:&String){
        let data = self.clone();
        let result=data.button_coltroll().with_tooltip(input);
        self.set_coltrol(result);
    }
    fn with_size(&mut self,input:&Vec2){
        let data = self.clone();
        let result=data.button_coltroll().with_size(input);
        self.set_coltrol(result);
    }
    fn with_id(&mut self,input:usize){
        let data = self.clone();
        let result=data.button_coltroll().with_id(input);
        self.set_coltrol(result);
    }
    fn with_is_drag(&mut self,input:bool){
        let data = self.clone();
        let result=data.button_coltroll().with_is_drag(input);
        self.set_coltrol(result);
    }
    fn with_selected(&mut self,input:bool){
        let data = self.clone();
        let result=data.button_coltroll().with_selected(input);
        self.set_coltrol(result);
    }
    fn with_tint(&mut self,input:egui::Color32){
        let data = self.clone();
        let result=data.button_coltroll().with_tint(input);
        self.set_coltrol(result);
    }

}

trait RanderTemplate<T: IconButtonStyle>{
    fn new(size: Vec2) ->Self where Self: Sized;
    fn new_default(default_style: Option<UiStyle>,hover_style: Option<UiStyle>,click_style: Option<UiStyle>,
        tooltip: Option<String>,texture: Option<TextureHandle>,size: Vec2)->impl RanderTemplateWrapper<T>;
    fn render_explorer<RanderTemplateWrapper>(&mut self,mut self_w : T,  ui: &mut Ui) -> Response {ui.button("")}
}



#[derive( Clone)]
struct ConstDummy{// button cotroll struct for default value.
    icon: Icon,
    size: egui::Vec2,
    tint: Option<egui::Color32>,
    style: ButtonStyle,
    selected: bool,
    tooltip:Option<String>,
    id:usize,
    is_drag:bool,
    default_style: Option<UiStyle>,
    hover_style: Option<UiStyle>,
    click_style: Option<UiStyle>,
    selected_style: Option<UiStyle>,
    drag_style : Option<UiStyle>,
}
impl IconButtonStyle for ConstDummy{
    fn new(icon: Icon, button_style: ButtonStyle)->Self{
        let size = match button_style {
            ButtonStyle::Menu => egui::vec2(16.0, 16.0),
            _ => egui::vec2(20.0, 20.0),
        };
        Self{
        icon,
        style:button_style,
        size,
        tint: None,
        selected: false,
        tooltip: None,
        id:0,
        is_drag:false,
        default_style:Some(UiStyle::deep_navy(1)),
        hover_style:Some( UiStyle::deep_navy(1)),
        click_style:Some( UiStyle::deep_navy(1)),
        selected_style:Some(UiStyle::bright_blue()),
        drag_style :Some( UiStyle::deep_navy(1)),
        }
    }
    fn button_style_get(self)->Self{
        self
    }
    fn style_split_get(self)->( Option<UiStyle>, Option<UiStyle>, Option<UiStyle>, Option<UiStyle>, Option<UiStyle>)
    {
       return ( self.default_style.clone(),  self.hover_style.clone(),  self.click_style.clone(),  self.selected_style.clone(),  self.drag_style.clone())
    }
    fn with_style(mut self,input:ButtonStyle)->Self{
        self.style=input;
        self
    }
    fn with_default_style(mut self,input:&UiStyle)->Self {
        self.default_style=Some(*input);
        return self
    }
    fn with_hover_style(mut self,input:&UiStyle)->Self{
        self.hover_style=Some(*input);
        return self
    }
    fn with_click_style(mut self,input:&UiStyle)->Self{
        self.click_style=Some(*input);
        return self
    }
    fn with_selected_style(mut self,input:&UiStyle)->Self{
        self.selected_style=Some(*input);
        return self
    }
    fn with_drag_style(mut self,input:&UiStyle)->Self{
        self.drag_style=Some(*input);
        return self
    }
    fn with_tooltip(mut self,input:&String)->Self{
        self.tooltip=Some(input.clone());
        return self
    }
    fn with_size(mut self,input:&Vec2)->Self{
        self.size=*input;
        return self
    }
    fn with_id(mut self,input:usize)->Self{
        self.id=input;
        return self
    }
    fn with_is_drag(mut self,input:bool)->Self{
        self.is_drag=input;
        return self
    }
    fn with_selected(mut self,input:bool)->Self{
        self.selected=input;
        return self
    }
    fn with_tint(mut self,input:egui::Color32)->Self{
        self.tint=Some(input);
        return self
    }
}

//macro_rules! impl_with_methods {
//    ($STRUCT:ty, $($FIELD:ident,$FIELD_TYPE:ty),*)=>{
//        impl $STRUCT{
//            $(
//                pub fn with_$FIELD(mut self, value:$FIELD_TYPE) ->Self{
//                    self.$FIELD=value;
//                    self
//                }
//            )*
//        }
//    }
//}
//macro_rules! impl_with_option_methods {
//    ($STRUCT:ty, $($FIELD:ident,$FIELD_TYPE:ty),*)=>{
//        impl $STRUCT{
//            $(
//                pub fn with_$FIELD(mut self, value:$FIELD_TYPE) ->Self{
//                    self.$FIELD=Some(*value);
//                    self
//                }
//            )*
//        }
//    }
//}

#[derive(Clone)]
pub struct Sample{
    pub default_button_style:ConstDummy,
    pub texture: TextureHandle,

}
impl RanderTemplateWrapper <ConstDummy> for Sample{
    fn new(ctx: &egui::Context, icon: Icon, input_button_style: Option<ButtonStyle>)->Self{
    let texture: TextureHandle = load_svg_icon(ctx, icon.data());
    if let Some(buttonstyle)=input_button_style{
        let input_st=DEFAULT_STYLE.clone().with_style(buttonstyle);
        Self {
            default_button_style: input_st,
            texture,
        }
    }else{
        let input_st= DEFAULT_STYLE.clone();
        Self {
            default_button_style: input_st,
            texture,
        }
        }
    }
    fn button_coltroll(self)-> ConstDummy{
        self.default_button_style
    }
    fn set_coltrol(&mut self,input:ConstDummy){
        self.default_button_style = input;
    }

}
impl Sample{
    pub fn show(self, ui: &mut egui::Ui) -> egui::Response {
        let mut response = match self.default_button_style.style  {
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
        if let Some(tooltip_text) = self.default_button_style.tooltip {
            response = response.on_hover_text(tooltip_text);
        }
    apply_interactive_styles(input,ui, &response);    
        response
    }
    fn framd_taplate(&self, ui: &mut egui::Ui)->Response{
        // 이미지 버튼 처리
        let mut button = egui::ImageButton::new(
            egui::load::SizedTexture::new(self.texture.id(), self.default_button_style.size)
        ).frame(self.default_button_style.style == ButtonStyle::Framed);
        
        if let Some(tint) = self.default_button_style.tint {
            button = button.tint(tint);
        }
        ui.add(button)
    }
    fn explorer_taplate(&self, ui: &mut egui::Ui)->Response{
        let text = match &self.default_button_style.tooltip {
            Some(text) => text.as_str(),
            None => "No tooltip"
        };
        let (rect, response) = ui.allocate_at_least(self.default_button_style.size, egui::Sense::click());
        // 아이콘 그리기
        let icon_pos = egui::pos2(
            rect.center().x - (self.default_button_style.size.x * 0.5),
            rect.center().y - (self.default_button_style.size.y * 0.5),
        );
        ui.painter().image(
            self.texture.id(),
            egui::Rect::from_min_size(icon_pos, self.default_button_style.size),
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
            rect.center().y + (self.default_button_style.size.y * 0.6)
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
        if let Some(default_style) = &self.default_button_style.default_style {
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
        if let Some(hover_style) = &self.default_button_style.hover_style {
            //visuals.widgets.hovered.bg_fill = hover_style.background;
            visuals.widgets.hovered.bg_fill = hover_style.hover;
            visuals.widgets.hovered.fg_stroke = egui::Stroke::new(1.0, hover_style.text);//default_style
            visuals.widgets.hovered.bg_stroke = egui::Stroke::new(1.0, hover_style.border);
        }
        
        // 클릭 스타일이 제공된 경우 오버라이드
        if let Some(click_style) = &self.default_button_style.click_style {
            //visuals.widgets.active.bg_fill = click_style.background;
            visuals.widgets.active.bg_fill = click_style.accent;
            visuals.widgets.active.fg_stroke = egui::Stroke::new(1.0, click_style.text);
            visuals.widgets.active.bg_stroke = egui::Stroke::new(1.0, click_style.border);
        }
        
        let old_visuals = ui.style().visuals.clone();
        ui.style_mut().visuals = visuals.clone();
        
        let button = egui::Button::new({
        let text = match &self.default_button_style.tooltip {
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
            if self.default_button_style.selected {
                rich_text = rich_text.strong();
            }
            rich_text
        })
        .fill(if self.default_button_style.selected {
            self.default_button_style.default_style.map_or(egui::Color32::from_rgb(45, 55, 65), |s| s.selected)
        } else {
            self.default_button_style.default_style.map_or(egui::Color32::TRANSPARENT, |s| s.background)
        })
        .frame(true);
        
        let response = ui.add_sized([ui.available_width(), 32.0], button);
        
        // 아이콘 그리기
        let rect = response.rect;
        let icon_pos = egui::pos2(
            rect.min.x + 10.0,
            rect.center().y - self.default_button_style.size.y / 2.0
        );
        
        ui.painter().image(
            self.texture.id(),
            egui::Rect::from_min_size(icon_pos, self.default_button_style.size),
            egui::Rect::from_min_max(egui::pos2(0.0, 0.0), egui::pos2(1.0, 1.0)),
            self.default_button_style.tint.unwrap_or(egui::Color32::WHITE)
        );
        
        // 선택된 항목의 왼쪽에 표시기 추가
        if self.default_button_style.selected {
            let indicator_width = 3.0;
            let accent_color = self.default_button_style.default_style.map_or(
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
        let accent_color = self.default_button_style.default_style.map_or(
            egui::Color32::from_rgb(37, 99, 235), 
            |s| s.accent
        );
        
        let button = egui::Button::new({
            let text = format!("{:?}", self.default_button_style.icon);
            egui::RichText::new(text).strong()
        })
        .fill(accent_color);
        
        ui.add(button)
    } 
    fn secondary_taplate(&self, ui: &mut egui::Ui)->Response{
        // Secondary 버튼 스타일도 유사하게 처리
        let bg_color = self.default_button_style.default_style.map_or(
            egui::Color32::from_rgb(75, 85, 99), 
            |s| s.selected
        );
        
        let button = egui::Button::new(format!("{:?}", self.default_button_style.icon))
        .fill(bg_color);
        
        ui.add(button)
    }     
}