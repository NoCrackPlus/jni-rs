use crate::sys::jobjectRefType;

/// The type of a JVM reference.
#[allow(missing_docs)]
#[derive(Eq, PartialEq, Debug, Clone, Copy)]
pub enum RefType {
    InvalidRefType = 0,
    LocalRefType = 1,
    GlobalRefType = 2,
    WeakGlobalRefType = 3,
}

impl RefType {
    /// Get RefType from raw.
    pub fn from_raw(reftype: jobjectRefType) -> Self {
        return match reftype{
            jobjectRefType::JNIInvalidRefType => RefType::InvalidRefType,
            jobjectRefType::JNILocalRefType => RefType::LocalRefType,
            jobjectRefType::JNIGlobalRefType => RefType::GlobalRefType,
            jobjectRefType::JNIWeakGlobalRefType => RefType::WeakGlobalRefType,
        }
    }
}