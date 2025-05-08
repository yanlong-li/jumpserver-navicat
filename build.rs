use std::env;

fn main() {
    // 获取构建模式
    let profile = env::var("PROFILE").unwrap_or_else(|_| String::from("debug"));
    

    if profile == "release" {
        println!("cargo:rerun-if-changed=app.manifest");
        let mut res = winres::WindowsResource::new();
        res.set_manifest_file("app.manifest");
        // **应用名称、公司信息、版本信息**
        res.set("FileDescription", "JumpServerClientNavicat"); // 应用描述
        res.set("ProductName", "JumpServerClientNavicat"); // 应用名称
        res.set("CompanyName", "yanlongli"); // 作者/公司信息
        res.set("LegalCopyright", "Copyright © 2025 yanlongli"); // 版权信息
        res.compile().unwrap();
    }
}
