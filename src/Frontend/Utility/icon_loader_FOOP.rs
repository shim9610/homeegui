use Rusty_egui::egui::{self, Response};
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
static defualt_style=ConstDummy::new();
static default_button=StyleTemplate::new(defualt_style);
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
trait IconButtonStyle{
    fn new()->Self where Self : Sized;
    fn default_style_get(&mut self)->&mut Option<UiStyle>;
    fn hover_style_get(&mut self)->&mut Option<UiStyle>;
    fn click_style_get(&mut self)->&mut Option<UiStyle>;
    fn selected_style_get(&mut self)->&mut Option<UiStyle>;
    fn drag_style_get(&mut self)->&mut Option<UiStyle>;
    fn style_get(&mut self)->(&mut Option<UiStyle>,&mut Option<UiStyle>,&mut Option<UiStyle>,&mut Option<UiStyle>,&mut Option<UiStyle>);
    //fn style_template_get(&mut self)->&mut StyleTemplate;
    //fn set_style(&mut self) -> Self{
    //    let calculator=self.style_template_get(&mut self);
    //    calculator.set_style(&mut self)
    //}

}
struct StyleTemplate<'a>{
    default_style: UiStyle,
    hover_style: UiStyle,
    click_style: UiStyle,
    selected_style: UiStyle,
    drag_style : UiStyle,
    setter_default_style: &'a mut Option<UiStyle>,
    setter_hover_style: &'a mut Option<UiStyle>,
    setter_click_style: &'a mut Option<UiStyle>,
    setter_selected_style: &'a mut Option<UiStyle>,
    setter_drag_style : &'a mut Option<UiStyle>,
}
impl<'a>StyleTemplate<'a>{
    fn new(selfo: &'a mut dyn IconButtonStyle)-> Self{


        let (A,B,C,D,E)=selfo.style_get();
        Self{
        default_style:UiStyle::deep_navy(1),
        hover_style: UiStyle::deep_navy(1),
        click_style: UiStyle::deep_navy(1),
        selected_style:UiStyle::bright_blue(),
        drag_style : UiStyle::deep_navy(1),
        setter_default_style: A,
        setter_hover_style: B,
        setter_click_style: C,
        setter_selected_style: D,
        setter_drag_style : E,
        }
    }
    fn set_style(&mut self){
        if let  Some(dafault)=self.setter_default_style.clone(){
            self.default_style=dafault.clone();
        }else{
            *self.setter_default_style=Some(self.default_style.clone());
        }
        if let  Some(hover)=self.setter_hover_style.clone(){
            self.hover_style=hover.clone();
        }else{
            *self.setter_hover_style=Some(self.hover_style.clone());
        }
        if let  Some(click)=self.setter_click_style.clone(){
            self.click_style=click.clone();
        }else{
            *self.setter_click_style=Some(self.click_style.clone());
        }
        if let  Some(selected)=self.setter_selected_style.clone(){
            self.selected_style=selected.clone();
        }else{
            *self.setter_selected_style=Some(self.selected_style.clone());
        }
        if let  Some(drag)=self.setter_drag_style.clone(){
            self.drag_style=drag.clone();
        }else{
            *self.setter_drag_style=Some(self.drag_style.clone());
        }
    }
}

struct ConstDummy{
    default_style: Option<UiStyle>,
    hover_style: Option<UiStyle>,
    click_style: Option<UiStyle>,
    selected_style: Option<UiStyle>,
    drag_style : Option<UiStyle>,
}
impl IconButtonStyle for ConstDummy{
    fn new()->Self{
        Self{
        default_style:Some(UiStyle::deep_navy(1)),
        hover_style:Some( UiStyle::deep_navy(1)),
        click_style:Some( UiStyle::deep_navy(1)),
        selected_style:Some(UiStyle::bright_blue()),
        drag_style :Some( UiStyle::deep_navy(1)),}
    }

    fn default_style_get(&mut self)->&mut Option<UiStyle>{
        return &mut self.default_style
    }
    
    fn hover_style_get(&mut self)->&mut Option<UiStyle>{
        return &mut self.hover_style
    }
    fn click_style_get(&mut self)->&mut Option<UiStyle>{
        return &mut self.click_style
    }
    fn selected_style_get(&mut self)->&mut Option<UiStyle>{
        return &mut self.selected_style
    }
    fn drag_style_get(&mut self)->&mut Option<UiStyle>{
        return &mut self.drag_style
    }
    fn style_get(&mut self)->(&mut Option<UiStyle>,&mut Option<UiStyle>,&mut Option<UiStyle>,&mut Option<UiStyle>,&mut Option<UiStyle>)
    {
       
       return (&mut self.default_style, &mut self.hover_style, &mut self.click_style, &mut self.selected_style, &mut self.drag_style)

    }

}

trait RanderTemplateWrapper{
    fn new()->Self where Self: Sized;
    fn default_style(self) -> Self where Self: Sized;

}


trait RanderTemplate{
    fn new(size: Vec2) ->Self where Self: Sized;
    fn default_style<T: RanderTemplateWrapper + 'static>(mut self_w : T, style: UiStyle) -> T {self_w}
    fn hover_style<T: RanderTemplateWrapper + 'static>(mut self_w : T, style: UiStyle) -> T {self_w}
    fn click_style<T: RanderTemplateWrapper + 'static>(mut self_w : T, style: UiStyle) -> T {self_w}
    fn tooltip<T: RanderTemplateWrapper + 'static>(mut self_w : T, style: UiStyle) -> T {self_w}
    fn texture<T: RanderTemplateWrapper + 'static>(mut self_w : T, style: UiStyle) -> T {self_w}
    fn render_explorer<T: RanderTemplateWrapper + 'static>(mut self_w : T,  ui: &mut Ui) -> Response {ui.button("")}
    
 
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



use egui::{Color32,  Stroke, TextureHandle, Ui, Vec2};

/// 예시: 버튼 스타일
#[derive(Debug, Clone)]
pub struct ButtonStruct {
    pub background: Color32,
    pub text: Color32,
    pub border: Color32,
    pub hover: Color32,
    pub accent: Color32,
}


/// 체이닝 + Explorer 템플릿 구조체(실제 그리는 로직은 없음)
#[derive( Clone)]
pub struct ButtonTemplate {
    // 세 가지 스타일
    pub default_style: Option<UiStyle>,
    pub hover_style: Option<UiStyle>,
    pub click_style: Option<UiStyle>,

    // 예: Explorer에서 쓸만한 필드들
    pub tooltip: Option<String>,
    pub texture: Option<TextureHandle>,
    pub size: Vec2,
}

impl RanderTemplate for ButtonTemplate {
    //---------------------------------------------------
    // 체이닝용 초기 생성자 & 스타일 설정자
    //---------------------------------------------------
    fn new(size: Vec2) -> Self {
        Self {
            default_style: None,
            hover_style: None,
            click_style: None,
            tooltip: None,
            texture: None,
            size,
        }
    }

    fn default_style<T: RanderTemplateWrapper + 'static>(mut self_w : T, style: UiStyle) -> T 
    {
        self_w.default_style = Some(style);
        self_w
    }

    fn hover_style(&self_w:dyn RanderTemplateWrapper, style: UiStyle) -> Self {
        self_w.hover_style = Some(style);
        self_w
    }

    fn click_style(&self_w:dyn RanderTemplateWrapper, style: UiStyle) -> Self {
        self_w.click_style = Some(style);
        self_w
    }

    fn tooltip(&self_w:dyn RanderTemplateWrapper, tooltip: impl Into<String>) -> Self {
        self_w.tooltip = Some(tooltip.into());
        self_w
    }

    fn texture(&self_w:dyn RanderTemplateWrapper, texture: TextureHandle) -> Self {
        self_w.texture = Some(texture);
        self_w
    }

    //---------------------------------------------------
    // Explorer 템플릿 (실제 렌더 함수: 내부 로직은 비워 둠)
    //---------------------------------------------------
    fn render_explorer(&self_w:RanderTemplateWrapper, ui: &mut Ui) -> Response {
        // 1) 기존 스타일 백업
        let old_visuals = ui.style().visuals.clone();
        let mut visuals = old_visuals.clone();

        // 2) default 스타일 적용
        if let Some(ref default) = self_w.default_style {
            visuals.widgets.inactive.bg_fill   = default.background;
            visuals.widgets.inactive.fg_stroke = Stroke::new(1.0, default.text);
            visuals.widgets.inactive.bg_stroke = Stroke::new(1.0, default.border);

            visuals.widgets.hovered.bg_fill    = default.hover;
            visuals.widgets.hovered.fg_stroke  = Stroke::new(1.0, default.text);
            visuals.widgets.hovered.bg_stroke  = Stroke::new(1.0, default.border);

            visuals.widgets.active.bg_fill     = default.accent;
            visuals.widgets.active.fg_stroke   = Stroke::new(1.0, default.text);
            visuals.widgets.active.bg_stroke   = Stroke::new(1.0, default.border);
        }

        // 3) hover_style 오버라이드
        if let Some(ref hover) = self_w.hover_style {
            visuals.widgets.hovered.bg_fill    = hover.hover;
            visuals.widgets.hovered.fg_stroke  = Stroke::new(1.0, hover.text);
            visuals.widgets.hovered.bg_stroke  = Stroke::new(1.0, hover.border);
        }

        // 4) click_style 오버라이드
        if let Some(ref click) = self_w.click_style {
            visuals.widgets.active.bg_fill     = click.accent;
            visuals.widgets.active.fg_stroke   = Stroke::new(1.0, click.text);
            visuals.widgets.active.bg_stroke   = Stroke::new(1.0, click.border);
        }

        // 5) 새로운 Visuals 적용
        ui.style_mut().visuals = visuals;

        // ------------------------------------------------------
        // 여기서부터 실제 그리는 로직(아이콘, 텍스트, etc.)은 비워 둠
        // 예: allocate_at_least(), painter().image(), painter().layout() 등
        // ------------------------------------------------------
        // ┌─────────────────────────────────────────────────────┐
        // │   // 실제 그리는 로직을 넣고 싶으면 여기 작성     │
        // └─────────────────────────────────────────────────────┘

        // 임시로 기본 반응(Response)만 만든다고 가정
        let response = ui.button("Temp Explorer Button");

        // 6) 스타일 복원
        ui.style_mut().visuals = old_visuals;

        // 7) Response 반환
        response
    }
}

pub struct MenuButton{
    tamp:ButtonTemplate,
    pub default_style: Option<UiStyle>,
    pub hover_style: Option<UiStyle>,
    pub click_style: Option<UiStyle>,
    pub tooltip: Option<String>,
    pub texture: Option<TextureHandle>,
    pub size: Vec2,
    pub visuals:Visuals,
}

impl RanderTemplateWrapper for MenuButton{
    fn new()->Self{
        let buf=ButtonTemplate::new(egui::vec2(64.0, 64.0));
        Self{
            tamp:ButtonTemplate::new(egui::vec2(64.0, 64.0)),
            default_style:buf.default_style,
            hover_style : buf.hover_style,
            click_style: buf.click_style,
            tooltip:buf.tooltip,
            texture:buf.texture,
            size:buf.size,
            visuals:Visuals::default(),
        }
    }
    fn default_style(mut self) -> Self {
        if let Some(nstyle)= self.default_style{
            
            self.visuals.widgets.inactive.bg_fill = nstyle.background;
            self.visuals.widgets.inactive.fg_stroke = egui::Stroke::new(1.0, nstyle.text);
            self.visuals.widgets.inactive.bg_stroke = egui::Stroke::new(1.0, nstyle.border);
            
            self.visuals.widgets.hovered.bg_fill = nstyle.hover;
            self.visuals.widgets.hovered.fg_stroke = egui::Stroke::new(1.0, nstyle.text);
            self.visuals.widgets.hovered.bg_stroke = egui::Stroke::new(1.0, nstyle.border);
          
            self.visuals.widgets.active.bg_fill = nstyle.accent;
            self.visuals.widgets.active.fg_stroke = egui::Stroke::new(1.0,nstyle.text);
            self.visuals.widgets.active.bg_stroke = egui::Stroke::new(1.0, nstyle.border);
            let style=nstyle;
        
        self.default_style = Some(style,);
        };
        self
    
    }
        
    

}
impl MenuButton{
    pub fn hover_style(mut self) -> Self {
        if let Some(nstyle)=self.hover_style {
           self.visuals.widgets.hovered.bg_fill = nstyle.background;
           self.visuals.widgets.hovered.bg_fill = nstyle.hover;
           self.visuals.widgets.hovered.fg_stroke = egui::Stroke::new(1.0, nstyle.text);//default_style
           self.visuals.widgets.hovered.bg_stroke = egui::Stroke::new(1.0, nstyle.border);
           let style = nstyle;
           self.hover_style = Some(style);
        };

        self
    }

    pub fn click_style(mut self) -> Self {
        let style={
            //visuals.widgets.active.bg_fill = click_style.background;
          //  visuals.widgets.active.bg_fill = click_style.accent;
          //  visuals.widgets.active.fg_stroke = egui::Stroke::new(1.0, click_style.text);
          //  visuals.widgets.active.bg_stroke = egui::Stroke::new(1.0, click_style.border);
        };
        self.click_style = Some(style);
        self
    }

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
}

