use std::env;
use std::fs::read_to_string;
use std::path::{Path, PathBuf};

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

    match path_str.is_some() {
        true => Some(PathBuf::from(navicat_path_file)),
        false => find_existing_navicat(),
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
