pub fn print_smt(item: &String, obj: &str) -> String {
    let mut result = String::new();
    result.push_str(obj);

    result.push_str(
        &item
            .clone()
            .as_str()
            .replace("# ", "")
            .replace("##", "")
            .replace("###", "")
            .replace("####", "")
            .replace("*", "")
            .replace("* ", "")
            .replace("+ ", "")
            .replace("- ", "")
            .replace("__", "")
            .as_ref(),
    );

    result.push_str(obj.replace("<", "</").as_str());
    result.push('\n');

    result
}
