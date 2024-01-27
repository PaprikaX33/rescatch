pub fn str_expand(code: u16) -> &'static str {
    match code {
        400 => "Bad Request",
        200 => "OK",
        201 => "Created",
        _ => "Unknown Code",
    }
}
