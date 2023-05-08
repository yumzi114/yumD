use super::MyInfo;
use eframe::{egui};
use egui::{RichText,Color32,collapsing_header::CollapsingState,InnerResponse,Ui,Response};
pub struct  MyApp{
    pub(crate) date: bool,
    pub(crate) has_next: bool,
    pub(crate) current_page: u32,
    pub(crate) page_line: u32,
}
impl MyApp{
    pub fn new() -> MyApp {
        MyApp { 
            date: false, 
            has_next: false, 
            current_page: 1, 
            page_line: 10 
        }
    }
    pub fn render_sys(&mut self, ui: &mut Ui){
        let my_system = MyInfo::MyInfo::new();
        ui.heading("Check System Files");
        
        ui.horizontal_wrapped(|ui|{
            for i in my_system.list{
                ui.label(i.menu().as_str());
                if i.used {
                    ui.label(RichText::new("Used").color(Color32::from_rgb(110, 255, 110)));
                    let btn = ui.small_button(RichText::new("📋").size(15.))
                            .on_hover_text("copy path");
                    if btn.clicked(){
                        ui.output_mut(|o| o.copied_text = i.path.into());
                    }
                    let winbtn = ui.small_button(RichText::new("📑").size(15.))
                            .on_hover_text("show code");
                    if winbtn.clicked(){
                        let temp = egui::Window::new("My Window").id(i.sysname.into()).open(&mut self.has_next);
                        // let temp = egui::Window::new("My Window").id("ttt".into()).open(&mut self.has_next);
                        temp.show(ui.ctx(), |ui| {
                            ui.label("Hello World!");
                        });
                    }
                }else {
                    ui.label(RichText::new("Undefined").color(Color32::from_rgb(244, 4, 4)));
                }
            }
        });
    }
    pub fn news_menu(&mut self, ui: &mut Ui){
        let mut news_view = collaps_head("news",ui);
        let news_header_res = collaps_head_respone(ui,&mut news_view,"show!");
        news_view.show_body_indented(&news_header_res.response, ui, |ui|{
            ui.horizontal_wrapped(|ui|{
                ui.checkbox(&mut self.date, "DATE");
                // #[cfg(feature = "chrono")]
                if self.date {
                    // let mut local: DateTime<Local> = Local::now();
                    ui.label("dd");
                    // ui.add(DatePicker::new("datetime",&mut local));
                };
                if ui.small_button(RichText::new("◀️").size(15.)).clicked(){
                    if self.current_page!=1{
                        self.current_page -=1;
                    };
                };
                ui.add(egui::DragValue::new(&mut self.current_page).speed(1.0));
                if ui.small_button(RichText::new("▶️").size(15.)).clicked(){
                    self.current_page +=1;
                };
                ui.label("page line : ");
                ui.add(egui::DragValue::new(&mut self.page_line));
                if ui.button(RichText::new("🔁").size(15.)).clicked(){};
            });
        });
    }
    pub fn db_menu(&mut self, ui: &mut Ui){
        let mut db_view = collaps_head("dbv",ui);
        let db_header_res = collaps_head_respone(ui,&mut db_view,"show!");
        db_view.show_body_indented(&db_header_res.response, ui, |ui| {
            ui.label("Body");
            let mut temp = "// A very simple example\n\
            fn main() {\n\
            \tprintln!(\"Hello world!\");\n\
            }\n\
            ";
            ui.add(
                egui::TextEdit::multiline(&mut temp).lock_focus(true)
                .desired_width(f32::INFINITY)
                .font(egui::TextStyle::Monospace)
                .desired_rows(10)
                .code_editor());
            ui.code("syntax_highlighting");
            // ui.text_edit_multiline(&mut temp);
            ui.label("Body");
        }
        );
    }
    pub fn stream_menu(&mut self, ui: &mut Ui){
        let mut stream_view = collaps_head("stream",ui);
        let stream_header_res = collaps_head_respone(ui,&mut stream_view,"show!");
        stream_view.show_body_indented(&stream_header_res.response, ui, |ui| 
            ui.label("Body")
        );
    }
    pub fn video_menu(&mut self, ui: &mut Ui){
        let mut video_down = collaps_head("downvideo",ui);
        let video_down_res = collaps_head_respone(ui,&mut video_down,"show!");
        video_down.show_body_indented(&video_down_res.response, ui, |ui| 
            ui.label("Body")
        );
    }
}





fn circle_icon(ui: &mut Ui, openness: f32, response: &Response) {
    let stroke = ui.style().interact(&response).fg_stroke;
    let radius = egui::lerp(6.0..=8.0, openness);
    ui.painter().circle_filled(response.rect.center(), radius, stroke.color);
}
fn collaps_head (id:&str, ui: &mut Ui)->CollapsingState{
    let head = CollapsingState::load_with_default_open(
        ui.ctx(),
        ui.make_persistent_id(id),
        false,
    );
    head
}
fn collaps_head_respone(ui: &mut egui::Ui,statename:&mut CollapsingState,lable:&str)->InnerResponse<()>{
    let respone =ui.horizontal(|ui| {
        ui.label(lable);
        statename.show_toggle_button(ui, circle_icon);
    });
    respone
}