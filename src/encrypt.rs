use std::env;
use std::fs::{read_to_string, remove_file, write};
use std::process::Command;

use crate::php::get_php_str;

pub fn encrypt(password: &str) -> String {
    let php = get_php_str();

    let root_path = env::current_exe().expect("获取当前路径失败").parent().expect("获取父级目录").to_str().expect("转换为字符串失败").to_string();

    // 检查文件 phppath.txt 是否存在
    let phppath = read_to_string(format!("{}\\php.path", root_path));

    let phppath = match phppath {
        Ok(v) => v.to_string(),
        Err(_) => String::from("php"),
    };

    write(format!("{}\\encrypt.php", root_path), php).expect("写入加密文件失败");

    let a = Command::new(phppath)
        .arg(format!("{}\\encrypt.php", root_path))
        .arg(password)
        .output()
        .expect("加密失败");

    let _ = remove_file(format!("{}\\encrypt.php", root_path));

    let ass = String::from_utf8(a.stdout).unwrap();

    return ass;
}

#[cfg(test)]
mod test {
    use crate::encrypt::encrypt;

    #[test]
    fn test() {
        assert_eq!(encrypt("123456"), "123456")
    }
}