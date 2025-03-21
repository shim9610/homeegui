use Rusty_egui::egui::PointerState;
use Rusty_egui::egui;
use Rusty_egui::egui::Modifiers;

use Rusty_egui::egui::Pos2;

#[derive(Clone, Debug)]
pub struct DragAndDrop{
    pos:Pos2,
    state:bool
}
//latest_pos, press_origin

pub struct PointScanner{
    pointstats: Vec<PointerState>,
    dragging_flag: bool,
    dragndrop:bool,
    flag_count: usize,
    drag:Vec<Pos2>,
    output:DragAndDrop

}
impl PointScanner{
    pub fn new()->Self{
        Self{
        pointstats:Vec::new(),
        flag_count:0,
        dragging_flag:false,
        dragndrop:false,
        drag:Vec::new(),
        output:DragAndDrop{
            pos:Pos2::new(0.0,0.0),
            state:false
        }
        }
    }
    pub fn check(&mut self, ctx: &egui::Context )-> Option<DragAndDrop>{
        let pointer = ctx.input(|i| i.pointer.clone());
        if self.dragndrop{
            if pointer.any_released(){
                self.dragndrop=false;
            }
            if let Some(indata)= pointer.interact_pos(){
                let data=self.info(indata);
                return Some(data)
            }   
            
        }else if pointer.is_decidedly_dragging() || self.dragging_flag{
            if !self.dragging_flag{
                self.dragging_flag=true;
                self.flag_count=0
            }
            if pointer.any_down(){
                self.dragging_flag=false;
                self.flag_count=0;
                self.dragndrop=true;
            }
            self.flag_count+=1;
            if self.flag_count==3{
                self.dragging_flag=false;
                self.flag_count=0
            }
            
        }
        return None
    }
    fn info(&mut self,inpos:Pos2)->DragAndDrop{
        self.output=DragAndDrop{
            pos:inpos,
            state:self.dragndrop
        };
        self.output.clone()
        }
    }
