use serde::{Deserialize, Serialize};
use std::fmt::{self, Display, Formatter};

cfg_if::cfg_if! {
    if #[cfg(any(
        target_pointer_width = "64",
        target_pointer_width = "128",
        feature = "big-numbers"
    ))] {
        pub type Inner = u64;
        #[cfg(feature = "postgres")]
        pub(crate) type Signed = i64;
    } else {
        pub type Inner = u32;
        #[cfg(feature = "postgres")]
        pub(crate) type Signed = i32;
    }
}

#[derive(Serialize, Deserialize, Debug, Copy, Clone, PartialEq)]
pub struct Id(pub(crate) Inner);

impl From<Inner> for Id {
    fn from(v: Inner) -> Id {
        Self(v)
    }
}
impl From<Id> for Inner {
    fn from(id: Id) -> Inner {
        id.0
    }
}

impl Display for Id {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}", Inner::from(*self))
    }
}
