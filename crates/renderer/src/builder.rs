use winit::application::ApplicationHandler;
use std::any::Any;
use std::rc::Rc;
use std::marker::Copy;
use std::clone::Clone;
use crate::renderer::fill_window_with_color;
use winit::event::ElementState;
use winit::event_loop::ActiveEventLoop;
use winit::window::{Window, WindowAttributes, WindowId};
use winit::event::{WindowEvent, MouseButton};

// 1) 우리의 앱 상태
#[derive(Default)]
pub struct MainEventLoop {
    // 실제 Window 객체를 가지고 있을 필드
    window: Option<Window>,
}
// 2) ApplicationHandler 트레이트 구현
impl ApplicationHandler for MainEventLoop {
    // (필수) 앱이 'Resumed' 상태가 될 때 실행되는 콜백
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        // - 이 시점에서만 창을 생성하는 게 권장됨 (Android/iOS에서 Surface 생성 시점 등 고려)
        // - WindowAttributes를 커스텀해서 쓸 수도 있고, 그냥 디폴트로도 가능
        let attrs = WindowAttributes::default()
            .with_title("Hello Winit!")
            .with_inner_size(winit::dpi::PhysicalSize::new(800, 600))
            .with_visible(true);

        // event_loop.create_window(...) 사용
        let window = event_loop.create_window(attrs)
            .expect("Failed to create window");
            window.set_ime_allowed(true);
            println!("IME enabled?{:?}\n",window);
            window.set_ime_allowed(true);
            println!("IME enabled?{:?}\n",window);
        self.window = Some(window);
        println!("[resumed] Window created!");
    }

    // (필수) 특정 window에 발생하는 이벤트를 처리
    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        window_id: WindowId,
        event: WindowEvent,
    ) {
        // 만약 이 이벤트가 내 창에서 온 게 아니라면 무시
        if let Some(ref window) = self.window {
            if window.id() != window_id {
                return;
            }
        } else {
            // 아직 self.window가 없으면 스킵
            return;
        }
        // 이벤트 매칭
        match event {
            WindowEvent::CloseRequested => {
                println!("Close button pressed -> exit()");
                // 이벤트 루프 종료
                event_loop.exit();
            }
            WindowEvent::Resized(size) => {
                println!("Resized to: {:?}", size);
            }
            WindowEvent::RedrawRequested => {
                // [중요] 실제 렌더링 로직(예: OpenGL/Vulkan/DirectX 등)을 넣는 곳
                // 여기서는 단순히 "화면을 다시 그려야 함" 이라는 로그만 출력
               // println!("RedrawRequested -> Here you would render your stuff.");
               
                   match &self.window {
                    Some(window) => fill_window_with_color(window, 0xFFb8d8ef),
                    None => {
                        // 윈도우가 없을 때의 처리
                        println!("윈도우가 존재하지 않습니다!");
                    }
                }
               
            }
            WindowEvent::Ime (ime)=>{
                println!("IME event: {:?}\n",ime);
                println!("In to Ime loop");
            }
 
            WindowEvent::KeyboardInput { event, .. } => {
                println!("logical_key pressed: {:?}",event.logical_key);
                println!("physical_key pressed: {:?}",event.physical_key);
                println!("text pressed: {:?}",event.text);    
                println!("Key location: {:?}",event.location);
                println!("Key state: {:?}",event.state);  

            }
            WindowEvent::MouseInput { state, button, .. } => {
                match button {
                    MouseButton::Left => {
                        if state == ElementState::Pressed {
                            println!("🖱 왼쪽 버튼 클릭됨!");
                        }
                    }
                    MouseButton::Right => {
                        if state == ElementState::Pressed {
                            println!("🖱 오른쪽 버튼 클릭됨!");
                        }
                    }
                    _ => {}
                }
            }
            WindowEvent::CursorMoved { ..} => {
                //println!("🖱 마우스 이동: ({}, {})", position.x, position.y);
            }
            _ => {}
        }
    }
}








/// 공통 인터페이스 – 이벤트 수신 & 드로우
pub trait Block: Any + Send{
    fn on_event(&mut self, id: winit::window::WindowId, evt: &winit::event::WindowEvent);
    fn draw(&mut self, id: winit::window::WindowId);
}

/// Boxed 동적 디스패치 타입

pub type DynBlock = Rc<dyn Block>;

pub struct ApplicationHandlerBlock {

}

pub trait ApplicationHandlerBuilder {
    fn new() -> Self;
    /// 핸들러 요소 추가
    fn add(&mut self,block:ApplicationHandlerBlock)->Self;
    fn remove(&mut self,block:ApplicationHandlerBlock)->Self;
    /// get 핸들러 반환
    fn build(&mut self)->Box<dyn ApplicationHandler>;

}


#[derive(Clone)]
pub struct Delegator {
    blocks: Vec<DynBlock>,
}
#[derive(Clone)]
pub struct HandlerBuilder {
    blocks: Vec<DynBlock>,
}
impl ApplicationHandlerBuilder for HandlerBuilder {
    fn new() -> Self { Self { blocks: Vec::new() } }

    fn add(&mut self, block: ApplicationHandlerBlock) -> Self {
        // self.blocks.push(Box::new(block));
        self.clone()
    }

    fn remove(&mut self, block: ApplicationHandlerBlock) -> Self {
        // self.blocks.retain(|b| b.id() != block.id());
        self.clone()
    }

    fn build(&mut self) -> Box<dyn ApplicationHandler> {
        Box::new(MainEventLoop::default())
    }
}




