use serde::{Serialize, Deserialize, Serializer, Deserializer};

// This part is copied from https://docs.rs/cookie_store/0.20.0/src/cookie_store/cookie_store.rs.html#532

struct CookieStoreVisitor;
impl<'de> serde::de::Visitor<'de> for CookieStoreVisitor {
    type Value = reqwest_cookie_store::CookieStore;

    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "a sequence of cookies")
    }

    fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
    where
        A: serde::de::SeqAccess<'de>,
    {
        reqwest_cookie_store::CookieStore::from_cookies(std::iter::from_fn(|| seq.next_element().transpose()), false)
    }
}

// I changed these parts to make the (de)serialization of SessionInfo struct nicer
pub fn serialize<S>(cookie_store: &std::sync::Arc<reqwest_cookie_store::CookieStoreMutex>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let store = cookie_store.lock().unwrap();
    // I'm also doing "iter_any" instead of "iter_unexpired().filter...."
    serializer.collect_seq(store.iter_any())
}
pub fn deserialize<'de, D>(deserializer: D) -> Result<std::sync::Arc<reqwest_cookie_store::CookieStoreMutex>, D::Error>
where
    D: Deserializer<'de>,
{
    let cookie_store = deserializer.deserialize_seq(CookieStoreVisitor)?;
    let cookie_store = reqwest_cookie_store::CookieStoreMutex::new(cookie_store);
    let cookie_store = std::sync::Arc::new(cookie_store);
    Ok(cookie_store)
}
