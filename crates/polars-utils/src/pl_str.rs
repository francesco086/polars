use std::sync::Arc;

use once_cell::sync::Lazy;

#[macro_export]
macro_rules! format_pl_smallstr {
    ($($arg:tt)*) => {{
        use std::fmt::Write;

        let mut string = String::new();
        write!(string, $($arg)*).unwrap();
        PlSmallStr::from_string(string)
    }}
}

/// String type that interns small strings and has O(1) clone.
#[derive(Clone, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct PlSmallStr(Arc<str>);

impl PlSmallStr {
    /// Initialize an empty string ""
    /// TODO: make this a `const fn`
    #[inline(always)]
    pub fn const_default() -> Self {
        Self::empty_static().clone()
    }

    /// This is a workaround until `const_default` becomes a const fn
    #[inline(always)]
    pub fn empty_static() -> &'static Self {
        static THIS: Lazy<PlSmallStr> = Lazy::new(|| PlSmallStr::from_static(""));
        &THIS
    }

    /// TODO: make this a `const fn`
    #[inline(always)]
    pub fn from_static(s: &'static str) -> Self {
        Self(Arc::from(s))
    }

    #[inline(always)]
    #[allow(clippy::should_implement_trait)]
    pub fn from_str(s: &str) -> Self {
        Self(Arc::from(s))
    }

    #[inline(always)]
    pub fn from_string(s: String) -> Self {
        Self(Arc::from(s))
    }

    #[inline(always)]
    pub fn as_str(&self) -> &str {
        self.0.as_ref()
    }

    #[inline(always)]
    pub fn into_string(self) -> String {
        self.0.to_string()
    }
}

impl Default for PlSmallStr {
    #[inline(always)]
    fn default() -> Self {
        Self::const_default()
    }
}

impl AsRef<str> for PlSmallStr {
    #[inline(always)]
    fn as_ref(&self) -> &str {
        self.0.as_ref()
    }
}

impl<T> PartialEq<T> for PlSmallStr
where
    T: AsRef<str> + ?Sized,
{
    fn eq(&self, other: &T) -> bool {
        self.as_str() == other.as_ref()
    }
}

impl PartialEq<PlSmallStr> for &str {
    fn eq(&self, other: &PlSmallStr) -> bool {
        *self == other.as_str()
    }
}

impl PartialEq<PlSmallStr> for &&str {
    fn eq(&self, other: &PlSmallStr) -> bool {
        **self == other.as_str()
    }
}

impl PartialEq<PlSmallStr> for String {
    fn eq(&self, other: &PlSmallStr) -> bool {
        self.as_str() == other.as_str()
    }
}

impl PartialEq<PlSmallStr> for &String {
    fn eq(&self, other: &PlSmallStr) -> bool {
        self.as_str() == other.as_str()
    }
}

impl core::fmt::Debug for PlSmallStr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl core::ops::Deref for PlSmallStr {
    type Target = str;

    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        self.0.as_ref()
    }
}

impl core::borrow::Borrow<str> for PlSmallStr {
    #[inline(always)]
    fn borrow(&self) -> &str {
        self.0.as_ref()
    }
}

/// We implement `From<&PlSmallStr>` to support `&[S] where S: Into<PlSmallStr>`,
/// prefer calling `.clone()` for existing `PlSmallStr`s where possible.
impl From<&PlSmallStr> for PlSmallStr {
    #[inline(always)]
    fn from(value: &PlSmallStr) -> Self {
        value.clone()
    }
}

impl From<&&PlSmallStr> for PlSmallStr {
    #[inline(always)]
    fn from(value: &&PlSmallStr) -> Self {
        (*value).clone()
    }
}

impl From<&str> for PlSmallStr {
    #[inline(always)]
    fn from(value: &str) -> Self {
        Self::from_str(value)
    }
}

impl From<&&str> for PlSmallStr {
    #[inline(always)]
    fn from(value: &&str) -> Self {
        Self::from_str(value)
    }
}

impl From<String> for PlSmallStr {
    #[inline(always)]
    fn from(value: String) -> Self {
        Self::from_string(value)
    }
}

impl From<&String> for PlSmallStr {
    #[inline(always)]
    fn from(value: &String) -> Self {
        Self::from_str(value.as_str())
    }
}

impl core::fmt::Display for PlSmallStr {
    #[inline(always)]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}
