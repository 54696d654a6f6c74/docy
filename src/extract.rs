use crate::storefile::{self, Capture};
use std::{ffi::OsString, fs, str::Split};
use walkdir::DirEntry;

pub fn run(targets: Vec<DirEntry>, mut store_data: storefile::StoreFile) {
    if store_data.last_action.is_some() && store_data.last_action.unwrap() == storefile::Action::Ex
    {
        println!("Previous action was extraction. Exiting without doing any work...");
        return;
    }

    let mut changed_files = vec![];
    let mut new_file_contents = vec![];

    for target in &targets {
        println!("{:?}", &target);
        let contents = fs::read_to_string(target.path()).expect("Failed to read file");

        let (captrues, lines) = identify_js_doc(&contents);
        let lines: Vec<&str> = lines.collect();

        let f = storefile::File {
            captrues: captrues.clone(),
            path: OsString::from(target.path()),
        };
        changed_files.push(f);

        let new_content = get_content_without_jsdoc(&captrues, &lines);
        new_file_contents.push(new_content);
    }

    store_data.last_action = Some(storefile::Action::Ex);
    store_data.files = changed_files;
    store_data.commit().expect("Failed to write to store file");

    for (i, target) in targets.clone().iter().enumerate() {
        fs::write(target.path(), &new_file_contents[i]).expect(&format!(
            "Failed to write to target file {:?}",
            &target.path()
        ))
    }
}

fn identify_js_doc(text: &str) -> (Vec<storefile::Capture>, Split<&str>) {
    let lines = text.split("\n");
    let mut captrues: Vec<storefile::Capture> = vec![];

    let mut start: Option<usize> = None;

    for (i, line) in lines.clone().enumerate() {
        let line = line.trim();

        if line == "/**" && start == None {
            start = Some(i);
        } else if line == "**/" && start != None {
            let content_lines: Vec<String> = lines.clone().map(String::from).collect();

            captrues.push(storefile::Capture {
                start: start.unwrap() + 1,
                end: i + 1,
                content: content_lines[start.unwrap()..i + 1].join("\n"),
            });

            start = None;
        }
    }

    return (captrues, lines);
}

fn get_content_without_jsdoc(file_captures: &Vec<Capture>, file_lines: &Vec<&str>) -> String {
    let mut caps = file_captures.iter();
    let mut cur_cap = caps.next();
    let mut new_lines = vec![];

    for i in 0..file_lines.len() {
        if cur_cap.is_some() && (i + 1) >= cur_cap.unwrap().start {
            if (i + 1) >= cur_cap.unwrap().end {
                cur_cap = caps.next()
            }
            continue;
        }
        new_lines.push(file_lines[i]);
    }

    let new_content = new_lines.join("\n");

    return new_content;
}
