/*use std::{
    fs::*,
    io::{prelude::*, BufReader},
    path::Path,
};*/
//use adw::Application;
use gtk::{glib, CssProvider, prelude::*, gdk::Display};
//const APP_ID: &str = "org.gtk_rs.Css1";

fn main() -> glib::ExitCode {
    let application = adw::Application::new(
        Some("rusty_diary"),
        Default::default(),
    );
    application.connect_startup(|_| load_css());
    application.connect_activate(build_ui);
    application.run()
}

fn load_css() {
    let provider = CssProvider::new();
    provider.load_from_path("./src/style.css");

    gtk::style_context_add_provider_for_display(
        &Display::default().expect("Could not connect to a display."),
        &provider,
        gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
    );
}

pub fn build_ui(application: &adw::Application) {
    let ui_src = include_str!("text_viewer.ui");
    let builder = gtk::Builder::from_string(ui_src);

    let window = builder
        .object::<gtk::ApplicationWindow>("window")
        .expect("Couldn't get window");
    window.set_application(Some(application));

    let text_view = builder
        .object::<gtk::TextView>("text_view")
        .expect("Couldn't get text_view");
    let _text_view2 = builder
        .object::<gtk::TextView>("text_view2")
        .expect("Couldn't get text_view");
    let calendar = builder
        .object::<gtk::Calendar>("calendar")
        .expect("Couldn't get calendar");
    let save_button = builder
        .object::<gtk::Button>("save_button")
        .expect("Couldn't get button");

    calendar.connect_day_selected(glib::clone!(@weak window, @weak text_view => move |cal| {
        let date = cal.date().format("%Y.%m.%d").unwrap();
        let file_path = format!("./entries/{}", date);
        let contents = match std::fs::read_to_string(&file_path) {
            Ok(contents) => contents,
            Err(_) => {
                std::fs::write(&file_path, "").expect("Unable to write file");
                String::new()
            },
        };
        text_view.buffer().set_text(&contents);
    }));
    
    save_button.connect_clicked(glib::clone!(@weak text_view, @weak calendar => move |_| {
        let _ = update_entry(calendar,text_view);
    }));

    window.present();
}

fn update_entry(calendar: gtk::Calendar,text_view: gtk::TextView) -> std::io::Result<()> {
        let date = calendar.date().format("%Y.%m.%d").unwrap();
        let buffer = text_view.buffer();
        let start = buffer.start_iter();
        let end = buffer.end_iter();
        let entry = buffer.text(&start,&end,true);
        std::fs::write(format!("./entries/{}", date),entry)
}
