use std::env;
use std::fs::{remove_file, File};
use std::process::Command;

use byteorder::{LittleEndian, WriteBytesExt};

pub fn get_reg_str() -> String {
    String::from(
        r#"Windows Registry Editor Version 5.00

[HKEY_CLASSES_ROOT\jms]
@="URL:jms Protocol"
"URL Protocol"=""

[HKEY_CLASSES_ROOT\jms\shell]

[HKEY_CLASSES_ROOT\jms\shell\open]

[HKEY_CLASSES_ROOT\jms\shell\open\command]
@="\"{{program}}\" \"%1\""

"#,
    )
}

pub fn detecting() -> bool {
    // 修改命令行编码
    Command::new("cmd")
        .arg("/C")
        .arg("chcp")
        .arg("65001")
        .output()
        .expect("<UNK>");
    let output = Command::new("reg")
        .arg("query")
        .arg("HKEY_CLASSES_ROOT\\jms\\shell\\open\\command")
        .output()
        .expect("查询失败");

    let mut ok = false;

    if output.status.success() {
        let binding = output.stdout.to_vec();
        let out_msg = String::from_utf8(binding).unwrap();
        // println!("ok {:?}", out_msg.0);
        let program = env::current_exe().expect("获取路径失败");

        let s = program.to_str().unwrap();

        ok = out_msg.contains(&s);

        if !ok {
            println!("jms:// 协议地址不匹配，重新注册")
        }
    } else {
        let binding = output.stderr.to_vec();
        let out_msg = String::from_utf8(binding);
        println!("err {:?}", out_msg);
    }

    ok
}

pub fn register() -> bool {
    let root_path = env::current_exe()
        .expect("获取当前路径失败")
        .parent()
        .expect("获取父级目录")
        .to_str()
        .expect("转换为字符串失败")
        .to_string();

    let program = env::current_exe().expect("获取路径失败");

    let s = program.to_str().unwrap().replace("\\", "\\\\");

    let reg_str = get_reg_str().replace("{{program}}", &s);

    let reg_path = format!("{}\\protocol.reg", root_path);

    {
        let v: Vec<u16> = reg_str.encode_utf16().collect();
        let mut file = File::create(&reg_path).unwrap();
        file.write_u16::<LittleEndian>(0xFEFF).unwrap();
        for i in 0..v.len() {
            file.write_u16::<LittleEndian>(v[i]).unwrap();
        }
    }

    println!("{}", "导入注册表数据");
    let output = Command::new("reg")
        .arg("import")
        .arg(&reg_path)
        .output()
        .expect("导入注册表失败");

    // 将输出转换为字符串

    if output.status.success() {
        let binding = output.stdout.to_vec();
        println!("{:?}", String::from_utf8(binding));
    } else {
        let binding = output.stderr.to_vec();
        println!("{:?}", String::from_utf8(binding));
    }

    // 删除文件
    let res = remove_file(reg_path);
    println!("清理临时数据 {:?}", res);

    output.status.success()
}

#[cfg(test)]
mod tests {
    use std::process::Command;

    #[test]
    fn it_works() {
        Command::new("cmd")
            .arg("/C")
            .arg("chcp")
            .arg("65001")
            .output()
            .expect("<UNK>");
        let output = Command::new("reg")
            .arg("query")
            .arg("HKEY_CLASSES_ROOT\\jms\\shell\\open\\command")
            .output()
            .expect("查询失败");

        // let out_msg = UTF_8.decode(&output.stdout);
        println!("{:?}", String::from_utf8(output.stdout));
    }
}
