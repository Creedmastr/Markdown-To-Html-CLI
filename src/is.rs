pub fn is_italic(vec: Vec<String>) -> (bool, String) {
    let mut index: u32 = 0;
    for item in vec {
        if item.starts_with("*")
            && item.ends_with("*")
            && !item.ends_with("**")
            && !item.starts_with("**")
        {
            return (true, item);
        }

        index += 1;
    }

    return (false, "None".to_string());
}

pub fn is_underligned(vec: Vec<String>) -> (bool, String) {
    let mut index: u32 = 0;
    for item in vec {
        if item.starts_with("__") && item.ends_with("__") {
            return (true, item);
        }

        index += 1;
    }

    return (false, "None".to_string());
}
