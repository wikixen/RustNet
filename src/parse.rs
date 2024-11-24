fn parse_body(body: String) {
    let mut in_tag = false;
    for char in body.chars() {
        if char == '<' {
            in_tag == true;
        } else if char == '>' {
            in_tag = false;
        } else if !in_tag {
            print!("{}", char);
        }
    }
}