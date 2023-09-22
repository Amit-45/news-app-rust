use confy::ConfyError;
use eframe::egui::{
    self, Button, Color32, Context, CtxRef, FontDefinitions, FontFamily, Hyperlink, Label, Layout,
    Response, Separator, TopBottomPanel, Window,
};
use newsapi::NewsAPI;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;
// const PADDING: f32 = 5.0;
// const WHITE: Color32 = Color32::from_rgb(255, 255, 255);

const PADDING: f32 = 5.0;
const WHITE: Color32 = Color32::from_rgb(255, 255, 255);
const CYAN: Color32 = Color32::from_rgb(0, 255, 255);
const BLACK: Color32 = Color32::from_rgb(0, 0, 0);
const RED: Color32 = Color32::from_rgb(255, 0, 0);

#[derive(Serialize, Deserialize, Clone)]
pub struct HeadlinesConfig {
    pub dark_mode: bool,
    pub api_key: String,
    pub api_key_initialised: bool,
}



impl Default for HeadlinesConfig {
    fn default() -> Self {
        Self {
            dark_mode: Default::default(),
            api_key: String::new(),
            api_key_initialised: false,
        }
    }
}



// impl HeadlinesConfig {
//     fn new() -> Self {
//         Self { dark_mode: true }
//     }
// }


// pub fn store<'a, T: Serialize>(
//     app_name: &str,
//     config_name: impl Into<Option<&'a str>>,
//     cfg: T
// ) -> Result<(), ConfyError>


#[derive(Default)]
pub struct Headlines {
    pub articles: Vec<NewsCardData>,
    pub config: HeadlinesConfig,
    pub api_key_initialised: bool,
}
// pub enum Msg {
//     ApiKeySet(String),
//     Refresh,
// }

pub struct NewsCardData {
    pub title: String,
    pub desc: String,
    pub url: String,
}
impl Headlines {
    pub fn new() -> Headlines {
        let iter = (0..20).map(|a| NewsCardData {
            title: format!("Title {}", a),
            desc: format!("Description {}", a),
            url: format!("https://example.com/{}", a),
        });
        // .collect();
        // let config := confy::load("headlines").unwrap_or_default();
        let config: HeadlinesConfig = confy::load("headlines", "headlines").unwrap_or_default();
        Headlines {
            api_key_initialised: !config.api_key.is_empty(),
            articles: Vec::from_iter(iter),
            config,
        }
    }

    pub fn configure_fonts(&self, ctx: &Context) {
        let mut font_def = FontDefinitions::default();
        font_def.font_data.insert(
            "MesloLGS".to_string(),
            Cow::Borrowed(include_bytes!("../../MesloLGS_NF_Regular.ttf")),
        );

        font_def.family_and_size.insert(
            eframe::egui::TextStyle::Heading,
            (FontFamily::Proportional, 35.0),
        );
        font_def.family_and_size.insert(
            eframe::egui::TextStyle::Body,
            (FontFamily::Proportional, 20.0),
        );
        font_def
            .fonts_for_family
            .get_mut(&FontFamily::Proportional)
            .unwrap()
            .insert(0, "MesloLGS".to_string());
        ctx.set_fonts(font_def);
    }

    pub fn render_news_cards(&self, ui: &mut eframe::egui::Ui) {
        for article in &self.articles {
            ui.add_space(PADDING);
            //render title
            let title: String = format!("{}", article.title);
            if self.config.dark_mode {
                ui.colored_label(WHITE, title);
            } else {
                ui.colored_label(BLACK, title);
            }

            // ui.colored_label(WHITE, title);
            //render desc
            ui.add_space(PADDING);
            let desc = Label::new(&article.desc).text_style(eframe::egui::TextStyle::Button);
            ui.add(desc);

            //render hyperlinks
            let hyperlink_color = if self.config.dark_mode { CYAN } else { RED };

            // Display a separator after each title.
            let sep = Separator::default().spacing(20.);
            ui.add(sep);

            ui.add_space(PADDING);
            ui.label(&article.url);
            ui.with_layout(Layout::right_to_left(), |ui| {
                ui.add(Hyperlink::new(&article.url).text("read more ⤴"));
            });
        }
    }
    // frame: &mut eframe::Frame
    // eframe::emath::Align::Min
    // RichText::new
    // .text_style(egui::TextStyle::Heading) [add after emoji]

    //
    #[allow(unused)]
    pub(crate) fn render_top_panel(&mut self, ctx: &CtxRef, frame: &mut eframe::epi::Frame) {
        // define a TopBottomPanel widget
        TopBottomPanel::top("top_panel").show(ctx, |ui| {
            ui.add_space(10.);
            egui::menu::bar(ui, |ui| {
                // logo
                ui.with_layout(Layout::left_to_right(), |ui| {
                    ui.add(Label::new(("📰")));
                });
                // controls

                let close_btn = ui.add(Button::new("❌").text_style(egui::TextStyle::Body));

                if close_btn.clicked() {
                    frame.quit();
                }

                let refresh_btn = ui.add(Button::new("🔄").text_style(egui::TextStyle::Body));
                let theme_btn = ui.add(Button::new({ "🌙" }).text_style(egui::TextStyle::Body));

                if theme_btn.clicked() {
                    self.config.dark_mode = !self.config.dark_mode;
                    // dark_mode !=dark_mode;
                }
            });

            ui.add_space(10.);
        });
    }

    //     pub fn render_config(&mut self, ctx: &CtxRef) {
    //         Window::new("Configuration").show(ctx, |ui| {
    //             // Add contents to the configuration window here.
    //             ui.label("Enter your API_KEY for newsapi.org");
    //             let text_input: Response = ui.text_edit_singleline(&mut self.config.api_key);
    //             // println!("API Key---> : {}", self.config.api_key);

    //             if text_input.lost_focus() && ui.input().key_pressed(egui::Key::Enter) {

    //             ui.label("If you have not registered an Api key ");
    //             ui.hyperlink("https://newsapi.org");
    //         });
    //     }
    // }
    // pub fn render_config(&mut self, ctx: &CtxRef) {
    //     Window::new("Configuration").show(ctx, |ui| {
    //         ui.label("Enter your API_KEY for newsapi.org");

    //         // Text input for API key
    //         let text_input: Response = ui.text_edit_singleline(&mut self.config.api_key);

    //         if text_input.lost_focus() && ui.input().key_pressed(egui::Key::Enter) {
    //             // When Enter key is pressed and the input field loses focus, save the API key
    //             // You can also check if the API key is not empty before saving it
    //             if !self.config.api_key.is_empty() {
    //                 if let  = confy::store_path("headlines", self.config.clone())
                    
    //                 // 
    //                 confy::store("my-app-name", None, my_cfg)?;

    //                  {
    //                     // Handle the error when saving the API key
    //                     println!("Failed to save API key: {:?}", e);
    //                 } else {
    //                     self.config.api_key_initialised = true;
    //                     println!("API key saved successfully!");
    //                 }
    //             }
    //         }

    //         ui.label("If you have not registered an API key ");
    //         ui.hyperlink("https://newsapi.org");
    //     });
    // }

    //          if let Err(e) = confy::store("headlines","headlines", HeadlinesConfig {
    //          dark_mode: self.config.dark_mode,
    // api_key: self.config.api_key.to_string();
    //      }) {
    //          tracing::error!("Failed saving app state",e);
    //      }

    //     //  self.api_key_initialised = true;

    //     tracing::error!("api key set");
}
