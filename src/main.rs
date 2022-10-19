use std::{path::Path, env};

mod extract;
mod ip_tool;
mod file_tool;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let param = param_analysis();

    let url;
    if param.ip2location_token == None {
       url = "https://download.ip2location.com/lite/IP2LOCATION-LITE-DB1.CSV.ZIP".to_string(); 
    } else {
        url = format!("https://www.ip2location.com/download/?token={}&file=DB1LITECSV", param.ip2location_token.unwrap());
    }
    let download_dir = format!("{}/IP2LOCATION-LITE-DB1.CSV.ZIP", param.download_dir);
    let download_dir = Path::new(&download_dir);
    file_tool::download_file(&url, &download_dir).unwrap_or_else(|e| {
        panic!("download file error {}", e)
    });
    println!("download file successed! path:{}", download_dir.display());

    let unzip_dir = Path::new(&param.unzip_dir);
    file_tool::unzip(&download_dir, unzip_dir).unwrap_or_else(|e| {
        panic!("unzip file error {}", e)
    });
    println!("unzip file successed! path:{}", unzip_dir.display());

    let list: Vec<extract::IpLocation> = file_tool::read_csv::<extract::IpLocation>(&unzip_dir.join(Path::new("IP2LOCATION-LITE-DB1.CSV"))).unwrap_or_else(|e| {
        panic!("read csv file error {}", e)
    });
    println!("read csv file successed!");
    
    let str_list = extract::collect(&list, "CN").unwrap_or_else(|e| {
        panic!("collect ip error {}", e)
    });
    println!("collect ip successed!");
    
    let format_list = extract::format_proxifier(str_list);
    println!("format ip successed!");

    let output_dir = format!("{}/proxifier.txt", param.output_dir);
    let output_dir = Path::new(&output_dir);
    file_tool::write_file(output_dir, format_list).unwrap_or_else(|e| {
        panic!("write file error {}", e)
    });
    println!("write file successed! path:{}", output_dir.display());

    let str_list2 = extract::to_clash(&list, "CN");
    println!("to clash successed!");

    let format_list2 = extract::format_clash(str_list2);
    println!("format clash successed!");

    let output_dir = format!("{}/clash.txt", param.output_dir);
    let output_dir = Path::new(&output_dir);
    file_tool::write_file(output_dir, format_list2).unwrap_or_else(|e| {
        panic!("write file error {}", e)
    });
    println!("write file successed! path:{}", output_dir.display());

    Ok(())
}

struct Param {
    ip2location_token: Option<String>,
    download_dir: String,
    unzip_dir: String,
    output_dir: String,
}

fn param_analysis() -> Param {
    let ip2location_token = match env::var_os("IP2LOCATION_TOKEN") {
        None => {
            println!("Missing ENV parameter 'IP2LOCATION_TOKEN', Use default download url");
            None
        },
        Some(x) => Some(x.to_str().unwrap().to_string()),
    };
    let download_dir = match env::var_os("DOWNLOAD_DIR") {
        None => "/data/ip-extract".to_string(),
        Some(x) => x.to_str().unwrap().to_string(),
    };
    let unzip_dir = match env::var_os("UNZIP_DIR") {
        None => "/data/ip-extract".to_string(),
        Some(x) => x.to_str().unwrap().to_string(),
    };
    let output_dir = match env::var_os("OUTPUT_DIR"){
        None => "/data/ip-extract".to_string(),
        Some(x) => x.to_str().unwrap().to_string(),
    };
    Param {
        ip2location_token,
        download_dir,
        unzip_dir,
        output_dir,
    }
}
