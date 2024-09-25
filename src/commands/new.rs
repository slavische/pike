use std::{env, fs, path::Path};

use include_dir::{include_dir, Dir, DirEntry};

static PLUGIN_TEMPLATE: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR/plugin_template");

fn place_file(target_path: &Path, t_ctx: &liquid::Object, entries: &[DirEntry<'_>]) {
    for entry in entries {
        match entry {
            DirEntry::Dir(inner_dir) => place_file(target_path, t_ctx, inner_dir.entries()),
            DirEntry::File(inner_file) => {
                let template = liquid::ParserBuilder::with_stdlib()
                    .build()
                    .unwrap()
                    .parse(inner_file.contents_utf8().unwrap())
                    .unwrap_or_else(|_| {
                        panic!("invalid template {}", inner_file.path().to_string_lossy())
                    });

                let dest_path = Path::new(&target_path).join(inner_file.path());
                if let Some(dest_dir) = dest_path.parent() {
                    if !dest_dir.exists() {
                        std::fs::create_dir_all(dest_dir).unwrap();
                    }
                }
                fs::write(&dest_path, template.render(&t_ctx).unwrap()).unwrap();
            }
        }
    }
}

pub fn cmd(path: Option<&Path>) {
    let path = match path {
        Some(p) => {
            if p.exists() {
                panic!("path {} already exists", p.to_string_lossy())
            };
            p.to_path_buf()
        }
        None => env::current_dir().unwrap(),
    };

    std::fs::create_dir_all(&path).unwrap();

    let templates_ctx = liquid::object!({
        "project_name": &path.file_name().unwrap().to_str().unwrap(),
    });

    place_file(&path, &templates_ctx, PLUGIN_TEMPLATE.entries());
}
