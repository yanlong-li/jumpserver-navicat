use std::env;
use std::fs::read_to_string;

pub fn get_reg_str() -> String {
    let root_path = env::current_exe().expect("获取当前路径失败").parent().expect("获取父级目录").to_str().expect("转换为字符串失败").to_string();


    let reg_result = read_to_string(format!("{}\\reg.reg", root_path));

    match reg_result {
        Ok(val) => {
            val
        }
        Err(_) => {
            String::from(r#"Windows Registry Editor Version 5.00

[HKEY_CURRENT_USER\Software\PremiumSoft\Navicat\Servers\{{name}}]
"ServiceProvider"="spDefault"
"Host"="{{host}}"
"Port"=dword:{{port}}
"UserName"="{{username}}"
"AskPassword"="false"
"Pwd"="{{password}}"
"AutoConnect"=dword:00000001
"Codepage"=dword:0000fde9
"UseCharacterSet"=dword:00000001
"ClientCharacterSet"=""
"ClientEncoding"=dword:0000fde9
"ClientDriverVersion"="cdvDefault"
"UseCompression"=dword:00000000
"UsePingInterval"=dword:00000001
"PingInterval"=dword:000000f0
"UseConnectionTimeout"=dword:00000001
"ConnectionTimeoutSeconds"=dword:0000001e
"UseReadTimeout"=dword:00000000
"ReadTimeoutSeconds"=dword:0000001e
"UseWriteTimeout"=dword:00000001
"WriteTimeoutSeconds"=dword:0000001e
"UseNamedPipe"=dword:00000000
"NamedPipeSocket"=""
"SessionLimit"=dword:00000000
"UseAdvSettings"="false"
"UseSSL"=dword:00000000
"UseSSLAuthen"=dword:00000000
"ClientKey"=""
"ClientCert"=""
"ClientKeyPassword"=""
"SaveClientKeyPassword"=dword:00000001
"CACert"=""
"Cipher"=""
"VerifyCACert"=dword:00000000
"WeakCertValidation"=dword:00000001
"AllowInvalidHostName"=dword:00000000
"RootCert"=""
"PGSSLCRL"=""
"ServerNameIndication"=""
"PGSSLMode"="smRequire"
"UseSSH"=dword:00000000
"SSH_Host"=""
"SSH_Port"=dword:00000016
"SSH_UserName"=""
"SSH_SavePassword"=dword:00000001
"SSH_Password"=""
"SSH_SavePassphrase"=dword:00000001
"SSH_Passphrase"=""
"SSH_PrivateKey"=""
"SSH_AuthenMethod"="saPassword"
"UseHTTP"=dword:00000000
"HTTP_URL"=""
"HTTP_EncodeBase64"=dword:00000000
"HTTP_Authen"=dword:00000000
"HTTP_Username"=""
"HttpSavePassword"=dword:00000001
"HTTP_Password"=""
"HTTP_CertAuth"=dword:00000000
"HTTP_ClientKey"=""
"HTTP_ClientCert"=""
"HTTP_CACert"=""
"HTTP_Passphrase"=""
"HTTP_Proxy"=dword:00000000
"HTTP_ProxyHost"=""
"HTTP_ProxyPort"=dword:00001f90
"HTTP_ProxyUsername"=""
"HttpProxySavePassword"=dword:00000001
"HTTP_ProxyPassword"=""
"UseCompatibilityMode"=dword:00000000
"Compatibility_OverrideLowerCaseTableNames"=dword:00000000
"Compatibility_LowerCaseTableNames"=dword:00000000
"Compatibility_OverrideSQLMode"=dword:00000000
"Compatibility_SQLMode"="ONLY_FULL_GROUP_BY, STRICT_TRANS_TABLES, NO_ZERO_IN_DATE, NO_ZERO_DATE, ERROR_FOR_DIVISION_BY_ZERO, NO_ENGINE_SUBSTITUTION"
"Compatibility_OverrideIsSupportNDB"=dword:00000000
"Compatibility_IsSupportNDB"=dword:00000000
"Compatibility_OverrideDatabaseListingMethod"=dword:00000000
"Compatibility_DatabaseListingMethod"="ShowDatabasesStatement"
"Compatibility_OverrideViewListingMethod"=dword:00000000
"Compatibility_ViewListingMethod"="ShowFullTablesStatement"
"QuerySavePath"="{{user_profile}}\\Documents\\Navicat\\MySQL\\Servers\\{{name}}"
"ActiveServerSettings"=""
"CreateTime"=hex(b):44,67,02,65,00,00,00,00
"ModifyTime"=hex(b):44,67,02,65,00,00,00,00
"NSYCreatedUser"=""
"NSYModifiedUser"=""
"NSYModifiedUserUUID"=""
"NSYCreatedUnixTimestamp"=hex(b):00,00,00,00,00,00,00,00
"NSYModifiedUnixTimestamp"=hex(b):00,00,00,00,00,00,00,00
"CommitUUID"=""
"ConnectionUUID"=""
"UnknownJsonPropertiesString"=""
"CloudSyncStatus"="Ready"

[HKEY_CURRENT_USER\Software\PremiumSoft\Navicat\Servers\{{name}}\Profiles]

"#)
        }
    }
}