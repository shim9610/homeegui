use std::fs;
use std::path::Path;
use winit::event_loop::ActiveEventLoop;
use winit::window::{Window, WindowAttributes, WindowId};
use std::cell::RefCell;
use std::collections::HashMap;
use softbuffer::{Context, Surface};
use std::mem;
use winit::application::ApplicationHandler;

use winit::event::{WindowEvent, MouseButton};
use std::time::Duration;
use std::thread::sleep;
use winit::event_loop::{ControlFlow, EventLoop};




thread_local! {
    static GC: RefCell<Option<GraphicsContext>> = RefCell::new(None);
}

struct GraphicsContext {
    /// The global softbuffer context.
    context: RefCell<Context<&'static Window>>,

    /// The hash map of window IDs to surfaces.
    surfaces: HashMap<WindowId, Surface<&'static Window, &'static Window>>,
}
impl GraphicsContext {
    fn new(w: &Window) -> Self {
        Self {
            context: RefCell::new(
                Context::new(unsafe {
                    mem::transmute::<& Window, &'static Window>(w)
                })
                .expect("Failed to create a softbuffer context"),
            ),
            surfaces: HashMap::new(),
        }
    }

    fn create_surface(
        &mut self,
        window: & Window,
    ) -> &mut Surface<&'static Window, &'static Window> {
        self.surfaces.entry(window.id()).or_insert_with(|| {
            Surface::new(&self.context.borrow(), unsafe {
                mem::transmute::<&'_  Window, &'static Window>(window)
            })
            .expect("Failed to create a softbuffer surface")
        })
    }

    fn _destroy_surface(&mut self, window: & Window) {
        self.surfaces.remove(&window.id());
    }
}


pub fn fill_window_with_color(window: &Window, color: u32) {
    GC.with(|gc| { let size = window.inner_size();
        let  (Some(width), Some(height)) =
            (std::num::NonZeroU32::new(size.width), std::num::NonZeroU32::new(size.height))
        else {
            return;
        };
        // GC: thread_local! 로 만든 GraphicsContext<Option>을 담는 전역(?) 변수라고 가정.
        //     이미 코드 상단 어딘가에서
        //       thread_local! {
        //           static GC: RefCell<Option<GraphicsContext>> = RefCell::new(None);
        //       }
        //     이런 식으로 선언했을 것입니다.
        let mut gc = gc.borrow_mut();

        // softbuffer Surface 생성
        let surface =
            gc.get_or_insert_with(|| GraphicsContext::new(window))
              .create_surface(window);

        // 창 크기에 맞춰 surface 사이즈 설정
        surface.resize(width, height)
               .expect("Failed to resize the softbuffer surface");

        // 실제 픽셀 데이터를 담을 수 있는 buffer 얻기
        let mut buffer = surface.buffer_mut()
                                .expect("Failed to get the softbuffer buffer");

        // buffer 전체를 지정한 color(u32)로 채우기
        buffer.fill(color);

        // 그린 결과를 화면에 표시
        buffer.present()
              .expect("Failed to present the softbuffer buffer");
    })
}