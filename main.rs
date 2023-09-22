mod headlines;
use newsapi::NewsAPI;
use std::borrow::Cow;

const PADDING: f32 = 5.0;
const WHITE: Color32 = Color32::from_rgb(255, 255, 255);
use eframe::{
    egui::FontDefinitions,
    egui::{
        CentralPanel, Color32, Context, Hyperlink, Label, Layout, Response, ScrollArea, Separator,
        TopBottomPanel, Ui,
    },
    egui::{CtxRef, FontFamily, Vec2},
    epi::App,
    epi::NativeOptions,
    run_native,
};
use headlines::{Headlines, NewsCardData};
const CYAN: Color32 = Color32::from_rgb(0, 255, 255);

// fn fetch_news(api_key: &str){
//     if let Ok() =(response = NewsAPI::new(api_key).fetch(){
//         let articles = response.articles =resposmse articles();
//         for a: &Article in articles .iter(){
//             let news: NewscardData = NewsCardDaa{
//                 title:article.titile
//                 urldesc
//             }
//         }
//     }
// }

fn fetch_news(api_key: &str, articles: &mut Vec<NewsCardData>) {
    println!("API Key: {}", api_key);
    let BASE_URL = "https://newsapi.org/v2";
    let api_key = "API_KEY here";
    let url = format!("{}/top-headlines?country=us&apiKey={}", BASE_URL, api_key);

    println!("URL: {}", url);
    if let Ok(response) = NewsAPI::new(api_key).fetch() {
        let resp_articles = &response.articles();

        for a in resp_articles.iter() {
            // let news = NewsCardData {
            //     title: a.title().to_string(),
            //     urldesc: a.url().to_string(),
            //     desc: a.desc().map(|s| s.to_string()).unwrap_or("...".to_string()),
            // };
            let news = NewsCardData {
                title: a.title().to_string(),
                url: a.url().to_string(),
                desc: a.desc().map(|s| s.to_string()).unwrap_or("...".to_string()),
            };

            articles.push(news)

            // Do something with the 'news' data
            // For example, you can print it: println!("{:?}", news);
        }
    }

    // else
    // {
    //     println!("Failed to fetch news");
    // }
    if let Err(err) = NewsAPI::new(api_key).fetch() {
        println!("Failed to fetch news: {:?}", err);
    }
}

impl App for Headlines {
    fn setup(
        &mut self,
        ctx: &CtxRef,
        _frame: &mut eframe::epi::Frame,
        storage: Option<&dyn eframe::epi::Storage>,
    ) {
        fetch_news(&self.config.api_key, &mut self.articles);
        self.configure_fonts(ctx);
    }
    fn update(&mut self, ctx: &eframe::egui::CtxRef, frame: &mut eframe::epi::Frame) {
        if self.config.dark_mode {
            ctx.set_visuals(eframe::egui::Visuals::dark()); // Set dark visuals
        } else {
            ctx.set_visuals(eframe::egui::Visuals::light()); // Set light visuals
        }

        // if !self.config.api_key_initialised {
        //     self.render_config(ctx);
        // } else {
        // self.render_top_panel(ctx, frame);
        // self.render_config(ctx);

        self.render_top_panel(ctx, frame);
        CentralPanel::default().show(ctx, |ui| {
            // render_footer(&self, ctx);
            render_header(ui);

            ScrollArea::auto_sized().show(ui, |ui| {
                self.render_news_cards(ui);
            });
            render_footer(ctx);
        });
    }

    fn name(&self) -> &str {
        "Headlines"
    }
}

fn render_header(ui: &mut Ui) {
    ui.vertical_centered(|ui| {
        ui.heading("headlines");
    });
    ui.add_space(PADDING);
    let sep = Separator::default().spacing(20.);
    ui.add(sep);
}
// pub(crate)
// &self,
fn render_footer(ctx: &CtxRef) {
    TopBottomPanel::bottom("footer").show(ctx, |ui| {
        ui.vertical_centered(|ui| {
            ui.add_space(10.);
            ui.add(Label::new("API source: newsapi.org").monospace());

            ui.add(Hyperlink::from_label_and_url(
                "Made with egui",
                "https://github.com/emilk/egui",
            ));
            ui.add(Hyperlink::from_label_and_url(
                "github/amit45",
                "https://github.com/Amit-45",
            ));

            ui.add_space(10.);
        })
    });
}

// pub(crate)
//&self,

fn main() {
    tracing_subscriber::fmt::init();
    tracing::error!("this is log");
    let app = Headlines::new();
    // let headlines = Headlines::new();
    let mut win_option = NativeOptions::default();
    win_option.initial_window_size = Some(Vec2::new(540.0, 960.0));
    run_native(Box::new(app), win_option);
}
