
# learning project Rust egui 파일 관리자 프로젝트 유틸리티 모듈 구조

## 1. 키보드/마우스 이벤트 관리 클래스

**목적**: 사용자 입력을 추상화하고 일관된 상호작용 시스템 제공

**주요 기능**:
- 키보드 단축키 등록 및 처리
- 마우스 이벤트 (클릭, 더블클릭, 드래그 등) 캡처 및 처리
- 포커스 관리 및 탭 순서 제어
- 접근성 지원 기능 (키보드 내비게이션)

**활용 예시**:
```
// 단축키 등록 예시
event_manager.register_shortcut(
    KeyCombination::new(Key::C).with_modifier(Modifier::CTRL),
    Action::Copy
);

// 이벤트 처리 예시
if event_manager.is_action_triggered(Action::Copy) {
    // 복사 기능 실행
}
```

## 2. UI 컴포넌트 라이브러리 클래스 icon_loader.rs

**목적**: 재사용 가능한 UI 요소 제공하여 일관된 디자인 시스템 구축

**주요 기능**:
- 아이콘 버튼 및 메뉴 항목 컴포넌트
- 열거형 기반 버튼 생성 시스템 (`IconButton::new(ButtonType::Back, ButtonStyle::Frame)`)
- 탭 및 내비게이션 컴포넌트
- 파일 목록 표시용 그리드 및 리스트 컴포넌트

**활용 예시**:
```
// 아이콘 버튼 생성 예시
if ui_components.icon_button(IconType::Back, ButtonStyle::Framed)
    .tooltip("뒤로 가기")
    .show(ui)
    .clicked() 
{
    // 뒤로가기 기능
}
```

## 3. 애니메이션 관리 클래스

**목적**: 부드러운 UI 전환 및 시각적 피드백 제공

**주요 기능**:
- 값 애니메이션 (크기, 위치, 투명도 등)
- 이징 함수 및 타이밍 제어
- 시퀀스 및 병렬 애니메이션
- 알림 및 토스트 메시지 표시 애니메이션

**활용 예시**:
```
// 크기 변화 애니메이션 예시
let size_animation = animation_manager.animate(
    &mut self.panel_size,
    Vec2::new(200.0, 300.0),
    Duration::from_secs_f32(0.3),
    EasingFunction::CubicOut
);

// 알림 표시 예시
animation_manager.show_notification(
    "파일이 성공적으로 복사되었습니다.",
    NotificationType::Success,
    Duration::from_secs(3)
);
```
스케줄러 추가?
DB sql 비스무리한거?

## 4. 영역 슬라이서 -> 대충 구현됨 탐색기나 일부 UI 전용 레이아웃을 따로 구현할 예정

**목적**: UI 레이아웃을 유연하게 분할하고 관리

**주요 기능**:
- 비율 기반 화면 분할
- 반응형 레이아웃 지원
- 중첩 가능한 레이아웃 구조
- 드래그로 크기 조절 가능한 분할 패널

**활용 예시**:
```
// 화면 분할 예시
let regions = area_slicer.slice(
    ui.max_rect(),
    SliceDirection::Horizontal,
    &[0.3, 0.7]  // 30%와 70%로 분할
);

ui.allocate_ui(regions[0], |ui| {
    // 왼쪽 30% 영역 UI 렌더링
});

ui.allocate_ui(regions[1], |ui| {
    // 오른쪽 70% 영역 UI 렌더링
});
```

## 5. 스타일 관리 (이미 구현됨)

**목적**: 일관된 시각적 테마 및 UI 스타일링 제공

**주요 기능**:
- 색상 팔레트 및 테마 관리
- 폰트 및 텍스트 스타일
- 위젯 시각적 속성 (테두리, 그림자, 라운딩 등)
- 다크/라이트 모드 지원

**활용 예시**:
```
// 스타일 적용 예시
let _ = &ctx.apply_style(&UiStyle::deep_navy(2));

// 특정 UI 요소에 스타일 적용
panel.apply_style(&UiStyle::dark_blue()).show(ctx, |ui| {
    // 패널 내용 렌더링
});
```

## 6. 글로벌 스택 관리자

**목적**: 내비게이션 및 상태 히스토리 관리

**주요 기능**:
- 페이지 네비게이션 히스토리 (뒤로가기/앞으로가기)
- 작업 실행/취소 (Undo/Redo) 지원
- 복합 작업 그룹화 및 관리
- 변경 사항 시각화 및 추적

**활용 예시**:
```
// 히스토리에 작업 추가 예시
global_stack.push_action(
    Action::MoveFile { 
        source: "folder1/file.txt", 
        destination: "folder2/file.txt" 
    }
);

// 뒤로가기 기능 예시
if global_stack.can_go_back() && back_button.clicked() {
    global_stack.go_back();
}
```

## 7. 대화상자/컨텍스트 메뉴 관리자

**목적**: 팝업 UI 요소의 통합 관리

**주요 기능**:
- 컨텍스트 메뉴 (우클릭 메뉴) 생성 및 표시
- 확인/취소 대화상자, 알림 대화상자 등 표준 템플릿
- 모달 대화상자 스택 관리
- 비동기 결과 처리 (콜백 시스템)

**활용 예시**:
```
// 컨텍스트 메뉴 표시 예시
if ui.response().secondary_clicked() {
    dialog_manager.show_context_menu(
        &["복사", "붙여넣기", "삭제", "이름 변경"],
        |selected| match selected {
            0 => copy_file(),
            1 => paste_file(),
            2 => delete_file(),
            3 => rename_file(),
            _ => {}
        }
    );
}

// 확인 대화상자 예시
dialog_manager.show_confirm_dialog(
    "파일 삭제",
    "정말로 이 파일을 삭제하시겠습니까?",
    |confirmed| {
        if confirmed {
            delete_file();
        }
    }
);
```
