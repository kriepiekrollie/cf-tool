use reqwest_cookie_store::{CookieStore, CookieStoreMutex};
use serde::{Serializer, Deserializer};
use std::sync::Arc;

// This is all mostly is copied from 
// docs.rs/cookie_store/0.20.0/src/cookie_store/cookie_store.rs.html#532

struct CookieStoreVisitor;
impl<'de> serde::de::Visitor<'de> for CookieStoreVisitor {

    type Value = CookieStore;

    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "a sequence of cookies")
    }

    fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
    where
        A: serde::de::SeqAccess<'de>,
    {
        reqwest_cookie_store::CookieStore::from_cookies(
            std::iter::from_fn(|| seq.next_element().transpose()), false
        )
    }
}

// I changed these parts to make the (de)serialization of the 
// SessionInfo struct nicer
pub fn serialize<S>(cookies: &Arc<CookieStoreMutex>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let store = cookies.lock().unwrap();
    // I'm doing "iter_any" instead of "iter_unexpired().filter...."
    serializer.collect_seq(store.iter_any())
}

pub fn deserialize<'de, D>(deserializer: D) -> Result<Arc<CookieStoreMutex>, D::Error>
where
    D: Deserializer<'de>,
{
    let cookies = deserializer.deserialize_seq(CookieStoreVisitor)?;
    let cookies = CookieStoreMutex::new(cookies);
    let cookies = Arc::new(cookies);
    Ok(cookies)
}
