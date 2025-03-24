use Rusty_egui::egui::{self, Rect, Ui};
use std::collections::HashMap;
use std::vec::Vec;
use std::iter::IntoIterator;
use Rusty_egui::egui::Context;
use crate::Frontend::Utility::icon_loader::{ExplorerIcon,IconButton};

// 슬라이싱 방향 정의
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SliceDirection {
    Horizontal, // 수평 방향으로 자름 (위/아래 분할)
    Vertical,   // 수직 방향으로 자름 (좌/우 분할)
}
// 영역 구조체 정의
#[derive(Debug, Clone)]
pub struct AreaData {
    pub id: usize,
    pub rect: Rect,
}
pub struct Area<'a> {
    pub data: AreaData,
    pub render_fn: Option<Box<dyn FnMut(&mut Ui)+'a>>,
}

// 슬라이서 트레잇 정의


// 영역 슬라이서 구조체
pub struct DefaultAreaSlicer <'a>{
    next_id: usize,
    areas: HashMap<usize, Area<'a>>,
}

impl <'a>DefaultAreaSlicer<'a> {
    // 새 슬라이서 생성
    pub fn new() -> Self {
        Self {
            next_id: 0,
            areas: HashMap::new(),
        }
    }
    
    // 초기 루트 영역 생성
    pub fn initialize(&mut self, root_rect: Rect) -> usize {
        let id = self.next_id;
        self.next_id += 1;
        
        let area = Area {
            data: AreaData {
                id,
                rect: root_rect,
            },
            render_fn: None,
        };
        
        self.areas.insert(id, area);
        id
    }
    
    // 영역을 특정 방향으로 비율에 따라 분할
    pub fn slice(&mut self, area_id: usize, direction: SliceDirection, ratios: &[f32]) -> Vec<usize> {
        if let Some(area) = self.areas.get(&area_id) {
            let parent_rect = area.data.rect;
            let mut result = Vec::new();
            let mut start_pos;
            
            // 분할 방향에 따라 처리
            match direction {
                SliceDirection::Horizontal => {
                    // 높이를 비율에 따라 분할
                    let total_height = parent_rect.height();
                    start_pos = parent_rect.min.y;
                    
                    for ratio in ratios {
                        let height = total_height * ratio;
                        let slice_rect = Rect::from_min_size(
                            egui::pos2(parent_rect.min.x, start_pos),
                            egui::vec2(parent_rect.width(), height)
                        );
                        
                        let id = self.next_id;
                        self.next_id += 1;
                        
                        let area = Area {
                            data: AreaData {
                                id,
                                rect: slice_rect,
                            },
                            render_fn: None,
                        };
                        
                        self.areas.insert(id, area);
                        result.push(id);
                        
                        start_pos += height;
                    }
                },
                SliceDirection::Vertical => {
                    // 너비를 비율에 따라 분할
                    let total_width = parent_rect.width();
                    start_pos = parent_rect.min.x;
                    
                    for ratio in ratios {
                        let width = total_width * ratio;
                        let slice_rect = Rect::from_min_size(
                            egui::pos2(start_pos, parent_rect.min.y),
                            egui::vec2(width, parent_rect.height())
                        );
                        
                        let id = self.next_id;
                        self.next_id += 1;
                        
                        let area = Area {
                            data: AreaData {
                                id,
                                rect: slice_rect,
                            },
                            render_fn: None,
                        };
                        
                        self.areas.insert(id, area);
                        result.push(id);
                        
                        start_pos += width;
                    }
                },
            }
            
            return result;
        }
        
        Vec::new() // 해당 ID의 영역이 없으면 빈 벡터 반환
    }
    
    // 영역을 균등하게 분할
    pub fn split_evenly(&mut self, area_id: usize, direction: SliceDirection, count: usize) -> Vec<usize> {
        if count == 0 {
            return Vec::new();
        }
        
        let ratio = 1.0 / (count as f32);
        let ratios = vec![ratio; count];
        
        self.slice(area_id, direction, &ratios)
    }
    
    // 그리드 형태로 분할
    pub fn grid(&mut self, area_id: usize, rows: usize, cols: usize) -> Vec<Vec<usize>> {
        if let Some(area) = self.areas.get(&area_id) {
            let parent_rect = area.data.rect;
            let mut result = Vec::with_capacity(rows);
            
            // 먼저 행으로 분할
            let row_ratio = 1.0 / (rows as f32);
            let row_ratios = vec![row_ratio; rows];
            let row_ids = self.slice(area_id, SliceDirection::Horizontal, &row_ratios);
            
            // 각 행을 열로 분할
            for row_id in row_ids {
                let col_ratio = 1.0 / (cols as f32);
                let col_ratios = vec![col_ratio; cols];
                let col_ids = self.slice(row_id, SliceDirection::Vertical, &col_ratios);
                result.push(col_ids);
            }
            
            return result;
        }
        
        Vec::new() // 해당 ID의 영역이 없으면 빈 벡터 반환
    }
    
    // 렌더링 함수 설정
    pub fn set_render_fn<F>(&mut self, area_id: usize, render_fn: F) -> bool
    where
        F: FnMut(&mut Ui) + 'a,
    {
        if let Some(area) = self.areas.get_mut(&area_id) {
            area.render_fn = Some(Box::new(render_fn));
            return true;
        }
        false
    }
    
    // 모든 영역 렌더링
    pub fn render_all(&mut self, ui: &mut Ui) {
        // ID 목록을 먼저 수집 (borrow checker 이슈 방지)
        let ids: Vec<usize> = self.areas.keys().cloned().collect();
        
        for id in ids {
            if let Some(area) = self.areas.get_mut(&id) {
                if let Some(render_fn) = &mut area.render_fn {
                    let area_rect = area.data.rect;
                    ui.allocate_ui_at_rect(area_rect, |ui| {
                        render_fn(ui);
                    });
                }
            }
        }
    }
    
    // 크기 업데이트 (윈도우 크기 변경 시 호출)
    pub fn update_size(&mut self, new_root_rect: Rect) {
        // 여기서는 간단하게 루트만 업데이트
        // 실제로는 비율을 유지하면서 모든 영역을 재계산해야 함
        if let Some(root_area) = self.areas.get_mut(&0) {
            root_area.data.rect = new_root_rect;
        }
    }
    
    // 영역 ID로 Rect 가져오기
    pub fn get_rect(&self, area_id: usize) -> Option<Rect> {
        self.areas.get(&area_id).map(|area| area.data.rect)
    }
}

pub trait AreaSlicer<'a> {
    // 슬라이싱 작업 수행
    fn slice(&mut self, area_id: usize, direction: SliceDirection, ratios: &[f32]) -> Vec<usize>;
    
    // 균등 분할
    fn split_evenly(&mut self, area_id: usize, direction: SliceDirection, count: usize) -> Vec<usize>;
    
    // 그리드 형태로 분할
    fn grid(&mut self, area_id: usize, rows: usize, cols: usize) -> Vec<Vec<usize>>;
    
    // 렌더링 함수 설정
    fn set_render_fn<F>(&mut self, area_id: usize, render_fn: F) -> bool
    where
        F: FnMut(&mut Ui) + 'a;
    
    // 모든 영역 렌더링
    fn render_all(&mut self, ui: &mut Ui);
    
    // 영역 존재 여부 확인
    fn has_area(&self, area_id: usize) -> bool;
    
    // 영역 정보 가져오기
    fn get_area_data(&self, area_id: usize) -> Option<&AreaData>;
}
impl<'a> AreaSlicer<'a> for DefaultAreaSlicer<'a>{
    fn slice(&mut self, area_id: usize, direction: SliceDirection, ratios: &[f32]) -> Vec<usize> {
        DefaultAreaSlicer::slice(self, area_id, direction, ratios)
    }
    fn split_evenly(&mut self, area_id: usize, direction: SliceDirection, count: usize) -> Vec<usize> {
        DefaultAreaSlicer::split_evenly(self, area_id, direction, count)
    }
    fn grid(&mut self, area_id: usize, rows: usize, cols: usize) -> Vec<Vec<usize>> {
        DefaultAreaSlicer::grid(self, area_id, rows, cols)
    }
    fn set_render_fn<F>(&mut self, area_id: usize, render_fn: F) -> bool
    where
     F: FnMut(&mut Ui)+'a ,
    {
        DefaultAreaSlicer::set_render_fn(self, area_id, render_fn)
    }
    fn render_all(&mut self, ui: &mut Ui) {
        DefaultAreaSlicer::render_all(self, ui)
    }
    fn has_area(&self, area_id: usize) -> bool {
        self.areas.contains_key(&area_id)
    }
    fn get_area_data(&self, area_id: usize) -> Option<&AreaData> {
        self.areas.get(&area_id).map(|area| &area.data)
    }

}
#[derive(Clone)]
struct XYindex{
    xindex:usize,
    yindex:usize,
}
struct Indexer{
    len:usize,
    xyindex:XYindex,
    width:usize,
    height:usize,
}
impl Indexer{
    fn new(len:usize,width:usize,height:usize)->Self{
        Self{
            len,
            xyindex:XYindex{xindex:0,yindex:0,},
            width,
            height,
        }
    }
    fn set_index(&mut self,index:usize)->XYindex{
        self.len=index;
        let xindex = index % self.width;
        let yindex = index / self.width;
        self.xyindex = XYindex{xindex,yindex}.clone();
        XYindex{xindex,yindex}
    }
}
// 파일 전용 슬라이서 구조체
pub struct FileSlicer <'a>{
    defaultareaslicer: DefaultAreaSlicer<'a>,
    margin: f32,
    file_height: f32,
    file_width: f32,
    distance: f32,
    width: f32,
    height: f32,
    rows: usize,
    cols: usize,
    maxnum:usize,
    addnum:usize,
    index:Indexer,
    root_id:usize,
    grid: Vec<Vec<usize>>,
    name : String,
    
}

#[derive(Clone)]
pub struct FileVec{
    files:Vec<ExplorerIcon>
}
impl FileVec{
        pub fn new (ctx:&mut Rusty_egui::egui::Context,text:String)->Self
    {
        let mut ndata=Vec::new();
        let in_data= ExplorerIcon::new(ctx,text);
        ndata.push(in_data);

        Self{
            files:ndata,

        }
    }
    pub fn add (&mut self,ctx:&mut Rusty_egui::egui::Context,text:String)
    {
        let in_data= ExplorerIcon::new(ctx,text);
        self.files.push(in_data);
    }
}

impl <'a> FileSlicer<'a>{
    pub fn new(margin: f32,file_width:f32 ,file_height: f32,distance:f32,root_rect: Rect  ) -> Self {
        let mut data=DefaultAreaSlicer::new();
        let root_id=data.initialize(root_rect);
        let width=root_rect.width();
        let height=root_rect.height();
        let rows:usize =0;
        let cols:usize =0;
        let maxnum:usize=0;
        let addnum:usize=0;
        let index=Indexer::new(0,width as usize,height as usize);
        let grid: Vec<Vec<usize>>=Vec::new();
        let name="임시 이름 입니다.".to_string();
        Self {
            defaultareaslicer: data,
            margin,
            distance,
            file_width,
            file_height,
            width,
            height,
            rows,
            cols,
            maxnum,
            addnum,
            index,
            root_id,
            grid,
            name,
        }
    }
    pub fn set_number_of_grid(&mut self){
        self.cols =((self.width-self.distance-self.margin*2.0)/(self.file_width+self.distance)).round() as usize;
        self.rows =((self.height-self.distance-self.margin*2.0)/(self.file_height+self.distance)).round()as usize;
        self.maxnum=self.cols*self.rows;
        let rario_columns=self.margin/self.width ;
        let input_ratio:[f32;3]=[rario_columns,(1.0-rario_columns*1.2),rario_columns*0.2];
        let columns = self.defaultareaslicer.slice(self.root_id,SliceDirection::Vertical,&input_ratio);
        let rario_row=self.margin/self.height ;
        let input_ratio_r:[f32;3]=[rario_row,(1.0-rario_row*1.2),rario_row*0.2];
        let row = self.defaultareaslicer.slice(columns[1],SliceDirection::Vertical,&input_ratio_r);
        self.root_id=row[1];
    }
    pub fn get_grid(&mut self){
        self.grid=self.defaultareaslicer.grid(self.root_id,self.rows,self.cols);
    }
    pub fn add_file_vec(&mut self,ui: &mut egui::Ui,vecdata:FileVec)
    {
        let copy_vecdata=vecdata.clone();
        self.get_grid();
        if self.grid.is_empty() || self.addnum >= self.maxnum {
            println!("Grid is empty or maximum items reached.");
            return;
        }
        self.index.height=self.grid.len();
        self.index.width=self.grid[0].len();
        let id = self.index.set_index(self.addnum);
            if id.yindex >= self.grid.len() || id.xindex >= self.grid[0].len() {
                println!("Index out of bounds: yindex={}, xindex={}, grid={}x{}", 
                id.yindex, id.xindex, self.grid.len(), 
                if self.grid.is_empty() { 0 } else { self.grid[0].len() });
            return;
        }

        for i in 0.. vecdata.files.len(){
            let icon_file=copy_vecdata.files[i].file.clone();
            self.index.height=self.grid.len();
            self.index.width=self.grid[0].len();
            let id = self.index.set_index(self.addnum);
            // 범위 검사 추가
            if id.yindex >= self.grid.len() || id.xindex >= self.grid[0].len() {
                println!("Index out of bounds: yindex={}, xindex={}, grid={}x{}", 
                         id.yindex, id.xindex, self.grid.len(), 
                         if self.grid.is_empty() { 0 } else { self.grid[0].len() });
                return;
            }
            let values = self.grid[id.yindex][id.xindex];
            self.defaultareaslicer.set_render_fn(values, move |ui| {
                 icon_file.clone().show(ui);
            });
            self.addnum += 1;
        }
    self.defaultareaslicer.render_all(ui);
    }

}
