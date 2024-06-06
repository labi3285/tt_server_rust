
pub fn format_ids(ids: &Option<Vec<u64>>) -> Option<String> {
    if let Some(ids) = ids {
        let codes = ids.iter().map(|e| e.to_string()).collect::<Vec<String>>();
        format_codes(&Some(codes))
    } else {
        None
    }
}
pub fn parse_ids(ids_formatted: &Option<String>) -> Option<Vec<u64>> {
    if let Some(ids_formatted) = ids_formatted {
        let codes = ids_formatted.split("/").filter(|s| !s.is_empty()).map(|e| e.parse::<u64>().unwrap()).collect::<Vec<u64>>();
        if codes.len() > 0 {
            Some(codes)
        } else {
            None
        }
    } else {
        None
    }
}
pub fn formatted_ids_by_add_id(ids_formatted: &Option<String>, id: u64) -> Option<String> {
    if let Some(arr) = parse_ids(ids_formatted) {
        let mut arr1 = Vec::<u64>::new();
        let mut find = false;
        for a in arr {
            if a == id {
                find = true;
            } else {
                arr1.push(a);
            }
        }
        if !find {
            arr1.push(id.clone());
        }
        format_ids(&Some(arr1))
    } else {
        format_ids(&Some(vec![id]))
    }
}
pub fn check_or_remove_id(ids_formatted: &Option<String>, id: u64) -> Option<Vec<u64>> {
    if let Some(arr) = parse_ids(ids_formatted) {
        let mut arr1 = Vec::<u64>::new();
        for a in arr {
            if a != id {
                arr1.push(a);
            }
        }
        Some(arr1)
    } else {
        None
    }
}


pub fn format_codes(codes: &Option<Vec<String>>) -> Option<String> {
    if let Some(codes) = codes {
        let mut fmt = codes.join("/").to_string();
        if fmt.len() > 0 {
            fmt = format!("/{}/", fmt);
            Some(fmt)
        } else {
            None
        }
    } else {
        None
    }
}
pub fn parse_codes(codes_formatted: &Option<String>) -> Option<Vec<String>> {
    if let Some(codes_formatted) = codes_formatted {
        let codes = codes_formatted.split("/").filter(|s| !s.is_empty()).map(|e| e.to_string()).collect::<Vec<String>>();
        if codes.len() > 0 {
            Some(codes)
        } else {
            None
        }
    } else {
        None
    }
}
pub fn check_or_add_code(codes_formatted: &Option<String>, code: String) -> Option<String> {
    if let Some(arr) = parse_codes(codes_formatted) {
        let mut arr1 = Vec::<String>::new();
        let mut find = false;
        for a in arr {
            if a == *code {
                find = true;
            } else {
                arr1.push(a);
            }
        }
        if !find {
            arr1.push(code.clone());
        }
        format_codes(&Some(arr1))
    } else {
        format_codes(&Some(vec![code]))
    }
}
pub fn check_or_remove_code(codes_formatted: &Option<String>, code: String) -> Option<Vec<String>> {
    if let Some(arr) = parse_codes(codes_formatted) {
        let mut arr1 = Vec::<String>::new();
        for a in arr {
            if a != *code {
                arr1.push(a);
            }
        }
        Some(arr1)
    } else {
        None
    }
}
