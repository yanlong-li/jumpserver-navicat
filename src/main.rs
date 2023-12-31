use std::{env};
use std::env::args;
use std::fs::{File, read_to_string, remove_file};
use std::process::Command;
use std::thread::sleep;

use base64::{Engine, engine::general_purpose};
use byteorder::{LittleEndian, WriteBytesExt};
use hex::ToHex;
use serde::Deserialize;

use crate::encrypt::encrypt;
use crate::reg::get_reg_str;

mod reg;

mod encrypt;

#[derive(Deserialize, Debug)]
struct Config {
    // version: u8,
    // id: String,
    // value: String,
    // name: String,
    protocol: String,
    token: Token,
    // command: String,
    asset: Asset,
    endpoint: Endpoint,
}

#[derive(Deserialize, Debug)]
struct Token {
    id: String,
    value: String,
}

#[derive(Deserialize, Debug)]
struct Asset {
    // id: String,
    // category: String,
    name: String,
    // address: String,
}


#[derive(Deserialize, Debug)]
struct Endpoint {
    host: String,
    port: u32,
}


fn main() {
    let root_path = env::current_exe().expect("获取当前路径失败").parent().expect("获取父级目录").to_str().expect("转换为字符串失败").to_string();

    // println!("exe 目录: {}", root_path);

    // 从 args 中取第二个参数
    let arg = args().nth(1).unwrap_or_else(|| "0".to_string());

    // 去除 args 的 前 6个字符
    let arg_str = arg.replace("jms://", "").replace("/", "");

    // println!("{}", arg_str);

    // sleep(std::time::Duration::from_secs(5));
    println!("{}", "解析数据");
    // base64解码
    let json_str = general_purpose::STANDARD.decode(arg_str.to_string()).unwrap();

    let config = serde_json::from_slice::<Config>(&json_str).unwrap();

    // println!("{:#?}", config);

    match config.protocol.as_str() {
        "mysql" => {
            println!("{}", "准备注册表");
            let reg_str = get_reg_str();

            println!("{}", "加密密码");
            let ass = encrypt(&config.token.value);

            println!("{}", "写注册表数据临时文件");

            let port = config.endpoint.port.to_be_bytes();

            let port_str: String = port.encode_hex();

            let user_profile = env::var("UserProfile").expect("获取用户环境变量失败");

            let reg_str = reg_str.replace("{{name}}", &config.asset.name)
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
            let navicat_path = read_to_string(format!("{}\\navicat.path", root_path));

            let navicat_path = match navicat_path {
                Ok(v) => v.to_string(),
                Err(_) => {
                    let program_file_dir = env::var("ProgramFiles").expect("获取ProgramFiles失败");
                    format!("{}\\PremiumSoft\\Navicat Premium 16\\navicat.exe", program_file_dir)
                }
            };

            Command::new(navicat_path).spawn().expect("启动失败");

            sleep(std::time::Duration::from_secs(5));
            println!("{}", "清理注册表");
            Command::new("reg")
                .arg("delete")
                .arg(format!("HKEY_CURRENT_USER\\Software\\PremiumSoft\\Navicat\\Servers\\{}", config.asset.name))
                .arg("/f")
                .output()
                .expect("删除注册表失败");

            println!("{}", format!("HKEY_CURRENT_USER\\Software\\PremiumSoft\\Navicat\\Servers\\{}", config.asset.name));
        }
        _ => {
            Command::new(format!("{}\\JumpServerClient2.exe", root_path)).arg(arg).spawn().expect("启动失败");
        }
    }
}