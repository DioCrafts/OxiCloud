use crate::cache::file_global_gc::FileGlobalGC;

/// Wrapper class that extends the base FileGlobalGC implementation
pub struct LegacyFileGlobalGC {
    inner: FileGlobalGC,
}

impl LegacyFileGlobalGC {
    pub fn new() -> Self {
        Self {
            inner: FileGlobalGC::new(),
        }
    }
}

impl std::ops::Deref for LegacyFileGlobalGC {
    type Target = FileGlobalGC;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl std::ops::DerefMut for LegacyFileGlobalGC {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl From<FileGlobalGC> for LegacyFileGlobalGC {
    fn from(inner: FileGlobalGC) -> Self {
        Self { inner }
    }
}