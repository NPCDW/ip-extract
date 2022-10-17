use std::collections::HashSet;

use crate::{file_tool, ip_tool};

#[allow(dead_code)]
#[derive(Debug)]
pub struct IpLocation {
    pub ip_start: String,
    pub ip_end: String,
    pub country_code: String,
    pub country_name: String,
}

impl file_tool::CsvTrait for IpLocation {
    fn new(line: String) -> Option<Self> {
        let line = line.replace("\"","");
        let split = line.split(",").collect::<Vec<&str>>();
        if split.len() != 4 {
            return None;
        }
        Some(Self {
            ip_start: split.get(0)?.to_string(),
            ip_end: split.get(1)?.to_string(),
            country_code: split.get(2)?.to_string(),
            country_name: split.get(3)?.to_string(),
        })
    }
}

pub fn to_clash(list: &[IpLocation], exclude_country_code: &str) -> HashSet<String> {
    let mut result: HashSet<String> = Default::default();
    let mut index = 0;
    while index < list.len() {
        let item = list.get(index).unwrap();
        if exclude_country_code == item.country_code || "-" == item.country_code {
            index += 1;
            continue;
        }
        result.insert(item.country_code.to_string());
        index += 1;
    }
    result
}

pub fn collect(list: &[IpLocation], exclude_country_code: &str) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let mut result: Vec<String> = Default::default();
    let mut ip_str: String = String::new();

    let mut start_index = 0;
    let mut end_index;
    while start_index < list.len() {
        let start: &IpLocation = list.get(start_index).unwrap();
        if exclude_country_code == start.country_code || "-" == start.country_code {
            start_index += 1;
            continue;
        }
        end_index = start_index + 1;
        while end_index < list.len() {
            let end: &IpLocation = list.get(end_index).unwrap();
            if exclude_country_code == end.country_code || "-" == end.country_code {
                break;
            }
            end_index += 1;
        }
        let from = ip_tool::u32_to_ipv4(start.ip_start.parse::<u32>()?);
        let to = ip_tool::u32_to_ipv4(list.get(end_index - 1).unwrap().ip_end.parse::<u32>()?);
        ip_str.push_str(&format!("{}-{};", &from[..], to)[..]);

        if ip_str.len() > 32000 {
            result.push(ip_str);
            ip_str = String::new();
        }
        start_index = end_index;
    }
    if ip_str.len() > 0 {
        result.push(ip_str);
    }
    Ok(result)
}

pub fn format_proxifier(list: Vec<String>) -> Vec<String> {
    let mut result: Vec<String> = Default::default();
    for (index, value) in list.iter().enumerate() {
        result.push(format!("\t\t<Rule enabled=\"true\">
\t\t\t<Action type=\"Proxy\">100</Action>
\t\t\t<Targets>{}</Targets>
\t\t\t<Name>IP-PROXY-{}</Name>
\t\t</Rule>\n", value, index))
    }
    result
}

pub fn format_clash(list: HashSet<String>) -> Vec<String> {
    let mut result: Vec<String> = Default::default();
    for value in list {
        result.push(format!("GEOIP,{},auto\n", value))
    }
    result
}

#[cfg(test)]
mod extract_test {
    use std::path::Path;

    use crate::{file_tool::*, extract::*};

    #[test]
    fn collect_test() {
        let file_path = Path::new("/data/test/test2/IP2LOCATION-LITE-DB1.CSV");
        let list: Vec<IpLocation> = read_csv::<IpLocation>(&file_path).unwrap_or_else(|e| {
            panic!("read csv file error {}", e)
        });
        let str_list = collect(&list, "CN").unwrap_or_else(|e| {
            panic!("collect ip error {}", e)
        });
        assert_eq!(true, str_list.len() == 7);
    }
    
    #[test]
    fn format_proxifier_test() {
        let str_list = vec!["1.0.0.0-1.0.0.255;1.0.4.0-1.0.7.255;1.0.16.0-1.0.31.255;1.0.64.0-1.0.255.255;1.1.1.0-1.1.1.255;".to_string(),
        "17.81.130.0-17.81.130.255;17.81.132.0-17.81.132.255;17.81.134.0-17.81.134.255;".to_string(),
        "23.129.128.0-23.129.128.255;23.129.144.0-23.129.144.255;23.129.152.0-23.129.152.255;23.129.160.0-23.129.160.255;".to_string()];
        let format_list = format_proxifier(str_list);
        print!("{:#?}", format_list);
        assert_eq!(true, format_list.len() == 3);
    }
    
    #[test]
    fn to_clash_test() {
        let file_path = Path::new("E:/Temp/新建文件夹 (2)/IP2LOCATION-LITE-DB1.CSV/IP2LOCATION-LITE-DB1.CSV");
        let list: Vec<IpLocation> = read_csv::<IpLocation>(&file_path).unwrap_or_else(|e| {
            panic!("read csv file error {}", e)
        });
        let str_list2 = to_clash(&list, "CN");
        println!("to clash successed!");
    
        let format_list2 = format_clash(str_list2);
        println!("format clash successed!");
    
        let output_dir = file_path.parent().unwrap().join("clash.txt");
        write_file(&output_dir, format_list2).unwrap_or_else(|e| {
            panic!("write file error {}", e)
        });
        println!("write file successed! path:{}", output_dir.display());    
    }
    
}
