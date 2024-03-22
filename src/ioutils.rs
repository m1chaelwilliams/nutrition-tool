use std::{io, str::FromStr};

pub fn io_read_parse<T>() -> std::io::Result<T> 
where
    T: FromStr
{
    let mut buf = String::new();

    match io::stdin().read_line(&mut buf) {
        Ok(_) => {
            match buf.trim().parse::<T>() {
                Ok(val) => Ok(val),
                Err(_) => Err(io::Error::new(io::ErrorKind::InvalidInput, "Failed to parse input."))
            }
        },
        Err(e) => Err(io::Error::new(io::ErrorKind::InvalidInput, e))
    }
}

pub fn io_read_strip() -> std::io::Result<String> {
    let mut buf = String::new();

    match io::stdin().read_line(&mut buf) {
        Ok(_) => Ok(buf.trim().to_string()),
        Err(e) => Err(io::Error::new(io::ErrorKind::InvalidInput, e))
    }
}