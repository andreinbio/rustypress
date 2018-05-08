use std::collections::BTreeMap;

pub struct Recognizer<T> {
    pub handler: T,
}