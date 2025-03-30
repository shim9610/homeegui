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