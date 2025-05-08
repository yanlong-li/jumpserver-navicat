use std::env;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::thread::sleep;
use std::time::Duration;

pub fn launcher(arg: &str) {
    let jms_client_path = find_existing_jms_client();

    match jms_client_path {
        None => {
            println!("未找到 JumpServerClient 安装目录");
            sleep(Duration::from_secs(5));
        }
        Some(path) => {
            Command::new(path)
                .arg(arg)
                .spawn()
                .expect("启动 JumpServerClient.exe 失败");
            
            
            sleep(Duration::from_secs(5));
        }
    }
}

fn find_existing_jms_client() -> Option<PathBuf> {
    let program_files_dir = env::var("ProgramFiles").expect("获取 ProgramFiles 失败");
    let user_profile_dir = env::var("UserProfile").expect("获取 AppData 失败");

    let binding = env::current_exe()
        .expect("获取当前路径失败");
    let root_path = binding
        .parent()
        .expect("获取父级目录");

    let program_path = Path::new(&program_files_dir);
    let user_profile_dir = Path::new(&user_profile_dir);

    // JumpServerClient
    let list = vec![
        user_profile_dir.join("AppData\\Roaming\\JumpServer\\client\\bin\\JumpServerClient.exe"),
        user_profile_dir.join("AppData\\Local\\Programs\\JumpServerClient\\resources\\bin\\windows\\JumpServerClient.exe"),
        user_profile_dir.join("AppData\\Local\\Programs\\JumpServerClient\\resources\\windows\\JumpServerClient.exe"),
        program_path.join("JumpServerClient\\resources\\bin\\windows\\JumpServerClient.exe"),
        program_path.join("JumpServerClient\\resources\\windows\\JumpServerClient.exe"),
        root_path.join("JumpServerClient2.exe"),
    ];

    for path in list {
        if path.exists() {
            return Some(path);
        }
    }

    None
}

#[cfg(test)]
mod test {
    use crate::protocol::other::find_existing_jms_client;

    #[test]
    fn test() {
        assert!(find_existing_jms_client().is_some());
    }
}
