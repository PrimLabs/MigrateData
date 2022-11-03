use isp_sdk::icsp;
use std::io::Write;

static OLD_ICSP_CANISTER_TEXT: &'static str = "4radi-oqaaa-aaaan-qapwa-cai";
static NEW_ICSP_CANISTER_TEXT: &'static str = "csdii-qiaaa-aaaan-qazpq-cai";
static PEM_STR: &'static str = "identities/identity.pem";

#[tokio::main]
async fn main() {
    let put_args: Vec<(String, String, bool)> = read_data().await;
    write_data(put_args).await;
    check().await;
}

async fn read_data() -> Vec<(String, String, bool)> {
    let file_keys: Vec<String> = icsp::get_all_ic_file_key(PEM_STR, OLD_ICSP_CANISTER_TEXT).await;
    let mut index: u64 = 0;
    let mut put_args: Vec<(String, String, bool)> = Vec::new();
    for key in &file_keys {

        let file_info = icsp::get_file_info(PEM_STR, OLD_ICSP_CANISTER_TEXT, key.clone()).await.unwrap();

        let (data, file_type): (Vec<u8>, String) = icsp::get_file(PEM_STR, OLD_ICSP_CANISTER_TEXT, key.as_str()).await;

        let file_name = generate_file_name(index,file_type_to_extension(file_type.as_str()));

        put_args.push((file_name.clone(), key.clone(), file_info.is_http_open));

        let mut file = std::fs::File::create(file_name.as_str()).expect("create failed");
        file.write_all(&data).expect("write failed");
        index += 1;
    };
    put_args
}

async fn write_data(put_args: Vec<(String, String, bool)>) {
    for arg in &put_args {
        println!("complete store file: {:?}", icsp::store_file_by_key(PEM_STR, arg.0.as_str(), NEW_ICSP_CANISTER_TEXT, arg.2, arg.1.clone()).await.0);
    }
}

async fn check() {
    let file_keys: Vec<String> = icsp::get_all_ic_file_key(PEM_STR, OLD_ICSP_CANISTER_TEXT).await;

    let mut index: u64 = 0;
    for key in &file_keys {
        let data_1 = icsp::get_file(PEM_STR, OLD_ICSP_CANISTER_TEXT, key.as_str()).await.0;
        let data_2 = icsp::get_file(PEM_STR, NEW_ICSP_CANISTER_TEXT, key.as_str()).await.0;
        if data_1 != data_2 {
            panic!("error! file: {:?}, file_key: {:?}", index, key.clone());
        };
        println!("check file: {:?}, file_key: {:?}", index, key.clone());
        index += 1;
    }
}

fn generate_file_name(index: u64,file_type: &str) -> String {
    let mut file_name = "source/".to_string();
    file_name.push_str(index.to_string().as_str());
    file_name.push('.');
    file_name.push_str(file_type);
    file_name
}

fn file_type_to_extension(file_type: &str) -> &str {
    if file_type == "application/pdf" {
        return "pdf";
    } else if file_type == "image/jpg" {
        return "jpg";
    } else if file_type == "image/png" {
        return "png";
    } else if file_type == "video/mp4" {
        return "mp4";
    } else if file_type == "audio/mp3" {
        return "mp3";
    } else if file_type == "image/gif" {
        return "gif";
    } else if file_type == "text/plain" {
        return "txt";
    } else if file_type == "application/vnd.ms-powerpoint" {
        return "pptx";
    } else if file_type == "text/html" {
        return "html";
    } else if file_type == "application/msword" {
        return "docx";
    } else if file_type == "application/x-xls" {
        return "xls";
    } else if file_type == "application/vnd.android.package-archive" {
        return "apk"
    } else if file_type == "text/xml" {
        return "svg";
    } else if file_type == "video/x-ms-wmv" {
        return "wmv";
    } else {
        return "*";
    };
}