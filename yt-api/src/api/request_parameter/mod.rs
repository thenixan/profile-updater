use itertools::Itertools;

pub mod collection;
pub mod parts_holder;
pub mod page;

#[derive(Debug)]
pub struct RequestParameter {
    pub key: String,
    pub value: String,
}

impl RequestParameter {
    pub fn from_plain<T: AsRef<str>, K: AsRef<str>>(key: T, value: K) -> Self {
        RequestParameter {
            key: key.as_ref().to_string(),
            value: value.as_ref().to_string(),
        }
    }

    pub fn from_iterator<T: AsRef<str>, K: IntoIterator<Item=U>, U: AsRef<str>>(key: T, value: K) -> Self {
        let mapper = |s: U| s.as_ref().to_string();
        RequestParameter {
            key: key.as_ref().to_string(),
            value: value.into_iter().map(mapper).join(","),
        }
    }
}
