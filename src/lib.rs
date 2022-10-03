use std::{fs, fs::File};
use std::io::{self, Read, Write, BufReader, BufRead};
use std::path::Path;

#[tokio::main]
pub async fn download_file(url: &str, path: &Path) -> Result<File, Box<dyn std::error::Error>> {
    let body = reqwest::get(url).await?
        .bytes().await?;
    let dir = path.parent();
    if None != dir {
        let dir = dir.unwrap();
        fs::create_dir_all(dir).unwrap_or_else(|e| {
            panic!("Could not create file directory: {}, {:?}", dir.display(), e)
        });
    }
    let mut file = File::create(path).unwrap_or_else(|e| {
        panic!("Could not create file: {:?}", e);
    });
    let content = body.bytes();
    let data: Result<Vec<_>, _> = content.collect();
    file.write_all(&data.unwrap())?;

    Ok(file)
}

pub fn unzip(zip_file: &File, target: &Path) -> Result<(), Box<dyn std::error::Error>> {
    if !target.exists() {
        fs::create_dir_all(target).unwrap_or_else(|e| {
            panic!("Could not create target directory: {}, {:?}", target.display(), e)
        });
    }
    let mut zip = zip::ZipArchive::new(zip_file)?;
    for i in 0..zip.len() {
        let mut file = zip.by_index(i).unwrap();
        if file.is_dir() {
            println!("file utf8 path {:?}", file.name_raw());
            let target_dir = target.join(Path::new(&file.name()));
            fs::create_dir_all(&target_dir).unwrap_or_else(|e| {
                panic!("Could not create target directory: {}, {:?}", &target_dir.display(), e)
            });
        } else {
            let file_path = target.join(Path::new(file.name()));
            let mut target_file = File::create(&file_path).unwrap_or_else(|e| {
                panic!("Could not create file: {}, {:?}", &file_path.display(), e);
            });
            io::copy(&mut file, &mut target_file).unwrap_or_else(|e| {
                panic!("Could not copy file: {}, {:?}", &file_path.display(), e);
            });
        }
    }
    Ok(())
}

#[derive(Debug)]
pub struct IpLocation {
    _ip_start: String,
    _ip_end: String,
    _country_code: String,
    _country_name: String,
}

impl IpLocation {
    fn new(split: Vec<&str>) -> Self {
        Self {
            _ip_start: split.get(0).unwrap().to_string(),
            _ip_end: split.get(1).unwrap().to_string(),
            _country_code: split.get(2).unwrap().to_string(),
            _country_name: split.get(3).unwrap().to_string(),
        }
    }
}

pub fn read_csv(path: &Path) -> Result<Vec<IpLocation>, Box<dyn std::error::Error>> {
    let file: File = File::open(path)?;
    let mut list = vec![];
    let lines = BufReader::new(file).lines();
    for line in lines {
        let line = line?.replace("\"","");
        let split = line.split(",").collect::<Vec<&str>>();
        if split.len() != 4 {
            continue;
        }
        let ip_location = IpLocation::new(split);
        list.push(ip_location);
    }
    Ok(list)
}

pub fn ipv4_split_to_u32(split: Vec<&str>) -> Option<u32> {
    if split.len() != 4 {
        return None;
    }
    let mut number: u32 = match split.get(0)?.parse::<u32>() {
        Err(_) => None,
        Ok(x) => Some(x),
    }?;
    for i in 1..4 {
        number = number << 8;
        number = number | match split.get(i)?.parse::<u32>() {
            Err(_) => None,
            Ok(x) => Some(x),
        }?;
    }
    Some(number)
}

pub fn ipv4_to_u32(ipv4: &str) -> Option<u32> {
    let split = ipv4.split(".").collect::<Vec<&str>>();
    ipv4_split_to_u32(split)
}

pub fn ipv6_to_u128(ipv6: &str) -> Option<u128> {
    let split = ipv6.split([':', '.'].as_ref()).collect::<Vec<&str>>();
    if split.len() == 4 {
        return Some((ipv4_split_to_u32(split)? as u128) | 0xffff00000000);
    }
    if split.len() != 8 {
        return None;
    }
    let mut number: u128 = match u128::from_str_radix(split.get(0)?, 16) {
        Err(_) => None,
        Ok(x) => Some(x),
    }?;
    for i in 1..8 {
        number = number << 16;
        number = number | match u128::from_str_radix(split.get(i)?, 16) {
            Err(_) => None,
            Ok(x) => Some(x),
        }?;
    }
    Some(number)
}

pub fn u32_to_ipv4(mut number: u32) -> String {
    let mut arr: [String; 4] = Default::default();
    for i in (0..4).rev() {
        arr[i] = (number & 0xff).to_string();
        number = number >> 8;
    }
    arr.join(".")
}

pub fn u128_to_ipv6(mut number: u128) -> String {
    if number < 281474976710656 {
        return u32_to_ipv4((number & 0xffffffff).try_into().unwrap());
    }
    let mut arr: [String; 8] = Default::default();
    for i in (0..8).rev() {
        arr[i] = format!("{:x}", (number & 0xffff));
        number = number >> 16;
    }
    arr.join(":")
}

#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    fn download_file_test() {
        let url = "https://www.ip2location.com/download/?token=Bpty9cFpYzCxnL5F2Fhkqxu9FVZm0tT1GRN66ycs3w6xJv7qD1Gc2HeCiCMcfFqc&file=DB1LITECSVIPV6";
        let path = Path::new("/data/test/test2/IP2LOCATION-LITE-DB1.IPV6.CSV.ZIP");
        download_file(url, path).unwrap_or_else(|e| {
            panic!("download file error {}", e)
        });
        assert_eq!(true, path.exists());
    }
    
    #[test]
    fn unzip_test() {
        let path = Path::new("/data/test/test2/IP2LOCATION-LITE-DB1.IPV6.CSV.ZIP");
        let target_path = Path::new("/data/test/");
        let file = File::open(path).unwrap_or_else(|e| {
            panic!("open file error {}", e)
        });
        unzip(&file, &target_path).unwrap_or_else(|e| {
            panic!("unzip file error {}", e)
        });

        let file_path = Path::new("/data/test/IP2LOCATION-LITE-DB1.IPV6.CSV");
        let metadata = fs::metadata(file_path).unwrap_or_else(|e| {
            panic!("get file metadata error {}", e)
        });
    
        assert_eq!(true, metadata.len() > 0);
    }
    
    #[test]
    fn ipv4_to_u32_test() {
        let x = ipv4_to_u32("223.255.255.255");
        if x == None {
            panic!("convert ipv4 to u32 fail")
        }
        assert_eq!(3758096383, x.unwrap());
        let x = ipv4_to_u32("1.0.0.0");
        if x == None {
            panic!("convert ipv4 to u32 fail")
        }
        assert_eq!(16777216, x.unwrap());
        let x = ipv4_to_u32("0.0.0.0");
        if x == None {
            panic!("convert ipv4 to u32 fail")
        }
        assert_eq!(0, x.unwrap());
        let x = ipv4_to_u32("255.255.255.255");
        if x == None {
            panic!("convert ipv4 to u32 fail")
        }
        assert_eq!(4294967295, x.unwrap());
    }
    
    #[test]
    fn ipv6_to_u128_test() {
        let x = ipv6_to_u128("ffff:ffff:ffff:ffff:ffff:ffff:ffff:ffff");
        if x == None {
            panic!("convert ffff:ffff:ffff:ffff:ffff:ffff:ffff:ffff to u128 fail")
        }
        assert_eq!(340282366920938463463374607431768211455, x.unwrap());
        let x = ipv6_to_u128("2001:200:0:0:0:0:0:0");
        if x == None {
            panic!("convert 2001:200:0:0:0:0:0:0 to u128 fail")
        }
        assert_eq!(42540528726795050063891204319802818560, x.unwrap());
        let x = ipv6_to_u128("255.255.255.255");
        if x == None {
            panic!("convert 255.255.255.255 to u128 fail")
        }
        assert_eq!(281474976710655, x.unwrap());
        let x = ipv6_to_u128("0.0.0.0");
        if x == None {
            panic!("convert 0.0.0.0 to u128 fail")
        }
        assert_eq!(281470681743360, x.unwrap());
        let x = ipv6_to_u128("223.255.255.255");
        if x == None {
            panic!("convert 223.255.255.255 to u128 fail")
        }
        assert_eq!(281474439839743, x.unwrap());
    }
    
    #[test]
    fn u32_to_ipv4_test() {
        assert_eq!("255.255.255.255", u32_to_ipv4(4294967295));
        assert_eq!("223.255.255.255", u32_to_ipv4(3758096383));
        assert_eq!("1.0.0.0", u32_to_ipv4(16777216));
        assert_eq!("0.0.0.0", u32_to_ipv4(0));
    }
    
    #[test]
    fn u128_to_ipv6_test() {
        assert_eq!("ffff:ffff:ffff:ffff:ffff:ffff:ffff:ffff", u128_to_ipv6(340282366920938463463374607431768211455));
        assert_eq!("2001:200:0:0:0:0:0:0", u128_to_ipv6(42540528726795050063891204319802818560));
        assert_eq!("0.0.0.0", u128_to_ipv6(281470681743360));
        assert_eq!("223.255.255.255", u128_to_ipv6(281474439839743));
        assert_eq!("255.255.255.255", u128_to_ipv6(281474976710655));
    }
}