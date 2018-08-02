use std::fs::File;
use std::io::prelude::*;

crate fn append_dir(l: &[String], root: bool) {
    let mut html = File::create("dir.html").unwrap();
    let mut content = String::new();

    content.push_str("<!DOCTYPE HTML>\n");
    content.push_str("<html>\n");
    content.push_str("<body>\n<h1>Files list<h1>\n");
    content.push_str("<hr>\n<ul>\n");
    if !root {
        content.push_str("<li><a href='../'>..</a></li>\n");
    }

    for link in l {
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
