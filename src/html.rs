pub fn to_valid_html(previous_result: String) -> String {
    let mut result: String = String::from("
        <!DOCTYPE html>
        <html>
        <head>
            <meta charset='utf-8'>
            <meta http-equiv='X-UA-Compatible' content='IE=edge'>
            <title>Page Title</title>
            <meta name='viewport' content='width=device-width, initial-scale=1'>
        </head>
        <body> \n
    ");
    
    result.push_str(&previous_result);

    result.push_str("
        </body>
        </html>
    ");

    result
}