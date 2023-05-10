use super::MyInfo;
use eframe::{egui};
use egui::{RichText,Color32,collapsing_header::CollapsingState,InnerResponse,Ui,Response,ScrollArea};
// use std::{sync::mpsc::channel, thread};
use serde_derive::{Serialize, Deserialize};
use std::{
    sync::{Arc, Mutex},
    thread,
    time::Duration,
};

const BLUE: Color32 = Color32::from_rgb(123, 180, 255);
const GREEN: Color32 = Color32::from_rgb(110, 255, 110);
pub struct  MyApp{
    pub (crate) date: bool,
    has_next: bool,
    open_win_name:String,
    open_win_code:String,
    pub articles: Vec<NewsCardData>,
    news_config:NewsConfig,
    api_used:bool,
    totalResults:u32,
    pub field: Arc<Mutex<i128>>,
}
#[derive(Serialize, Deserialize,Default)]
pub struct NewsConfig{
    pub api_key:String,
    pub current_page: u32,
    pub page_line: u32,
}

impl  NewsConfig{
    fn new()->Self{
        Self { 
            current_page: Default::default(),
            page_line: Default::default(),
            api_key: String::new() 
        }
    }
}

pub struct NewsCardData {
    pub title: String,
    pub url:String,
    pub publishedAt:String
}
impl MyApp{
    pub fn new(cc: &eframe::CreationContext<'_>) -> MyApp {
        setup_custom_fonts(&cc.egui_ctx);
        let config: NewsConfig = confy::load("yumd", "yumdconfig").unwrap_or_default();
        // let iter = (0..20).map(|a| NewsCardData {
        //     title: format!("title{}", a),
        // });
        MyApp { 
            date: false, 
            has_next: false,
            api_used:!&config.api_key.is_empty(),
            open_win_name:"None".to_string(),
            open_win_code:String::new(), 
            articles: vec![],
            news_config:config,
            totalResults:0,
            field: Arc::new(Mutex::new(0))    
        }
    }
    pub fn fech_news(&mut self){
            let config:NewsConfig = confy::load("yumd", "yumdconfig").unwrap_or_default();
            if let Ok(response) = api::NewsApi::new("kr", config.page_line, config.current_page).get_api(config.api_key){
                self.totalResults=response.totalResults;
                let articles = response.articles();
                for a in articles.iter(){
                    let (first,last) = a.publishedAt.split_at(10);
                    let news = NewsCardData{
                        title: a.title.to_string(),
                        url:a.url.to_string(),
                        publishedAt:first.to_string()
                    };
                    self.articles.push(news);
                }
            };
    }
    fn fech_newsupdate(&mut self){
        self.articles.clear();
        if let Ok(response) = api::NewsApi::new("kr", self.news_config.page_line, self.news_config.current_page).get_api(self.news_config.api_key.to_string()){
            self.totalResults=response.totalResults;
            let articles = response.articles();
            for a in articles.iter(){
                let (first,last) = a.publishedAt.split_at(10);
                let news = NewsCardData{
                    title: a.title.to_string(),
                    url:a.url.to_string(),
                    publishedAt:first.to_string()
                };
                self.articles.push(news);
        };
        }
    }
    
    pub fn render_sys(&mut self, ui: &mut Ui,ctx: &egui::Context){
        let my_system = MyInfo::MyInfo::new();
        ui.heading("Check System Files");
        self.new_windows(ctx);
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
                        self.has_next=!self.has_next;
                        self.open_win_name=i.sysname;
                        self.open_win_code=i.code;
                    }
                }else {
                    ui.label(RichText::new("Undefined").color(Color32::from_rgb(244, 4, 4)));
                }
            }
        });
        ui.horizontal_wrapped(|ui|{
            ui.label("Second Thread Working time : ");
            ui.colored_label(GREEN,format!("{}",self.field.lock().unwrap()));
        });
    }
    pub fn news_menu(&mut self, ui: &mut Ui){
        // let mut newsapi = api::NewsApi::new("kr", self.news_config.page_line, self.news_config.current_page);
        // let article =newsapi.get_api().unwrap();
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
                if ui.small_button(RichText::new("➖").size(15.)).clicked(){
                    if self.news_config.current_page!=1{
                        self.news_config.current_page -=1;
                        self.fech_newsupdate();
                    };
                };
                ui.add(egui::DragValue::new(&mut self.news_config.current_page).speed(1.0));
                if ui.small_button(RichText::new("➕").size(15.)).clicked(){
                    self.news_config.current_page +=1;
                    self.fech_newsupdate();
                };
                ui.label("page line : ");
                ui.add(egui::DragValue::new(&mut self.news_config.page_line));
                if ui.button(RichText::new("🔁").size(15.)).clicked(){
                    self.fech_newsupdate();
                    // newsapi.update("kr", self.news_config.page_line, self.news_config.current_page);
                };
                ui.label("Total : ");
                ui.label(self.totalResults.to_string());
                
            });
            if self.api_used{
                ui.add_space(5.0);
                ui.separator();
                egui::ScrollArea::vertical().show(ui, |ui| {
                    for i in &self.articles{
                        ui.horizontal_wrapped(|ui|{
                            ui.hyperlink_to(i.title.as_str(), i.url.as_str());
                            ui.colored_label(GREEN,i.publishedAt.as_str());
                        });
                    };
                    // ui.vertical_centered(|ui| {
                        
                    // });
                });
            }else {
                ui.horizontal_wrapped(|ui|{
                    ui.label("API KEY");
                    let text_input=ui.text_edit_singleline(&mut self.news_config.api_key);
                    if text_input.lost_focus()&&ui.input(|i| i.key_pressed(egui::Key::Enter)){
                        if let Err(e)=confy::store("yumD", "yumdconfig", NewsConfig{
                            current_page:1,
                            page_line:20,
                            api_key:self.news_config.api_key.to_string()
                        }){
                            tracing::error!("Failed saving app state:{}",e);
                        }
                        // self.api_used=true;
                        tracing::error!("api key set");
                    }
                });
            };
        });
    }
    pub fn new_windows(&mut self, ctx: &egui::Context){
        let mut temp = egui::Window::new("Code View").id("code".into()).open(&mut self.has_next).vscroll(true);
            // let temp = egui::Window::new("My Window").id("ttt".into()).open(&mut self.has_next);
            temp.show(ctx, |ui| {
                ui.max_rect();
                ui.code_editor(&mut self.open_win_code);
                ui.label(self.open_win_name.as_str());
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
fn setup_custom_fonts(ctx: &egui::Context) {
    // Start with the default fonts (we will be adding to them rather than replacing them).
    let mut fonts = egui::FontDefinitions::default();

    // Install my own font (maybe supporting non-latin characters).
    // .ttf and .otf files supported.
    fonts.font_data.insert(
        "my_font".to_owned(),
        egui::FontData::from_static(include_bytes!(
            "../../../koryungddal.ttf"
        )),
    );

    // Put my font first (highest priority) for proportional text:
    fonts
        .families
        .entry(egui::FontFamily::Proportional)
        .or_default()
        .insert(0, "my_font".to_owned());

    // Put my font as last fallback for monospace:
    fonts
        .families
        .entry(egui::FontFamily::Monospace)
        .or_default()
        .push("my_font".to_owned());

    // Tell egui to use these fonts:
    ctx.set_fonts(fonts);
}