use axum::extract::FromRef;
use axum_extra::extract::cookie::Key;

#[derive(Debug, Clone)]
pub struct KeksiState {
    key: Key,
}

impl KeksiState {
    pub fn new(key: String) -> Self {
        Self {
            key: Key::from(key.as_bytes()),
        }
    }
}

impl FromRef<KeksiState> for Key {
    fn from_ref(input: &KeksiState) -> Self {
        input.key.clone()
    }
}
