use std::fs::File;
use std::io::prelude::*;

crate fn append_dir(l: &[String], root: bool) {
    let mut html = File::create("dir_toy_server.html").unwrap();
    let mut content = String::new();

    content.push_str("<!DOCTYPE HTML>\n");
    content.push_str("<html>\n");
    content.push_str("<body>\n<h1>Files list<h1>\n");
    content.push_str("<hr>\n<ul>\n");

    if !root {
        content.push_str(&get_parent(l.first()));
    }

    for link in l {
        if link.contains("dir_toy_server.html") {
            continue;
        }
        let mut name = link.clone();
        if name.ends_with('/') {
            name.pop();
        }

        let idx = name.rfind('/').unwrap_or(0);

        name.replace_range(..idx, "");

        name = name.replace('/', "");
        content.push_str(&format!("<li><a href='/{}'>{}</a></li>\n", link, name));
    }
    content.push_str("</ul>\n<hr>\n</body>\n</html>\n");
    writeln!(html, "{}", &content);
}
fn get_parent(s: Option<&String>) -> String {
    let result = match s {
        Some(s) => {
            let mut s = s.to_string();
            if s.ends_with('/') {
                s.pop();
            }
            let idx = s.rfind('/').unwrap_or(0);
            s.replace_range(idx.., "");
            match s.rfind('/') {
                Some(idx) => {
                    s.replace_range(idx.., "");
                    s.insert(0, '/');
                }
                None => s = ".".to_string(),
            }
            s
        }
        None => ".".to_string(),
    };

    format!("<li><a href='{}'>..</a></li>\n", &result)
}
