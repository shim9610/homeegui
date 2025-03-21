// src/main.rs

mod Frontend;  // 폴더 src/frontend/ => mod.rs
mod Backend;   // 폴더 src/backend/ => mod.rs

fn main() {
    println!("Hello from main!");

    // 혹시 프론트엔드 관련 함수를 호출하고 싶다면,
    Frontend::app::run_ui();

    // 백엔드에도 적당히 함수를 만든 뒤 호출할 수 있음
    // backend::something_init();
}
