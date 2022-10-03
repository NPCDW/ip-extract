use std::fs::File;
use std::path::Path;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let url = "https://www.ip2location.com/download/?token=Bpty9cFpYzCxnL5F2Fhkqxu9FVZm0tT1GRN66ycs3w6xJv7qD1Gc2HeCiCMcfFqc&file=DB1LITECSV";
    let path = Path::new("/data/test/test2/IP2LOCATION-LITE-DB1.CSV.ZIP");
    let file: File = ip_extract::download_file(url, path)?;

    ip_extract::unzip(&file, path.parent().unwrap())?;

    let _list = ip_extract::read_csv(&path.parent().unwrap().join(Path::new("IP2LOCATION-LITE-DB1.CSV")))?;

    println!("Hello, world!");
    Ok(())
}
