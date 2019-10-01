use serde::{Deserialize, Serialize};
use std::fmt::{self, Display, Formatter};

cfg_if::cfg_if! {
    if #[cfg(any(
        target_pointer_width = "64",
        target_pointer_width = "128",
        feature = "big-numbers"))
    ] {
        pub type Inner = i64;
        pub type Unsigned = u64;
        pub type Innerx2 = i128;
    } else {
        pub type Inner = i32;
        pub type Unsigned = u32;
        pub type Innerx2 = i64;
    }
}

#[derive(Serialize, Deserialize, Debug, Copy, Clone, PartialEq)]
pub struct Id(pub(crate) Inner);

impl From<Unsigned> for Id {
    fn from(v: Unsigned) -> Self {
        let value = if v > Inner::max_value() as Unsigned {
            // If v is above the max value, we subtract and cast the result.
            (v - (Inner::max_value() as Unsigned) - 1) as Inner
        } else {
            // Otherwise, since v is a valid IdInner, cast it and subtract from that.
            v as Inner - Inner::max_value() - 1
        };
        Self(value)
    }
}

impl Display for Id {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let output = self.0 as Innerx2 + Inner::max_value() as Innerx2 + 1;
        write!(f, "{}", output)
    }
}

#[cfg(test)]
mod tests {
    use super::{Id, Inner, Unsigned};
    #[test]
    fn correct_display() {
        assert_eq!(format!("{}", Id(Inner::min_value())), "0".to_string());
        assert_eq!(
            format!("{}", Id(0)),
            format!("{}", Inner::max_value() as Unsigned + 1)
        );
        assert_eq!(
            format!("{}", Id(Inner::max_value())),
            format!("{}", Unsigned::max_value())
        );
    }
    #[test]
    fn correct_from() {
        assert_eq!(Id::from(0 as Unsigned).0, Inner::min_value());
        assert_eq!(Id::from(Unsigned::max_value()).0, Inner::max_value());
        assert_eq!(Id::from(Inner::max_value() as Unsigned).0, -1);
        assert_eq!(Id::from(Inner::max_value() as Unsigned + 1).0, 0);
        assert_eq!(Id::from(Inner::max_value() as Unsigned + 2).0, 1);
    }
}
