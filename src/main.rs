use futures::future::join_all;
use isp_sdk::icsp;
use itertools::Itertools;
use regex::Regex;
use std::io::Write;
use tokio::task::JoinError;

static OLD_ICSP_CANISTER_TEXT: &'static str = "4radi-oqaaa-aaaan-qapwa-cai";
static NEW_ICSP_CANISTER_TEXT: &'static str = "csdii-qiaaa-aaaan-qazpq-cai";
static PEM_STR: &str = "identities/identity.pem";

#[tokio::main]
async fn main() {
    let put_args = read_data().await;
    println!("读完毕");
    write_data(put_args).await;
    println!("写完毕");
}

async fn read(key: String) -> (String, String, bool) {
    // 某个key下的文件信息
    let file_info = icsp::get_file_info(PEM_STR, OLD_ICSP_CANISTER_TEXT, key.to_owned())
        .await
        .unwrap();
    // 获取到文件buf和文件类型
    let (data, file_type): (Vec<u8>, String) =
        icsp::get_file(PEM_STR, OLD_ICSP_CANISTER_TEXT, &key).await;

    // 生成文件名，文件名 例子: source/nCqNxlHdV2Cy1WUWQ6JMe.txt
    let file_name = generate_file_name(&key, file_type_to_extension(&file_type));

    // push要导入的 (/source/nCqNxlHdV2Cy1WUWQ6JMe.pdf, nCqNxlHdV2Cy1WUWQ6JMe, true)
    // put_args.push((file_name.clone(), key.clone(), file_info.is_http_open));

    // 创建文件手柄
    let mut file = std::fs::File::create(&file_name).expect("create failed");

    // 判断 txt形式的，要修改和匹配
    if file_type_to_extension(&file_type) == "txt" {
        let orgin_txt = String::from_utf8_lossy(&data);
        let re1 = Regex::new(OLD_ICSP_CANISTER_TEXT).unwrap();
        let res2 = re1.replace_all(&orgin_txt, NEW_ICSP_CANISTER_TEXT);
        file.write_all(res2.as_bytes()).expect("write failed");
    } else {
        // 如果不是txt
        file.write_all(&data).expect("write failed");
    }
    (file_name, key.to_owned(), file_info.is_http_open)
}

async fn write(arg: (String, String, bool)) {
    println!(
        "complete store file:=================== {:?}",
        // pem_identity_path: &str,
        // file_path_str: &str,
        // icsp_canister_id_text: &str,
        // is_http_open: bool,
        // file_key: String,
        icsp::store_file_by_key(
            PEM_STR,
            &arg.0,
            NEW_ICSP_CANISTER_TEXT,
            arg.2,
            arg.1.clone()
        )
            .await
            .0
    );
}

async fn read_data() -> Vec<Vec<Result<(String, String, bool), JoinError>>> {
    // 所有的file_keys
    let mut file_keys: Vec<String> =
        icsp::get_all_ic_file_key(PEM_STR, OLD_ICSP_CANISTER_TEXT).await;
    // 记录用

    // 有多少个需要转的文件
    // let mut put_args: Vec<(String, String, bool)> = Vec::new();

    // 开始遍历
    let chunk_200: Vec<Vec<String>> = file_keys
        .chunks(20)
        .map(|x: &[String]| x.to_vec())
        .collect();
    let mut all_list = vec![];
    for group200 in chunk_200 {
        let mut handles = vec![];
        for key in group200 {
            let handle = tokio::spawn(read(key));
            handles.push(handle);
        }
        let res = join_all(handles).await;
        all_list.push(res);
    }
    all_list
}

async fn write_data(put_args: Vec<Vec<Result<(String, String, bool), JoinError>>>) {
    for group200 in put_args {
        let mut handles = vec![];
        for arg in group200 {
            if arg.is_ok() {
                let arg = arg.unwrap();
                let handle = tokio::spawn(write(arg));
                handles.push(handle);
            }
        }
        let _ = join_all(handles).await;
    }
}

// async fn check() {
//     let file_keys: Vec<String> = icsp::get_all_ic_file_key(PEM_STR, OLD_ICSP_CANISTER_TEXT).await;

//     let mut index: u64 = 0;
//     for key in &file_keys {
//         let data_1 = icsp::get_file(PEM_STR, OLD_ICSP_CANISTER_TEXT, key.as_str())
//             .await
//             .0;
//         let data_2 = icsp::get_file(PEM_STR, NEW_ICSP_CANISTER_TEXT, key.as_str())
//             .await
//             .0;
//         if data_1 != data_2 {
//             panic!("error! file: {:?}, file_key: {:?}", index, key.clone());
//         };
//         println!("check file: {:?}, file_key: {:?}", index, key.clone());
//         index += 1;
//     }
// }

fn generate_file_name(file_key: &str, file_type: &str) -> String {
    let mut file_name = "source/".to_string();
    file_name.push_str(file_key);
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
        return "apk";
    } else if file_type == "text/xml" {
        return "svg";
    } else if file_type == "video/x-ms-wmv" {
        return "wmv";
    } else {
        return "*";
    };
}