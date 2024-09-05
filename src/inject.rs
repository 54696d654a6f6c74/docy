use crate::{
    options,
    storefile::{self, File},
};
use std::fs;

pub fn run(mut store_data: storefile::StoreFile, options: options::Options) {
    if store_data.files.len() <= 0 {
        println!("No changes in store files. Exiting without doing any work...");
        return;
    }

    if store_data.last_action.is_some()
        && store_data.last_action.unwrap() == storefile::Action::In
        && !options.force_action
    {
        println!("Last call was inject. Exiting without doing any work...");
        return;
    }

    for file in &store_data.files {
        if options.verbose {
            println!("Injecting into: {:?}", file.path)
        }

        inject_into_file(file);
    }

    store_data.last_action = Some(storefile::Action::In);
    store_data
        .commit()
        .expect("Failed to update store file with latest action");
}

fn inject_into_file(file: &File) {
    let file_content =
        fs::read_to_string(&file.path).expect(&format!("Failed to open {:?}", &file.path));

    let file_lines = file_content.split("\n");

    let mut cur_cap = 0;
    let mut new_lines = vec![];
    let mut offset = 0;

    for (i, line) in file_lines.enumerate() {
        if file.captrues.len() > cur_cap && file.captrues[cur_cap].start - offset == (i + 1) {
            let injection = file.captrues[cur_cap].content.clone();
            offset += &injection.split("\n").collect::<Vec<&str>>().len();

            new_lines.push(injection);

            cur_cap += 1;
        }
        new_lines.push(line.to_string());
    }

    fs::write(&file.path, new_lines.join("\n"))
        .expect(&format!("Failed to inject into {:?}", &file.path));
}
