use Rusty_egui::egui::Color32;
use Rusty_egui::egui;
use std::default::Default;



#[derive(Clone, Copy, Debug)]
pub struct UiStyle {
    pub background: Color32,     // 기본 배경색
    pub selected: Color32,       // 선택된 요소 배경색
    pub accent: Color32,         // 강조색 (버튼 등)
    pub text: Color32,           // 텍스트 색상
    pub border: Color32,         // 테두리 색상
    pub hover: Color32,          // 호버 효과 색상
}

impl UiStyle {
    // 이미지에 보이는 스타일의 프리셋
    pub fn deep_navy(textflag: i8) -> Self {
        if textflag == 1 {
            Self {
                background: Color32::from_rgb(15, 20, 28),   // 매우 어두운 네이비 배경
                selected: Color32::from_rgb(28, 36, 48),     // 선택 영역용 약간 밝은 색상
                accent: Color32::from_rgb(59, 110, 194),     // 파란색 강조
                text: Color32::WHITE,                        // 흰색 텍스트
                border: Color32::from_rgb(35, 40, 50),       // 테두리
                hover: Color32::from_rgb(40, 50, 65)         // 호버 효과
                
            }
        } else if textflag == 2 {
            Self {
                background: Color32::from_rgb(15, 20, 28),   // 매우 어두운 네이비 배경
                selected: Color32::from_rgb(28, 36, 48),     // 선택 영역용 약간 밝은 색상
                accent: Color32::from_rgb(59, 110, 194),     // 파란색 강조
                text: Color32::from_rgb(163, 184, 204),      // 파스텔 블루 텍스트
                border: Color32::from_rgb(35, 40, 50),       // 테두리
                hover: Color32::from_rgb(40, 50, 65)         // 호버 효과
            }
        } else {
            // 기본값 (textflag가 1이나 2가 아닌 경우)
            Self {
                background: Color32::from_rgb(15, 20, 28),
                selected: Color32::from_rgb(28, 36, 48),
                accent: Color32::from_rgb(59, 110, 194),
                text: Color32::WHITE,
                border: Color32::from_rgb(35, 40, 50),
                hover: Color32::from_rgb(40, 50, 65)
            }
        }
    }

    pub fn dark_blue() -> Self {
        Self {
            background: Color32::from_rgb(18, 22, 32),
            selected: Color32::from_rgb(25, 34, 43),
            accent: Color32::from_rgb(59, 110, 194),
            text: Color32::WHITE,
            border: Color32::WHITE,
            hover: Color32::from_rgb(45, 55, 70),
        }
    }
        // 밝은 파란색 테마
    pub fn bright_blue() -> Self {
        Self {
            background: Color32::from_rgb(59, 110, 194),
            selected: Color32::from_rgb(75, 130, 215),  // 조금 더 밝은 파란색
            accent: Color32::from_rgb(255, 255, 255),   // 흰색 강조
            text: Color32::WHITE,
            border: Color32::from_rgb(40, 80, 150),     // 어두운 파란색 테두리
            hover: Color32::from_rgb(80, 140, 230),     // 호버 시 더 밝은 파란색
        }
    }
    pub fn debug() -> Self {
        Self {
            background: Color32::from_rgb(0, 0, 0),
            selected: Color32::from_rgb(150, 150, 150),  // 조금 더 밝은 파란색
            accent: Color32::from_rgb(255, 255, 255),   // 흰색 강조
            text: Color32::WHITE,
            border: Color32::from_rgb(40, 40, 40),     // 어두운 파란색 테두리
            hover: Color32::from_rgb(150, 150, 150),     // 호버 시 더 밝은 파란색
        }
    }
}
pub trait ContextStyle {
    fn apply_style(&self, style: &UiStyle);
}

impl ContextStyle for Rusty_egui::egui::Context {
    fn apply_style(&self, style: &UiStyle)   {
        let _ = &self.style_mut(|ctx_style| {
            ctx_style.visuals.panel_fill = style.background;
            ctx_style.visuals.window_fill = style.background;
            ctx_style.visuals.faint_bg_color = style.selected;
            ctx_style.visuals.widgets.active.bg_fill = style.accent;
            ctx_style.visuals.widgets.hovered.bg_fill = style.hover;
            ctx_style.visuals.override_text_color = Some(style.text);
            ctx_style.visuals.window_stroke = Rusty_egui::egui::Stroke::new(1.0, style.border);
        });

    }
}

pub trait WidgetStyle {
    fn apply_style(self, style: &UiStyle) -> Self;
}

// egui 라이브러리의 타입에 우리 트레이트를 구현
impl WidgetStyle for Rusty_egui::egui::TopBottomPanel {
    fn apply_style(self, style: &UiStyle) -> Self  {
        // 기존 타입을 확장하는 구현
        self.frame(Rusty_egui::egui::Frame::default()
            .fill(style.background)
            .stroke(Rusty_egui::egui::Stroke::new(1.0, style.border)))
        
    }
}
impl <'a> WidgetStyle for Rusty_egui::egui::Button<'a> {
    fn apply_style(self, style: &UiStyle) -> Self{
        // 기존 타입을 확장하는 구현
        self.fill(style.background)
            .stroke(Rusty_egui::egui::Stroke::new(1.0, style.border))
        
    }
}
impl WidgetStyle for Rusty_egui::egui::CentralPanel {
    fn apply_style(self, style: &UiStyle) -> Self  {
        // 기존 타입을 확장하는 구현
        self.frame(Rusty_egui::egui::Frame::default()
            .fill(style.background)
            .stroke(Rusty_egui::egui::Stroke::new(1.0, style.border)))
        
    }
}


/// 빈 UI 영역을 기본 배경으로 렌더링하는 구조체
pub struct EmptyRenderer {
    style: UiStyle,
    message: Option<String>,
}

impl EmptyRenderer {
    pub fn new(style: UiStyle) -> Self {
        Self {
            style,
            message: None,
        }
    }
    
    pub fn with_message(mut self, message: impl Into<String>) -> Self {
        self.message = Some(message.into());
        self
    }
    
    /// UI 영역에 기본 배경과 선택적 메시지를 렌더링
    pub fn render(&self, ui: &mut egui::Ui) {
        // 배경 그리기
        let rect = ui.max_rect();
        ui.painter().rect_filled(
            rect,
            0.0,  // 라운딩 없음
            self.style.background,
        );
        
        // 메시지가 있으면 표시
        if let Some(msg) = &self.message {
            ui.vertical_centered(|ui| {
                ui.add_space(rect.height() * 0.4);  // 수직 중앙 근처에 배치
                ui.label(egui::RichText::new(msg)
                    .color(self.style.text)
                    .size(16.0));
            });
        }
    }
}