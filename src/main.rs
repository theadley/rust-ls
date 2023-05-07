use chrono::offset::Local;
use chrono::DateTime;
use pretty_bytes::converter::convert;
use std::fs;
use std::os::unix::fs::MetadataExt;
use unix_mode;
use users::{get_group_by_gid, get_user_by_uid};

fn color_perms(input: &str) -> String {
    let mut output: String = "".to_owned();
    for c in input.chars() {
        match c {
            'd' => output.push_str(&format!("\x1b[96m{}\x1b[0m", c)),
            'r' => output.push_str(&format!("\x1b[38:5:117m{}\x1b[0m", c)),
            'w' => output.push_str(&format!("\x1b[93m{}\x1b[0m", c)),
            'x' => output.push_str(&format!("\x1b[31m{}\x1b[0m", c)),
            '-' => output.push_str(&format!("\x1b[90m{}\x1b[0m", c)),
            _ => output.push_str(&format!("{}", c)),
        }
    }
    return output;
}

fn left_pad_str(input: String, length: u8) -> String {
    let mut output: String = input.to_owned();
    while (output.len() as u8) < length {
        output = format!(" {}", output);
    }
    output
}

fn main() {
    if let Ok(entries) = fs::read_dir("./") {
        let paths: Vec<_> = entries.filter_map(|r| r.ok()).collect();

        let mut dirs: Vec<_> = paths
            .iter()
            .filter(|path| path.file_type().unwrap().is_dir())
            .collect();
        dirs.sort_by_key(|dir| dir.path());

        let mut files: Vec<_> = paths
            .iter()
            .filter(|path| !path.file_type().unwrap().is_dir())
            .collect();
        files.sort_by_key(|dir| dir.path());

        dirs.append(&mut files);

        for entry in dirs {
            if let Ok(meta) = fs::metadata(entry.path()) {
                let mode = meta.mode();
                let unix_mode_string = unix_mode::to_string(mode);
                let user = get_user_by_uid(meta.uid()).unwrap();
                let group = get_group_by_gid(meta.gid()).unwrap();
                let path: String = entry.path().to_str().unwrap().chars().skip(2).collect();

                if let Ok(datetime_modified) = meta.modified() {
                    let datetime: DateTime<Local> = DateTime::from(datetime_modified);
                    println!(
                            "{} \x1b[38:5:230m{}\x1b[0m \x1b[38:5:187m{}\x1b[0m \x1b[38:5:229m{}\x1b[0m \x1b[38:5:42m{}\x1b[0m {}",
                            color_perms(&unix_mode_string),
                            user.name().to_str().unwrap(),
                            group.name().to_str().unwrap(),
                            format!("{}", left_pad_str(convert(meta.len() as f64), 8)),
                            format!("{}", datetime.format("%a %b %e %H:%M:%S %Y")),
                            if meta.is_dir() {format!("\x1b[96m{}\x1b[0m", path)} else {format!("{}", path)}
                        )
                }
            }
        }
    }
}
