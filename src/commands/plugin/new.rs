use anyhow::{bail, Context, Result};
use std::{env, ffi::OsStr, fs, path::Path, process::Command};

use include_dir::{include_dir, Dir, DirEntry};

static PLUGIN_TEMPLATE: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR/plugin_template");

fn place_file(target_path: &Path, t_ctx: &liquid::Object, entries: &[DirEntry<'_>]) -> Result<()> {
    for entry in entries {
        match entry {
            DirEntry::Dir(inner_dir) => place_file(target_path, t_ctx, inner_dir.entries())?,
            DirEntry::File(inner_file) => {
                let template = liquid::ParserBuilder::with_stdlib()
                    .build()
                    .context("couldn't build from template")?
                    .parse(
                        inner_file
                            .contents_utf8()
                            .context("couldn't extract file contents")?,
                    )
                    .context(format!(
                        "invalid template {}",
                        inner_file.path().to_string_lossy()
                    ))?;

                let dest_path = Path::new(&target_path).join(inner_file.path());
                if let Some(dest_dir) = dest_path.parent() {
                    if !dest_dir.exists() {
                        std::fs::create_dir_all(dest_dir)?;
                    }
                }
                fs::write(
                    &dest_path,
                    template
                        .render(&t_ctx)
                        .context("failed to render the file")?,
                )
                .context(format!("couldn't write to {}", dest_path.display()))?;
            }
        }
    }

    Ok(())
}

fn git<I, S>(args: I) -> Result<()>
where
    I: IntoIterator<Item = S>,
    S: AsRef<OsStr>,
{
    Command::new("git")
        .args(args)
        .output()
        .context("failed to run git command, install git first")?;

    Ok(())
}

pub fn cmd(path: Option<&Path>, init_git: bool) -> Result<()> {
    let path = match path {
        Some(p) => {
            if p.exists() {
                bail!("path {} already exists", p.to_string_lossy())
            };
            p.to_path_buf()
        }
        None => env::current_dir()?,
    };

    std::fs::create_dir_all(&path).context(format!("failed to create {}", path.display()))?;

    let templates_ctx = liquid::object!({
        "project_name": &path.file_name().context("extracting project name")?.to_str().context("parsing filename to string")?,
    });

    place_file(&path, &templates_ctx, PLUGIN_TEMPLATE.entries())
        .context("error placing the template")?;

    // init git in plugin repository
    if init_git {
        let project_path = path.to_str().context("failed to extract project path")?;
        git(["-C", project_path, "init"])?;
        git(["-C", project_path, "add", "."])?;
    }

    Ok(())
}
