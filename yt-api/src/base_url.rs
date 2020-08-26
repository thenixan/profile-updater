use reqwest::Url;

pub static BASE_URL: &str = "https://www.googleapis.com/youtube/v3/";

pub trait BaseUrl {
    fn url(&self) -> Url;
    fn with_path<'a, T: AsRef<str>>(&self, path: &T) -> Url {
        self.url().join(path.as_ref()).unwrap()
    }
}

impl BaseUrl for &'static str {
    fn url(&self) -> Url {
        Url::parse(self).unwrap()
    }
}