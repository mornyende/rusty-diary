use gtk::Calendar;
use std::{fs,path::Path};

pub trait DiaryCal {
    fn mark_month(&self);
}

impl DiaryCal for Calendar {
    fn mark_month(&self) {
        let year = &self.year();
        let month = &self.month();
        let prefix = format!("{}.{}",year,month+1);
        let test: Vec<u32> = fs::read_dir(Path::new("entries")).expect("test")
            .filter_map(Result::ok) 
            .filter(|entry| entry.file_name().to_str().map_or(false, |s| s.starts_with(&prefix)))
            .filter_map(|entry| entry.path()
                .file_name()?.to_str()?.split('.')
                .last()?.parse::<u32>().ok())
            .collect();
        for day in test {
            self.mark_day(day);
        }
    }
}
