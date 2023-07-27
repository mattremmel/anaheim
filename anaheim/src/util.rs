use std::sync::OnceLock;

static HOSTNAME: OnceLock<String> = OnceLock::new();

pub fn hostname() -> &'static str {
    HOSTNAME
        .get_or_init(|| {
            ::hostname::get()
                .map(|h| format!("{:?}", h))
                .unwrap_or(String::from("<UNKNOWN_HOST>"))
        })
        .as_str()
}
