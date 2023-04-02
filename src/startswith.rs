pub fn smt_print(item: &String, obj: &str) -> String {
    let mut result = String::new();
    result.push_str(obj);
    result.push_str(&item.clone().as_str().replace("# ", "").replace("##", "").replace("###", "").replace("####", "").as_ref());
    result.push_str(obj);
    result.push('\n');

    result
}