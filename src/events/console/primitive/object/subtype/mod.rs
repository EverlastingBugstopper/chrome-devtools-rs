mod array;
mod description;

use std::fmt;

use array::ArrayData;
use description::Description;

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "subtype", rename_all = "lowercase")]
pub enum Subtype {
    Array(ArrayData),
    Null,
    RegExp(Description),
    Date(Description),
    Map(Description),     // TODO
    Set(Description),     // TODO
    WeakMap(Description), // TODO
    WeakSet(Description), // TODO
    #[serde(rename = "iterator")]
    JsIterator(Description), // TODO
    Generator(Description), // TODO
    Error(Description),   // TODO
    Proxy(Description),   // TODO
    Promise(Description), // TODO
    TypedArray(Description), // TODO
    ArrayBuffer(Description), // TODO
    DataView(Description), // TODO
}

impl fmt::Display for Subtype {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self {
            Subtype::Array(array) => write!(f, "{}", array),
            Subtype::Null => write!(f, "null"),
            Subtype::RegExp(reg_exp) => write!(f, "{}", reg_exp),
            Subtype::Date(date) => write!(f, "{}", date),
            _ => write!(f, "unhandled type"),
        }
    }
}
