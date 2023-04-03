pub fn to_valid_html(previous_result: String) -> String {
    let mut result: String = String::from("
        <!DOCTYPE html>\n
        <html>\n
        <head>\n
            <meta charset='utf-8'>\n
            <meta http-equiv='X-UA-Compatible' content='IE=edge'>\n
            <title>Page Title</title>\n
            <meta name='viewport' content='width=device-width, initial-scale=1'>\n
            <link rel='stylesheet' type='text/css' media='screen' href='main.css'>\n
            <script src='main.js'></script>\n
        </head>\n
        <body>\n
    ");
    
    result.push_str(&previous_result);
    result.push('\n');

    result.push_str("
        </body>\n
        </html>\n
    ");

    result
}