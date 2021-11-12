macro_rules! endpoint {
    ($php:expr) => {
        concat!("http://gdomega.7m.pl/gdpsdatabase/", $php, ".php")
    };
}

macro_rules! check_resp {
    ($data:expr) => {{
        if $data == "-1" {
            return Err(ApiError::NoData)
        }
    }};
}
