use std::collections::BTreeSet;
use std::ffi::OsStr;
use std::fs;
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use std::process::{Command, Output, Stdio};

use super::markdown::Page;

const LEFT_OUT_FOLDERS: &[&str] = &[
    "node_modules",
    "vendor",
    "third_party",
    "target",
    "build",
    "dist",
    "out",
    "Pods",
    "fixtures",
    "__fixtures__",
    "__snapshots__",
    "testdata",
    "test",
    "tests",
    "e2e",
    "evals",
];
const DOC_EXTENSIONS: &[&str] = &["md", "mdx", "markdown"];
const LICENSE_STEMS: &[&str] = &["LICENSE", "LICENCE", "COPYING"];

pub struct Project {
    pub root: PathBuf,
    pub files: BTreeSet<String>,
    pub folders: BTreeSet<String>,
    pub git: bool,
}

fn folders_of(files: &BTreeSet<String>) -> BTreeSet<String> {
    let mut folders = BTreeSet::new();
    for file in files {
        let mut rest = file.as_str();
        while let Some((parent, _)) = rest.rsplit_once('/') {
            folders.insert(parent.to_string());
            rest = parent;
        }
    }
    folders
}

fn walk(dir: &Path, root: &Path, files: &mut BTreeSet<String>) -> io::Result<()> {
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        let name = entry.file_name();
        let name = name.to_string_lossy();
        if entry.file_type()?.is_dir() {
            let hidden = name.starts_with('.') && name != ".cursor" && name != ".github";
            if !hidden && !LEFT_OUT_FOLDERS.contains(&name.as_ref()) {
                walk(&path, root, files)?;
            }
        } else {
            let relative = path.strip_prefix(root).unwrap_or(&path);
            files.insert(relative.to_string_lossy().into_owned());
        }
    }
    Ok(())
}

fn is_hidden_doc(path: &str) -> bool {
    let instruction = path.contains(".cursor/rules/") || path.ends_with("copilot-instructions.md");
    !instruction && path.split('/').any(|part| part.starts_with('.'))
}

fn is_doc(path: &str) -> bool {
    let path_ref = Path::new(path);
    let extension = path_ref.extension().and_then(OsStr::to_str).unwrap_or("");
    let stem = path_ref.file_stem().and_then(OsStr::to_str).unwrap_or("");
    let cursor_rule = extension == "mdc" && path.contains(".cursor/rules/");
    let folders = path.rsplit_once('/').map_or("", |(folders, _)| folders);
    (cursor_rule || DOC_EXTENSIONS.contains(&extension))
        && !LICENSE_STEMS.contains(&stem.to_uppercase().as_str())
        && !folders
            .split('/')
            .any(|part| LEFT_OUT_FOLDERS.contains(&part))
        && !is_hidden_doc(path)
}

fn is_excluded(path: &str, exclude: &[String]) -> bool {
    exclude.iter().any(|prefix| {
        let prefix = prefix.trim_start_matches("./").trim_matches('/');
        path == prefix || path.starts_with(&format!("{prefix}/"))
    })
}

impl Project {
    pub fn load(root: &Path) -> Result<Self, String> {
        let root = root.to_path_buf();
        let tracked = git_files(&root);
        let git = tracked.is_some();
        let files = match tracked {
            Some(files) => files,
            None => {
                let mut files = BTreeSet::new();
                walk(&root, &root, &mut files)
                    .map_err(|error| format!("can't read {}: {error}", root.display()))?;
                files
            }
        };
        let folders = folders_of(&files);
        Ok(Project {
            root,
            files,
            folders,
            git,
        })
    }

    pub fn read_pages(&self, exclude: &[String]) -> Result<Vec<Page>, String> {
        self.files
            .iter()
            .filter(|path| is_doc(path) && !is_excluded(path, exclude))
            .filter(|path| self.root.join(path).is_file())
            .map(|path| {
                let bytes = fs::read(self.root.join(path))
                    .map_err(|error| format!("can't read {path}: {error}"))?;
                Ok(Page::new(
                    path,
                    String::from_utf8_lossy(&bytes).into_owned(),
                ))
            })
            .collect()
    }

    pub fn git(&self, args: &[&str]) -> Option<Output> {
        git_command(&self.root, args).ok()
    }

    pub fn git_with_input(&self, args: &[&str], input: &str) -> Option<Output> {
        let mut child = git_base(&self.root, args)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::null())
            .spawn()
            .ok()?;
        child.stdin.take()?.write_all(input.as_bytes()).ok()?;
        child.wait_with_output().ok()
    }

    pub fn has_history(&self) -> bool {
        self.git
            && self
                .git(&["rev-parse", "--verify", "--quiet", "HEAD"])
                .is_some_and(|output| output.status.success())
    }

    pub fn is_ignored(&self, path: &str) -> bool {
        self.git
            && self
                .git(&["check-ignore", "-q", "--no-index", "--", path])
                .is_some_and(|output| output.status.success())
    }

    pub fn has_file_or_folder(&self, path: &str) -> bool {
        let tail = format!("/{path}");
        self.files.contains(path)
            || self.folders.contains(path)
            || self.files.iter().any(|file| file.ends_with(&tail))
            || self.folders.iter().any(|folder| folder.ends_with(&tail))
    }
}

fn git_base(root: &Path, args: &[&str]) -> Command {
    let mut command = Command::new("git");
    command
        .arg("-C")
        .arg(root)
        .args(["-c", "core.fsmonitor=false"])
        .args(args)
        .env("GIT_OPTIONAL_LOCKS", "0")
        .env("GIT_NO_LAZY_FETCH", "1");
    command
}

fn git_command(root: &Path, args: &[&str]) -> io::Result<Output> {
    git_base(root, args).stdin(Stdio::null()).output()
}

fn git_files(root: &Path) -> Option<BTreeSet<String>> {
    let args = [
        "ls-files",
        "-z",
        "--cached",
        "--others",
        "--exclude-standard",
    ];
    let output = git_command(root, &args).ok()?;
    if !output.status.success() {
        return None;
    }
    let list = String::from_utf8_lossy(&output.stdout);
    Some(
        list.split('\0')
            .filter(|path| !path.is_empty())
            .map(str::to_string)
            .collect(),
    )
}
