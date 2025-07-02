use crate::encrypt::encrypt;
use crate::navicat::get_navicat_path;
use crate::reg::get_mysql_reg_str;
use crate::Config;
use byteorder::{LittleEndian, WriteBytesExt};
use hex::ToHex;
use std::env;
use std::fs::{remove_file, File};
use std::process::Command;
use std::thread::sleep;

pub fn launcher(config: &Config) {
    let root_path = env::current_exe()
        .expect("获取当前路径失败")
        .parent()
        .expect("获取父级目录")
        .to_str()
        .expect("转换为字符串失败")
        .to_string();

    println!("{}", "准备注册表");
    let reg_str = get_mysql_reg_str();

    println!("{}", "加密密码");
    let ass = encrypt(&config.token.value);

    println!("{}", "写注册表数据临时文件");

    let port = config.endpoint.port.to_be_bytes();

    let port_str: String = port.encode_hex();

    let user_profile = env::var("UserProfile").expect("获取用户环境变量失败");

    let reg_str = reg_str
        .replace("{{name}}", &config.asset.name)
        .replace("{{user_profile}}", &user_profile.replace("\\", "\\\\"))
        .replace("{{host}}", &config.endpoint.host)
        .replace("{{port}}", &port_str)
        .replace("{{username}}", &config.token.id)
        .replace("{{password}}", &ass.as_str());

    let reg_path = format!("{}\\tmp.reg", root_path);

    {
        let v: Vec<u16> = reg_str.encode_utf16().collect();
        let mut file = File::create(&reg_path).unwrap();
        file.write_u16::<LittleEndian>(0xFEFF).unwrap();
        for i in 0..v.len() {
            file.write_u16::<LittleEndian>(v[i]).unwrap();
        }
    }

    println!("{}", "导入注册表数据");
    Command::new("reg")
        .arg("import")
        .arg(&reg_path)
        .output()
        .expect("导入注册表失败");

    // 删除文件
    let _ = remove_file(format!("{}\\tmp.reg", root_path));

    println!("{}", "启动 NativeClient");

    // 检查文件 phppath.txt 是否存在
    let navicat_path = get_navicat_path();

    if let Some(navicat_path) = navicat_path {
        Command::new(navicat_path).spawn().expect("启动失败");

        sleep(std::time::Duration::from_secs(5));
        println!("{}", "清理注册表");
        Command::new("reg")
            .arg("delete")
            .arg(format!(
                "HKEY_CURRENT_USER\\Software\\PremiumSoft\\Navicat\\Servers\\{}",
                config.asset.name
            ))
            .arg("/f")
            .output()
            .expect("删除注册表失败");

        println!(
            "{}",
            format!(
                "HKEY_CURRENT_USER\\Software\\PremiumSoft\\Navicat\\Servers\\{}",
                config.asset.name
            )
        );
    } else {
        println!("查找 Navicat 安装位置失败");
        sleep(std::time::Duration::from_secs(5));
    }
}
