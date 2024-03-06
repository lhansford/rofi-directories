use std::fs;
use std::env;
use std::process;

const ROOTS: &'static [&'static str] = &["/home/luke/Dropbox", "/home/luke/Downloads", "/home/luke/Documents"];

fn get_directories(dir: &str) -> Vec<String> {
    let mut directory_list = Vec::new();
    let entries = fs::read_dir(dir).unwrap();

    for entry in entries {
        let path = entry.unwrap().path();
        if path.is_dir() {
            directory_list.push(path.display().to_string());
        }
    }

    return directory_list;
}

fn get_directories_recursively(dir: &str, level: u8) -> Vec<String> {
    if level > 2 {
        return Vec::new();
    }

    let mut directory_list = Vec::new();
    directory_list.push(dir.to_string());

    let directories = get_directories(dir);

    for path in directories {
        for sub_path in get_directories_recursively(&path, level + 1) {
            directory_list.push(sub_path);
        }
    }

    return directory_list;
}

fn render_entry(dir: &str) -> String {
    return format!("<span>{}</span>\0icon\x1ffolder\n", dir);
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        let dir = str::replace(&args[1], "<span>", "");
        let dir = str::replace(&dir, "</span>", "");
        process::Command::new("xdg-open")
            .arg(dir)
            .output()
            .expect("???");
        process::exit(0);
    } else {
        println!("\0markup-rows\x1ftrue\n");

        let mut dir_strings: Vec<String> = Vec::new();
        for root in ROOTS {
            let directories: Vec<String> = get_directories_recursively(root, 0)
                .iter()
                .map(|dir| render_entry(dir))
                .collect();
            for directory in directories {
                dir_strings.push(directory);
            }
        }


        println!("{}", dir_strings.join("\n"));
    }
}

