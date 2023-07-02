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

pub fn collect(list: &[IpLocation], exclude_country_code: &str) -> Result<Vec<(String, String)>, Box<dyn std::error::Error>> {
    let mut result = vec![];

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
            if end.ip_end.len() - start.ip_end.len() > 5 {
                break;
            }
            if exclude_country_code == end.country_code || "-" == end.country_code {
                break;
            }
            end_index += 1;
        }
        let from = ip_tool::u32_to_ipv4(start.ip_start.parse::<u32>()?);
        let to = ip_tool::u32_to_ipv4(list.get(end_index - 1).unwrap().ip_end.parse::<u32>()?);
        result.push((from, to));

        start_index = end_index;
    }
    Ok(result)
}

#[allow(dead_code)]
pub fn collect_ipv6(list: &[IpLocation], exclude_country_code: &str) -> Result<Vec<(String, String)>, Box<dyn std::error::Error>> {
    let mut result = vec![];

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
            if end.ip_end.len() - start.ip_end.len() > 5 {
                break;
            }
            if exclude_country_code == end.country_code || "-" == end.country_code {
                break;
            }
            end_index += 1;
        }
        let start_number = start.ip_start.parse::<u128>()?;
        let end_number = list.get(end_index - 1).unwrap().ip_end.parse::<u128>()?;
        let from;
        let to;
        if start_number >= 0xffff_0000_0000 && start_number <= 0xffff_ffff_ffff {
            from = ip_tool::u32_to_ipv4((start_number & 0xffff_ffff).try_into().unwrap());
            to = ip_tool::u32_to_ipv4((end_number & 0xffff_ffff).try_into().unwrap());
        } else {
            from = ip_tool::u128_to_ipv6(start_number);
            to = ip_tool::u128_to_ipv6(end_number);
        }
        result.push((from, to));

        start_index = end_index;
    }
    Ok(result)
}

pub fn format_proxifier(list: &Vec<(String, String)>) -> String {
    let mut result = String::default();
    let mut count = 1;
    let mut str = String::default();
    for (from, to) in list {
        if str.len() > 32000 {
            result.push_str(&format!("\t\t<Rule enabled=\"true\">
\t\t\t<Action type=\"Proxy\">100</Action>
\t\t\t<Targets>{}</Targets>
\t\t\t<Name>IP-PROXY-{}</Name>
\t\t</Rule>\n", str, count));
            str = String::default();
            count += 1;
        }
        str.push_str(&format!("{}-{};", from, to));
    }
    result.push_str(&format!("\t\t<Rule enabled=\"true\">
\t\t\t<Action type=\"Proxy\">100</Action>
\t\t\t<Targets>{}</Targets>
\t\t\t<Name>IP-PROXY-{}</Name>
\t\t</Rule>\n", str, count));
    result
}

pub fn format_clash(list: &Vec<(String, String)>) -> String {
    let mut result = String::default();
    result.push_str("  - GEOIP,LAN,DIRECT\n");
    for (from, to) in list {
        match ip_tool::ipv4_to_cidr(&from, &to) {
            None => {
                continue
            },
            Some(cidrs) => {
                for cidr in cidrs {
                    result.push_str(&format!("  - IP-CIDR,{},auto\n", cidr));
                }
            }
        };
    }
    result.push_str("  - MATCH,DIRECT\n");
    result
}


#[allow(dead_code)]
pub fn format_clash_ipv6(list: &Vec<(String, String)>) -> String {
    let mut result = String::default();
    result.push_str("  - GEOIP,LAN,DIRECT\n");
    for (from, to) in list {
        if to.contains(".") {
            match ip_tool::ipv4_to_cidr(&from, &to) {
                None => {
                    continue
                },
                Some(cidrs) => {
                    for cidr in cidrs {
                        result.push_str(&format!("  - IP-CIDR,{},auto\n", cidr));
                    }
                }
            };
        } else {
            match ip_tool::ipv6_to_cidr(&from, &to) {
                None => {
                    continue
                },
                Some(cidrs) => {
                    for cidr in cidrs {
                        result.push_str(&format!("  - IP-CIDR,{},auto\n", cidr));
                    }
                }
            };
        }
    }
    result.push_str("  - MATCH,DIRECT\n");
    result
}

#[cfg(test)]
mod extract_test {
    use std::path::Path;

    use crate::{file_tool::*, extract::*};

    #[test]
    fn collect_test() {
        let file_path = Path::new(r"C:\data\ip-extract\IP2LOCATION-LITE-DB1.IPV6.CSV");
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
        let file_path = Path::new(r"C:\data\ip-extract\IP2LOCATION-LITE-DB1.IPV6.CSV");
        let list: Vec<IpLocation> = read_csv::<IpLocation>(&file_path).unwrap_or_else(|e| {
            panic!("read csv file error {}", e)
        });
        let str_list = collect(&list, "CN").unwrap_or_else(|e| {
            panic!("collect ip error {}", e)
        });
        let format_list = format_proxifier(&str_list);
        let output_dir = file_path.parent().unwrap().join("proxifier.txt");
        write_file(&output_dir, format_list).unwrap_or_else(|e| {
            panic!("write file error {}", e)
        });
        println!("write file successed! path:{}", output_dir.display());  
    }
    
    #[test]
    fn format_clash_test() {
        let file_path = Path::new("C:/data/ip-extract/IP2LOCATION-LITE-DB1.CSV");
        let list: Vec<IpLocation> = read_csv::<IpLocation>(&file_path).unwrap_or_else(|e| {
            panic!("read csv file error {}", e)
        });
        let str_list = collect(&list, "CN").unwrap_or_else(|e| {
            panic!("collect ip error {}", e)
        });
        let format_list2 = format_clash(&str_list);
        println!("format clash successed!");
    
        let output_dir = file_path.parent().unwrap().join("clash.txt");
        write_file(&output_dir, format_list2).unwrap_or_else(|e| {
            panic!("write file error {}", e)
        });
        println!("write file successed! path:{}", output_dir.display());    
    }
    
}
