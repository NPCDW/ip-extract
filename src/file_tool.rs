use std::{fs, fs::File};
use std::io::{self, Read, Write, BufReader, BufRead};
use std::path::Path;

#[tokio::main]
pub async fn download_file(url: &str, path: &Path) -> Result<(), Box<dyn std::error::Error>> {
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

    Ok(())
}

pub fn unzip(source: &Path, target: &Path) -> Result<(), Box<dyn std::error::Error>> {
    let zip_file = File::open(source).unwrap_or_else(|e| {
        panic!("open file error {}", e)
    });
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

pub trait CsvTrait {
    fn new(line: String) -> Option<Self> where Self: Sized;
}

pub fn read_csv<T: CsvTrait>(path: &Path) -> Result<Vec<T>, Box<dyn std::error::Error>> {
    let file: File = File::open(path)?;
    let mut list = vec![];
    let lines = BufReader::new(file).lines();
    for line in lines {
        match T::new(line?) {
            None => continue,
            Some(x) => list.push(x),
        }
    }
    Ok(list)
}

pub fn write_file(path: &Path, list: Vec<String>)-> Result<(), Box<dyn std::error::Error>> {
    let mut file: File = File::create(path)?;
    for ele in list {
        file.write(ele.as_bytes())?;
    }
    file.flush()?;
    Ok(())
}

#[cfg(test)]
mod file_util_test {
    use crate::{file_tool::*, extract::IpLocation};

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
        unzip(&path, &target_path).unwrap_or_else(|e| {
            panic!("unzip file error {}", e)
        });

        let file_path = Path::new("/data/test/IP2LOCATION-LITE-DB1.IPV6.CSV");
        let metadata = fs::metadata(file_path).unwrap_or_else(|e| {
            panic!("get file metadata error {}", e)
        });
    
        assert_eq!(true, metadata.len() > 0);
    }
    
    #[test]
    fn read_csv_test() {
        let file_path = Path::new("/data/test/IP2LOCATION-LITE-DB1.IPV6.CSV");
        let list: Vec<IpLocation> = read_csv::<IpLocation>(&file_path).unwrap_or_else(|e| {
            panic!("read csv file error {}", e)
        });
    
        assert_eq!(true, list.len() > 0);
        assert_eq!("281470681743359", list.get(0).unwrap().ip_end);
    }
    
    #[test]
    fn write_file_test() {
        let file_path = Path::new("/data/test/test-write.txt");
        let list = vec!["123".to_string(), "789".to_string(), "456".to_string()];
        write_file(file_path, list).unwrap_or_else(|e| {
            panic!("write file error {}", e)
        });
        let mut result = String::new();
        let mut file = File::open(file_path).unwrap_or_else(|e| {
            panic!("open file error {}", e)
        });
        file.read_to_string(&mut result).unwrap_or_else(|e| {
            panic!("read file error {}", e)
        });
    
        assert_eq!("123789456", result);
    }
}