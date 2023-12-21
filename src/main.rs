/*use std::{
    fs::*,
    io::{prelude::*, BufReader},
    path::Path,
};*/
//use adw::Application;
//use gtk::{glib, CssProvider, prelude::*, gdk::Display};
use gtk::{glib, CssProvider, gdk::Display};
use adw::prelude::*;
//use adw::subclass::prelude;
const APP_ID: &str = "com.rusty_diary";

fn main() -> glib::ExitCode {
    let application = adw::Application::new(
        Some(APP_ID),
        Default::default(),
    );
    application.connect_startup(|_| {load_css(); set_color_scheme()});
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

fn set_color_scheme() {
    let style_manager = adw::StyleManager::default();
    style_manager.set_color_scheme(adw::ColorScheme::PreferDark);
}

pub fn build_ui(application: &adw::Application) {
    let ui_src = include_str!("text_viewer.ui");
    let builder = gtk::Builder::from_string(ui_src);

    let window = builder
        .object::<gtk::ApplicationWindow>("window")
        .expect("Couldn't get window");
    window.set_application(Some(application));

    let text_title = builder
        .object::<gtk::Label>("text_title")
        .expect("Couldn't get label");
    text_title.add_css_class("entry_label");
    text_title.add_css_class("title-4");
    //text_title.set_halign(gtk::Align::Start);
    let text_view = builder
        .object::<gtk::TextView>("text_view")
        .expect("Couldn't get text_view");
    let _text_view2 = builder
        .object::<gtk::TextView>("text_view2")
        .expect("Couldn't get text_view");
    let calendar = builder
        .object::<gtk::Calendar>("calendar")
        .expect("Couldn't get calendar");
    calendar.set_show_week_numbers(true);
    let save_button = builder
        .object::<gtk::Button>("save_button")
        .expect("Couldn't get button");

    //Init calendar contents if they already exist
    let date = calendar.date().format("%Y.%m.%d").unwrap();
    let file_path = format!("./entries/{}", date);
    match std::fs::read_to_string(&file_path) {
        Ok(contents) => text_view.buffer().set_text(&contents),
        Err(_) => text_view.buffer().set_text(&""),
    };
    //calendar.mark_day(2);
    calendar.connect_day_selected(glib::clone!(@weak window, @weak text_view => move |cal| {
        let date = cal.date().format("%Y.%m.%d").unwrap();
        let file_path = format!("./entries/{}", date);
        match std::fs::read_to_string(&file_path) {
            Ok(contents) => text_view.buffer().set_text(&contents),
            Err(_) => text_view.buffer().set_text(&""),
        };
    }));
    
    save_button.connect_clicked(glib::clone!(@weak text_view, @weak calendar => move |_| {
        let _ = update_entry(calendar,text_view); }));

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
