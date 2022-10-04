use std::path::Path;

mod extract;
mod ip_tool;
mod file_tool;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let url = "https://www.ip2location.com/download/?token=Bpty9cFpYzCxnL5F2Fhkqxu9FVZm0tT1GRN66ycs3w6xJv7qD1Gc2HeCiCMcfFqc&file=DB1LITECSV";
    let path = Path::new("/data/ip-extract/IP2LOCATION-LITE-DB1.CSV.ZIP");
    file_tool::download_file(url, path).unwrap_or_else(|e| {
        panic!("download file error {}", e)
    });
    println!("download file successed!");

    file_tool::unzip(&path, path.parent().unwrap()).unwrap_or_else(|e| {
        panic!("unzip file error {}", e)
    });
    println!("unzip file successed!");

    let list: Vec<extract::IpLocation> = file_tool::read_csv::<extract::IpLocation>(&path.parent().unwrap().join(Path::new("IP2LOCATION-LITE-DB1.CSV"))).unwrap_or_else(|e| {
        panic!("read csv file error {}", e)
    });
    println!("read csv file successed!");
    
    let str_list = extract::collect(&list, "CN").unwrap_or_else(|e| {
        panic!("collect ip error {}", e)
    });
    println!("collect ip successed!");
    
    let format_list = extract::format_proxifier(str_list);
    println!("collect ip successed!");

    let path = Path::new("/data/ip-extract/proxifier.txt");
    file_tool::write_file(path, format_list).unwrap_or_else(|e| {
        panic!("write file error {}", e)
    });
    println!("write file successed! path:{}", path.display());

    Ok(())
}
