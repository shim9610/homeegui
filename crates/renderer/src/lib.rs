mod builder;
mod renderer;

use std::fs;
use std::path::Path;
use std::time::Duration;
use std::thread::sleep;
use winit::event_loop::{ControlFlow, EventLoop};






fn fix_vul() {
    let icd_path = "/usr/share/vulkan/icd.d";
    let layer_path = "/usr/share/vulkan/explicit_layer.d";

    if Path::new(icd_path).exists() {
        let nvidia_icd = Path::new(icd_path).join("nvidia_icd.json");
        if nvidia_icd.exists() {
            std::env::set_var("VK_ICD_FILENAMES", nvidia_icd.to_str().unwrap());
            println!("VK_ICD_FILENAMES 설정: {}", nvidia_icd.display());
        } else {
            // Fallback to another ICD file if NVIDIA is not found
            let available_files: Vec<_> = fs::read_dir(icd_path)
                .unwrap()
                .filter_map(|entry| entry.ok())
                .filter(|entry| entry.path().is_file())
                .collect();

            if !available_files.is_empty() {
                let fallback_icd = available_files[0].path();
                std::env::set_var("VK_ICD_FILENAMES", fallback_icd.to_str().unwrap());
                println!("Fallback VK_ICD_FILENAMES 설정: {}", fallback_icd.display());
            } else {
                println!("경고: Vulkan ICD 파일이 없습니다: {}", icd_path);
            }
        }
    } else {
        println!("경고: Vulkan ICD 경로가 없습니다: {}", icd_path);
    }

    if Path::new(layer_path).exists() {
        std::env::set_var("VK_LAYER_PATH", layer_path);
        println!("VK_LAYER_PATH 설정: {}", layer_path);
    } else {
        println!("경고: Vulkan Layer 경로가 없습니다: {}", layer_path);
    }
    std::env::set_var("WINIT_UNIX_BACKEND", "wayland");
}





pub fn run() {
    fix_vul();
    for (key, value) in std::env::vars() {
        println!("{}: {}", key, value);
    }
    println!("Environment variables are printed above.");
    
    // 1) EventLoop 생성
    let event_loop = EventLoop::new()
        .expect("Failed to create EventLoop");

    // 2) 컨트롤 플로우 모드 설정
    // - Poll: 매 프레임마다 이벤트를 체크하고 없으면 바로 반환(게임 등)  
    // - Wait: 이벤트가 없으면 대기(일반 GUI)  
    event_loop.set_control_flow(ControlFlow::Wait);

    // 3) 실제 App 인스턴스 생성
    let mut app = builder::MainEventLoop::default();

    // 4) 이벤트 루프 실행
    let _ =event_loop.run_app(&mut app);
    sleep(Duration::from_millis(16));
    // 여기까지. run_app(...)이 종료되는 순간 프로세스가 곧 끝납니다.
}