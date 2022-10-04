use std::{path::Path, env, process};

mod extract;
mod ip_tool;
mod file_tool;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let param = param_analysis();

    let url = format!("https://www.ip2location.com/download/?token={}&file=DB1LITECSV", param.ip2location_token);
    let download_dir = format!("{}/IP2LOCATION-LITE-DB1.CSV.ZIP", param.download_dir);
    let download_dir = Path::new(&download_dir);
    file_tool::download_file(&url, &download_dir).unwrap_or_else(|e| {
        panic!("download file error {}", e)
    });
    println!("download file successed! path:{}", download_dir.display());

    let unzip_dir = format!("{}/IP2LOCATION-LITE-DB1.CSV.ZIP", param.unzip_dir);
    let unzip_dir = Path::new(&unzip_dir);
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

    Ok(())
}

struct Param {
    ip2location_token: String,
    download_dir: String,
    unzip_dir: String,
    output_dir: String,
}

fn param_analysis() -> Param {
    let ip2location_token = env::var_os("IP2LOCATION_TOKEN").unwrap_or_else(|| {
        println!("Missing ENV parameter 'IP2LOCATION_TOKEN'");
        process::exit(1);
    });
    let download_dir = match env::var_os("DOWNLOAD_DIR") {
        None => "/data/ip-extract".to_string(),
        Some(x) => x.to_str().unwrap().to_string(),
    };
    let unzip_dir = match env::var_os("UNZIP_DIR") {
        None => "/data/ip-extract".to_string(),
        Some(x) => x.to_str().unwrap().to_string(),
    };
    let output_dir = env::var_os("OUTPUT_DIR").unwrap_or_else(|| {
        println!("Missing ENV parameter 'OUTPUT_DIR'");
        process::exit(1);
    });
    Param {
        ip2location_token: ip2location_token.to_str().unwrap().to_string(),
        download_dir,
        unzip_dir,
        output_dir: output_dir.to_str().unwrap().to_string(),
    }
}
