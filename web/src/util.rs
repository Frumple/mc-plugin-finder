use num_format::{Buffer, Locale};

pub fn format_number(number: &i32) -> String {
    let mut buf = Buffer::default();
    buf.write_formatted(number, &Locale::en);
    buf.as_str().to_string()
}