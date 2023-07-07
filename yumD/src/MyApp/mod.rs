use super::MyInfo;
use api::{TwitchToken,TwitchFollowRespone};
use twitch_irc::{ClientConfig, SecureTCPTransport, login::StaticLoginCredentials, TwitchIRCClient, message::ServerMessage};
use diesel::sql_types::Array;
use tokio::time::{sleep, Duration};
use eframe::{egui::{self, Label, Sense, Spinner}, emath::Align};

use std::{sync::atomic::{AtomicBool, Ordering} };
use std::{process::{Command, Stdio}, sync::mpsc::{Sender,Receiver},};
use egui::{RichText,Color32,collapsing_header::CollapsingState,InnerResponse,Ui,Response,ScrollArea};
// use std::{sync::mpsc::channel, thread};
use serde_derive::{Serialize, Deserialize};
use std::{
    sync::{Arc, Mutex},
    thread,
};


const BLUE: Color32 = Color32::from_rgb(123, 180, 255);
const GREEN: Color32 = Color32::from_rgb(110, 255, 110);
pub struct  MyApp{
    pub (crate) date: bool,
    setting_menu_open:bool,
    on_setting_menu:OpenSetting,
    has_next: bool,
    open_win_name:String,
    open_win_code:String,
    pub articles: Vec<NewsCardData>,
    tfollow_list: Vec<TwitchFollowInfo>,
    chatting_layer:Option<ChattingLayer>,
    news_config:NewsConfig,
    youtube_config:YoutubeConfig,
    twitch_config:TwitchConfig,
    api_used:bool,
    totalResults:u32,
    pub ttoken_time: Arc<Mutex<u64>>,
    pub field: Arc<Mutex<i128>>,
    token:Token,
    down_option:DownOption,
    down_history:Vec<DownOption>
}
enum ChattingLayer{
    TEXT,
    DATA
}
struct Token{
    twitch_id:String,
    twitch_nick:String,
    twitch_date:String,
    twitch_token:String,
    twitch_use:bool,
    youtubu_token:String,
    youtubu_use:bool,
}
impl Token{
    fn new()->Self{
        Self { 
            twitch_id:String::new(),
            twitch_nick:String::new(),
            twitch_date:String::new(),
            twitch_token: String::new(), 
            twitch_use: false, 
            youtubu_token: String::new(), 
            youtubu_use: false 
        }
    }
}
#[derive(Serialize, Deserialize,Default)]
pub struct NewsConfig{
    pub api_key:String,
    pub current_page: u32,
    pub page_line: u32,
}
#[derive(Serialize, Deserialize,Default)]
struct TwitchConfig{
    id:String,
    password:String,
    client_id:String,
    client_secret:String
}
#[derive(Serialize, Deserialize,Default)]
struct YoutubeConfig{
    id:String,
    password:String
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
impl  TwitchConfig{
    fn new()->Self{
        Self { 
            id: String::new(),
            password: String::new(),
            client_id:String::new(),
            client_secret:String::new()
        }
    }
}
impl  YoutubeConfig{
    fn new()->Self{
        Self { 
            id: String::new(),
            password: String::new() 
        }
    }
}
enum OpenSetting{
    twitch,
    youtube,
}

pub struct NewsCardData {
    pub title: String,
    pub url:String,
    pub publishedAt:String
}

#[derive(Debug,Clone)]
struct DownOption{
    quality:bool,
    sublang:bool,
    download_dir:String,
    download_url:String,
    
}
impl DownOption {
    fn new()->Self{
        return Self{
            quality:true,
            sublang:false,
            download_dir:"/home/yum/Download".to_string(),
            download_url:String::new(),
           
        };
    }
    // fn get(&mut self){
        
    //     let mut args: Vec<String>=Vec::new();
    //     if self.quality {
    //         args.push("-f bestvideo+bestaudio".to_string());
    //         // args.push("".to_string());
    //     }
    //     if self.sublang {
    //         args.push("--write-auto-subs".to_string());
    //         args.push("--sub-lang ko".to_string());
    //     }
    //     args.push(self.download_url.to_string());
    //     self.is_start=true;
    //     let download =thread::spawn(move||{
    //         Command::new("yt-dlp")
    //         .args(args)
    //         // .arg(format!("https://www.twitch.tv/{}",temp))
    //         .status()
    //         .unwrap();
    //     });
    //     match download.join() {
    //         Ok(_)=>{
    //             self.is_done=true;
    //             self.is_start=false;
    //         },
    //         Err(_)=>self.is_fail=true
    //     }
    // }
}
#[derive(Debug)]
struct TwitchMessage{
    client_nonce:String,
    tmi_sent_ts:String,
    server_timestamp:String,
    sender_name:String,
    sender_id:String,
    sender_login:String,
    text:String,
}
impl TwitchMessage {
    fn new()->Self{
        return  Self 
        { 
            client_nonce: String::new(), 
            tmi_sent_ts:String::new(),
            server_timestamp:String::new(),
            sender_name:String::new(),
            sender_id:String::new(),
            sender_login:String::new(),
            text:String::new()
        }

    }
}
#[derive(Clone)]
struct TwitchFollowInfo {
    id_num:String,
    id_str:String,
    nick_name:String,
    followed_at:String,
    view_on:Arc<AtomicBool>,
    chat:bool,
    chat_on:Arc<AtomicBool>,
    
    message_list:Arc<Mutex<Vec<TwitchMessage>>>
}

impl TwitchFollowInfo {
    fn view_open (&mut self){
        let temp = self.id_str.clone();
        let video_status = Arc::clone(&self.view_on);
        let videoview =thread::spawn(move||{
            let video =Command::new("mpv")
            .arg(format!("https://www.twitch.tv/{}",temp))
            .status();
            match video {
                Ok(status)=>{
                    match status.code() {
                        Some(0)=>(*video_status).store(false,Ordering::Relaxed),
                        Some(2)=>(*video_status).store(false,Ordering::Relaxed),
                        Some(_)=>println!("error"),
                        None=>println!("error")
                    }
                },
                Err(_)=>{
                    println!("에에에에러러ㅓ")
                    // self.view=false;
                }
            }
        });
    }
    #[tokio::main]
    pub async fn get_chatting(&mut self){
        let chat_status = Arc::clone(&self.chat_on);
        let messagelist = Arc::clone(&mut self.message_list);
        let config = ClientConfig::default();
        let (mut incoming_messages, client) =
        TwitchIRCClient::<SecureTCPTransport, StaticLoginCredentials>::new(config);
        let join_handle = tokio::spawn(async move {
            while (*chat_status).load(Ordering::Relaxed) {
                sleep(Duration::from_millis(1)).await;
                if let Some(message) = incoming_messages.try_recv().ok(){
                    let mut sendmsg = TwitchMessage::new();
                    match message {
                        ServerMessage::Privmsg(msg) => {
                            if let Some(clientnonce)=msg.source.tags.0.get("client-nonce"){
                                let temp = clientnonce.clone();
                                sendmsg.client_nonce=temp.unwrap();
                            }
                            if let Some(tmisent)=msg.source.tags.0.get("tmi-sent-ts"){
                                let temp = tmisent.clone();
                                sendmsg.tmi_sent_ts=temp.unwrap();
                            }
                            sendmsg.text=msg.message_text;
                            sendmsg.sender_login=msg.sender.login;
                            sendmsg.sender_id=msg.sender.id;
                            sendmsg.sender_name=msg.sender.name;
                            sendmsg.server_timestamp=msg.server_timestamp.to_string();
                            // println!("{:?}",sendmsg);
                            messagelist.lock().unwrap().push(sendmsg);
                        },
                        ServerMessage::Whisper(msg) => {
                            if let Some(clientnonce)=msg.source.tags.0.get("client-nonce"){
                                let temp = clientnonce.clone();
                                sendmsg.client_nonce=temp.unwrap();
                            }
                            if let Some(tmisent)=msg.source.tags.0.get("tmi-sent-ts"){
                                let temp = tmisent.clone();
                                sendmsg.tmi_sent_ts=temp.unwrap();
                            }
                            sendmsg.text=msg.message_text;
                            sendmsg.sender_login=msg.sender.login;
                            sendmsg.sender_id=msg.sender.id;
                            sendmsg.sender_name=msg.sender.name;
                            // sendmsg.server_timestamp=msg.server_timestamp.to_string();
                            // println!("{:?}",sendmsg);
                            messagelist.lock().unwrap().push(sendmsg);
                            // println!("(w) {}: {}{:?}", msg.sender.name, msg.message_text,msg.source.tags.0);
                        },
                        _ => {}
                    }
                };
                // println!("{:?}",(*chat_status).load(Ordering::Relaxed));
                // match  incoming_messages.try_recv().ok(){
                //     Some(message)=>{
                //         println!("{:?}",message);
                //     },
                //     None=>{}
                // }
            }
        });
        client.join(self.id_str.to_owned()).unwrap();
        join_handle.await.unwrap();
    }
    
}

impl MyApp{
    pub fn new(cc: &eframe::CreationContext<'_>) -> MyApp {
        setup_custom_fonts(&cc.egui_ctx);
        let config: NewsConfig = confy::load("yumd", "yumdconfig").unwrap_or_default();
        let twitch_config:TwitchConfig = confy::load("yumd", "TwitchConfig").unwrap_or_default();
        let youtube_config:YoutubeConfig = confy::load("yumd", "YoutubeConfig").unwrap_or_default();
        MyApp { 
            date: false, 
            setting_menu_open:false,
            on_setting_menu:OpenSetting::twitch,
            has_next: false,
            api_used:!&config.api_key.is_empty(),
            open_win_name:"None".to_string(),
            open_win_code:String::new(), 
            articles: vec![],
            news_config:config,
            chatting_layer:Some(ChattingLayer::DATA),
            totalResults:0,
            youtube_config:youtube_config,
            twitch_config:twitch_config,
            field: Arc::new(Mutex::new(0)),
            token:Token::new(),
            ttoken_time: Arc::new(Mutex::new(0)),
            tfollow_list: vec![],
            down_option:DownOption::new(),
            down_history:vec![],
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
    
    pub fn twitch_login(&mut self){
        let token = TwitchToken::new(
            &self.twitch_config.client_id,
            &self.twitch_config.client_secret,
        );
        match token {
            Ok(mut token)=>{
                self.token.twitch_token=token.access_token.clone();
                let getlogin = token.user_login(&self.twitch_config.id, &self.token.twitch_token, &self.twitch_config.client_id).unwrap();
                self.token.twitch_use=true;
                self.token.twitch_id=getlogin.data[0].id.clone();
                self.token.twitch_nick=getlogin.data[0].display_name.clone();
                self.token.twitch_date=getlogin.data[0].created_at.clone();
            },
            Err(e)=>println!("{:?}",e)
        }
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
        self.setting_menu(ctx);
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
            ui.label("Twitch Token Time : ");
            ui.colored_label(GREEN,format!("{}",self.ttoken_time.lock().unwrap()));
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
                        tracing::error!("api key set");
                    }
                });
            };
        });
    }
    pub fn new_windows(&mut self, ctx: &egui::Context){
        let temp = egui::Window::new("Code View").id("code".into()).open(&mut self.has_next).vscroll(true);
            temp.show(ctx, |ui| {
                ui.max_rect();
                ui.code_editor(&mut self.open_win_code);
                ui.label(self.open_win_name.as_str());
            });
    }
    fn new_chatting(&mut self, ctx: &egui::Context,){
        for i in self.tfollow_list.iter_mut(){
            if !i.chat{
                (*i.chat_on).store(i.chat,Ordering::Relaxed);
            }
            // (*i.chat_on).store(i.chat,Ordering::Relaxed);
            // println!("{:?}",(*i.chat_on).load(Ordering::Relaxed));
            let mut item=i.clone();
            let temp = egui::Window::new("Chattings").id(item.id_str.to_string().into()).open(&mut i.chat).vscroll(true);
                temp.show(ctx, |ui| {
                    let menu =ui.horizontal_top(|ui|{
                        ui.label("Text Layer : ");
                        if ui.button("TEXT").clicked(){
                            self.chatting_layer=Some(ChattingLayer::TEXT);
                            // let mut temp = item.clone();
                            
                        };
                        if ui.button("DATA").clicked(){
                            self.chatting_layer=Some(ChattingLayer::DATA);
                        };
                    });
                    
                    ui.separator();
                    egui::ScrollArea::vertical().show(ui, |ui| {
                        for sendmsg in &*i.message_list.lock().unwrap(){
                            ui.horizontal_wrapped(|ui|{
                                match self.chatting_layer {
                                    Some(ChattingLayer::TEXT)=>{
                                        ui.label(format!("{} : ",sendmsg.sender_name.to_string()));
                                        ui.label(sendmsg.text.to_string());
                                    },
                                    Some(ChattingLayer::DATA)=>{
                                        ui.label(format!("{},{},{},{},{},{},{} ",
                                        sendmsg.sender_name.to_string(),
                                        sendmsg.sender_id.to_string(),
                                        sendmsg.sender_login.to_string(),
                                        sendmsg.client_nonce.to_string(), 
                                        sendmsg.tmi_sent_ts.to_string(), 
                                        sendmsg.server_timestamp.to_string(),
                                        sendmsg.text.to_string(), 
                                    ));
                                    },
                                    None=>{}
                                }
                                
                            });
                        }
                        // egui::ScrollArea::vertical().show(ui, |ui| {
                            
                        // });
                        ui.label("");
                        ui.scroll_to_cursor(Some(Align::BOTTOM));
                    });
                    
                    ui.separator();
                    
                });
        }
    }
    fn setting_menu(&mut self, ctx: &egui::Context){
        match &self.on_setting_menu {
            OpenSetting::twitch=>{
                if !self.token.twitch_use{
                    let menu = egui::Window::new("Twitch Settings").id("settingmenu".into()).open(&mut self.setting_menu_open);
                menu.show(ctx, |ui|{
                    ui.max_rect();
                    ui.horizontal_wrapped(|ui|{
                        ui.label("ID                        : ");
                        ui.text_edit_singleline(&mut self.twitch_config.id);
                    });
                    ui.horizontal_wrapped(|ui|{
                        ui.label("PASSWORD   : ");
                        let psw = egui::TextEdit::singleline(&mut self.twitch_config.password).password(true).show(ui);
                    });
                    ui.horizontal_wrapped(|ui|{
                        ui.label("Client ID           : ");
                        let clientid = egui::TextEdit::singleline(&mut self.twitch_config.client_id).password(true).show(ui);
                    });
                    ui.horizontal_wrapped(|ui|{
                        ui.label("Client Secret : ");
                        // password::text_edit_singleline(&mut self.twitch_config.password);
                        let clientsecret = egui::TextEdit::singleline(&mut self.twitch_config.client_secret).password(true).show(ui);
                    });
                    ui.vertical_centered(|ui|{
                        if ui.button("OK").clicked(){
                            if let Err(e)=confy::store("yumD", "TwitchConfig", TwitchConfig{
                                id:self.twitch_config.id.to_string(),
                                password:self.twitch_config.password.to_string(),
                                client_id:self.twitch_config.client_id.to_string(),
                                client_secret:self.twitch_config.client_secret.to_string(),
                            }
                        ){
                                tracing::error!("Failed saving Twitch:{}",e);
                            }
                            // *self.twitch_login();
                            let token = TwitchToken::new(
                                &self.twitch_config.client_id,
                                &self.twitch_config.client_secret,
                            );
                            match token {
                                Ok(mut token)=>{
                                    *self.ttoken_time.lock().unwrap()=token.expires_in;
                                    self.token.twitch_token=token.access_token.clone();
                                    let getlogin = token.user_login(&self.twitch_config.id, &self.token.twitch_token, &self.twitch_config.client_id).unwrap();
                                    self.token.twitch_use=true;
                                    self.token.twitch_id=getlogin.data[0].id.clone();
                                    self.token.twitch_nick=getlogin.data[0].display_name.clone();
                                    self.token.twitch_date=getlogin.data[0].created_at.clone();
                                    let followlist = TwitchFollowRespone::twitch_get_follow(&self.token.twitch_id, &self.token.twitch_token, &self.twitch_config.client_id).unwrap();
                                    for i in followlist{  
                                        let info = TwitchFollowInfo{
                                            id_num:i.to_id,
                                            id_str:i.to_login,
                                            nick_name:i.to_name,
                                            followed_at:i.followed_at,
                                            view_on:Arc::new(AtomicBool::new(false)),
                                            chat:false,
                                            chat_on:Arc::new(AtomicBool::new(false)),
                                            message_list:Arc::new( Mutex::new(Vec::new())),
                                        };
                                        self.tfollow_list.push(info);
                                    }
                                },
                                Err(e)=>println!("{:?}",e)
                            }
                        };
                    });
                });
                }else{
                    let menu = egui::Window::new("Twitch Login Info").id("settingmenu".into()).open(&mut self.setting_menu_open);
                    menu.show(ctx, |ui|{
                        ui.label(RichText::new(&self.token.twitch_nick).color(Color32::from_rgb(110, 255, 110)));
                        ui.label(RichText::new(&self.token.twitch_id).color(Color32::from_rgb(110, 255, 110)));
                        ui.label(RichText::new(&self.token.twitch_date).color(Color32::from_rgb(110, 255, 110)));
                        if ui.button("LOGOUT").clicked(){
                            self.token.twitch_use=false;
                            *self.ttoken_time.lock().unwrap()=0;
                        };
                    });
                }
            },
            OpenSetting::youtube=>{
                let menu = egui::Window::new("Youtube Settings").id("settingmenu".into()).open(&mut self.setting_menu_open);
                menu.show(ctx, |ui|{
                    ui.max_rect();
                    ui.horizontal_wrapped(|ui|{
                        ui.label("ID                      : ");
                        ui.text_edit_singleline(&mut self.youtube_config.id);
                    });
                    ui.horizontal_wrapped(|ui|{
                        ui.label("PASSWORD : ");
                        let psw = egui::TextEdit::singleline(&mut self.youtube_config.password).password(true).show(ui);
                    });
                    ui.vertical_centered(|ui|{
                        if ui.button("OK").clicked(){
                            if let Err(e)=confy::store("yumD", "YoutubeConfig", YoutubeConfig{
                                id:self.twitch_config.id.to_string(),
                                password:self.twitch_config.password.to_string()
                            }){
                                tracing::error!("Failed saving Twitch:{}",e);
                            }
                        };
                    });
                });
            },
            _=>self.setting_menu_open=false
        };
        
    }
    pub fn db_menu(&mut self, ui: &mut Ui,){
        let mut connetc=false;
        let mut db_view = collaps_head("dbv",ui);
        let db_header_res = collaps_head_respone(ui,&mut db_view,"show!");
        db_view.show_body_indented(&db_header_res.response, ui, |ui| {
            if ui.button("connect setting").clicked(){
                
            }
        }
        );
    }
    pub fn stream_menu(&mut self, ctx: &egui::Context, ui: &mut Ui){
        self.new_chatting(ctx);
        if *self.ttoken_time.lock().unwrap()==0{
            self.token.twitch_use=false;
        };
        let mut stream_view = collaps_head("stream",ui);
        let stream_header_res = collaps_head_respone(ui,&mut stream_view,"show!");
        stream_view.show_body_indented(&stream_header_res.response, ui, |ui|{
            ui.horizontal_wrapped(|ui|{
                if ui.add(Label::new("Twitch").sense(Sense::click())).clicked() {
                    self.on_setting_menu=OpenSetting::twitch;
                    self.setting_menu_open=!self.setting_menu_open;
                };
                if self.token.twitch_use {
                    ui.label(RichText::new("ON").color(Color32::from_rgb(110, 255, 110)));
                }
                if ui.add(Label::new("Youtube").sense(Sense::click())).clicked() {
                    self.on_setting_menu=OpenSetting::youtube;
                    self.setting_menu_open=!self.setting_menu_open;
                };
                if self.token.youtubu_use {
                    ui.label(RichText::new("ON").color(Color32::from_rgb(110, 255, 110)));
                }
                ui.label("(login api settings)");
            });
            if self.token.twitch_use {
                egui::ScrollArea::vertical().show(ui, |ui| {
                    ui.horizontal_wrapped(|ui|{
                        egui::Grid::new("my_grid")
                            .num_columns(5)
                            .striped(true)
                            .spacing([4.0, 4.0])
                            .show(ui, |ui| {
                                ui.label("ID");
                                ui.label("NICK");
                                ui.label("Live");
                                ui.label("");
                                ui.end_row();
                                for  i in  self.tfollow_list.iter_mut(){
                                    ui.label(i.id_str.to_string());
                                    ui.label(i.nick_name.to_string());
                                    // let video_status: Arc<AtomicBool> = Arc::clone(&i.view_on);
                                    if !(*i.view_on).load(Ordering::Relaxed){
                                        if ui.button("video").clicked(){
                                            (*i.view_on).store(true,Ordering::Relaxed);
                                            i.view_open();
                                        };
                                    }else{
                                        ui.add(Spinner::new());
                                    }
                                    if !(*i.chat_on).load(Ordering::Relaxed){
                                        if ui.button("chat").clicked(){
                                            i.chat=!i.chat;
                                            (*i.chat_on).store(i.chat,Ordering::Relaxed);
                                            if i.chat{
                                                // i.get_chatting();
                                                let mut temp = i.clone();
                                                thread::spawn(move||{
                                                    temp.get_chatting();
                                                });
                                            }
                                        };
                                    }else{
                                        ui.add(Spinner::new());
                                    }
                                    ui.end_row();
                                }
                            });
                    })
                });
            }
        }
        );
        
    }
    pub fn video_menu(&mut self, ui: &mut Ui){
        let mut video_down = collaps_head("downvideo",ui);
        let video_down_res = collaps_head_respone(ui,&mut video_down,"show!");
        video_down.show_body_indented(&video_down_res.response, ui, |ui|{
            ui.horizontal_wrapped(|ui|{
                ui.label("Working Dir : ");
                ui.label(self.down_option.download_dir.to_string());
                if ui.button("choose").clicked(){
                    if let Some(path)=rfd::FileDialog::new().set_directory(self.down_option.download_dir.to_string()).pick_folder(){
                        self.down_option.download_dir=path.display().to_string();
                    }
                }
            });
            ui.horizontal_wrapped(|ui|{
                ui.checkbox(&mut self.down_option.quality, "Quality");
                ui.checkbox(&mut self.down_option.sublang, "한글자막");
                ui.label("URL : ");
                ui.text_edit_singleline(&mut self.down_option.download_url);
                if ui.button("Get").clicked(){
                    
                    // self.down_option.get();
                    self.down_history.push(self.down_option.clone());
                };
            });
            // ui.add_space(5.0);
            // ui.wrap_text();
            // ui.label("text");
            // ui.wrap_text();
            // ui.label("text");
            // ui.vertical(|ui| {
            //     ui.label("over");
            //     ui.label("under");
            // });
            // ui.columns(2, |columns| {
            //     columns[0].label("First column");
            //     columns[1].label("Second column");
            // });
            egui::ScrollArea::vertical().show(ui, |ui| {
                ui.horizontal_wrapped(|ui|{
                    egui::Grid::new("down_history")
                    // .with_row_color(GREEN)
                    // .num_columns(1)
                    // .striped(true)
                    // .spacing([4.0, 4.0])
                    .show(ui, |ui| {
                        for item in self.down_history.iter() {
                            ui.label("URL : ");
                            ui.label(item.download_url.to_string());
                            ui.horizontal(|ui|{
                                let dellist = ui.small_button(RichText::new("🗑").size(15.))
                                .on_hover_text("delete from list");
                                let opendir = ui.small_button(RichText::new("📂").size(15.))
                                .on_hover_text("open folder");
                            });
                            ui.menu_button("My menu", |ui| {
                                ui.menu_button("My sub-menu", |ui| {
                                    if ui.button("Close the menu").clicked() {
                                        ui.close_menu();
                                    }
                                });
                            });
                            
                            ui.end_row();
                        }
                    })
                })
            })
            // egui::Grid::new("some_unique_id").show(ui, |ui| {
                
            //     ui.label("First row, first column");
            //     ui.label("First row, second column");
            //     ui.end_row();
            
            //     ui.label("Second row, first column");
            //     ui.label("Second row, second column");
            //     ui.label("Second row, third column");
            //     ui.end_row();
            
            //     ui.horizontal(|ui| { ui.label("Same"); ui.label("cell"); });
            //     ui.label("Third row, second column");
            //     ui.end_row();
            // });
        });
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
