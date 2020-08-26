#[derive(Eq, PartialEq, Debug, Clone)]
pub struct ChannelId(pub(crate) String);

impl<T> From<T> for ChannelId where T: AsRef<str> {
    fn from(t: T) -> Self {
        ChannelId(t.as_ref().to_string())
    }
}
