use base64::{engine::general_purpose, Engine};
use serde::Deserialize;
use std::env::args;
use std::io::{stdout, Write};
use std::thread::sleep;
use std::time::Duration;

mod reg;

mod encrypt;

mod navicat;

mod protocol;
mod regedit;
mod uac;

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
    // 从 args 中取第二个参数
    let arg = args().nth(1).unwrap_or_else(|| "0".to_string());

    // 检查启动方式

    if !arg.starts_with("jms://") {
        println!("正在检测协议配置");
        if !regedit::detecting() {
            // 申请UAC权限
            uac::handle();
            regedit::register();
        } else {
            println!("检测正常，您现在可以通过jms网页来启动数据库连接了");
        }

        for i in 0..10 {
            print!("\r程序将在 {} 秒后结束......", 10 - i);
            stdout().flush().unwrap();
            sleep(Duration::from_secs(1));
        }
    } else {
        // 去除 args 的 前 6个字符
        let arg_str = arg.replace("jms://", "").replace("/", "");

        println!("{}", "解析数据");
        // base64解码
        let json_str = general_purpose::STANDARD
            .decode(arg_str.to_string())
            .unwrap();

        let config = serde_json::from_slice::<Config>(&json_str).unwrap();

        match config.protocol.as_str() {
            "mysql" => protocol::mysql::launcher(&config),
            _ => {
                println!("未支持的协议:{}，正在启动JMS原始客户端", config.protocol);
                // sleep(Duration::from_secs(1));
                protocol::other::launcher(&arg)
            }
        }
    }
}
