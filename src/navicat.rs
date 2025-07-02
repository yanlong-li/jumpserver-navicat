use rfd::{FileDialog, MessageDialogResult};
use std::fs::read_to_string;
use std::path::{Path, PathBuf};
use std::{env, fs};

pub fn get_navicat_path() -> Option<PathBuf> {
    let root_path = env::current_exe()
        .expect("获取当前路径失败")
        .parent()
        .expect("获取父级目录")
        .to_path_buf();

    let navicat_path_file = root_path.join("navicat.path");

    // 读取 navicat.path，如果失败则查找
    let path_str = read_to_string(&navicat_path_file)
        .ok()
        .map(|s| s.trim().to_string());

    match path_str {
        Some(path_str) => {
            let path = PathBuf::from(path_str);
            if path.exists() {
                Some(path)
            } else {
                fs::remove_file(navicat_path_file)
                    .expect("navicat.path中的内容失效，尝试删除文件但删除失败了~");
                get_navicat_path()
            }
        }
        None => match find_existing_navicat() {
            None => {
                let path = select_navicat();

                match path {
                    None => None,
                    Some(path) => {
                        save_navicat_path(navicat_path_file, &path).expect("保存navicat路径失败");

                        Some(path)
                    }
                }
            }
            Some(path) => Some(path),
        },
    }
}

fn save_navicat_path(navicat_path_file: PathBuf, navicat_path: &Path) -> std::io::Result<()> {
    fs::write(navicat_path_file, navicat_path.to_str().unwrap())
}

pub fn select_navicat() -> Option<PathBuf> {
    let res = rfd::MessageDialog::new()
        .set_title("启动 Navicat 失败")
        .set_description("请选择您安装的 Navicat.exe 位置")
        .set_level(rfd::MessageLevel::Warning)
        .set_buttons(rfd::MessageButtons::OkCancelCustom(
            "确认".to_string(),
            "取消".to_string(),
        ))
        .show();

    if res == MessageDialogResult::Ok {
        let program_file_dir = env::var("ProgramFiles").expect("获取 ProgramFiles 失败");
        let program_path = Path::new(&program_file_dir);

        FileDialog::new()
            .add_filter("Navicat", &["exe"])
            .set_directory(program_path.join("PremiumSoft"))
            .set_file_name("navicat.exe")
            .pick_file()
    } else {
        None
    }
}

fn find_existing_navicat() -> Option<PathBuf> {
    let program_file_dir = env::var("ProgramFiles").expect("获取 ProgramFiles 失败");

    let list = vec![
        "PremiumSoft\\Navicat Premium 17\\navicat.exe",
        "PremiumSoft\\Navicat Premium Lite 17\\navicat.exe",
        "PremiumSoft\\Navicat 17 for MySQL\\navicat.exe",
        "PremiumSoft\\Navicat Premium 16\\navicat.exe",
        "PremiumSoft\\Navicat 16 for MySQL\\navicat.exe",
    ];

    let program_path = Path::new(&program_file_dir);

    for template in list {
        let path = program_path.join(&template);
        if path.exists() {
            return Some(path);
        }
    }

    None
}

#[cfg(test)]
mod test {
    use crate::navicat::get_navicat_path;

    #[test]
    fn test() {
        assert!(get_navicat_path().is_some());
    }
}
