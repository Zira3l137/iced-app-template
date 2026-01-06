pub mod io {
    use std::{borrow::Cow, path::Path};

    use anyhow::{Context, Result};

    pub fn read_fonts<P: AsRef<Path>>(path: P) -> Result<Vec<Cow<'static, [u8]>>> {
        let path = path.as_ref();
        let loaded_fonts = path
            .read_dir()
            .context("Failed to read directory")?
            .filter_map(|entry| {
                let path = entry.ok()?.path();
                let bytes = std::fs::read(path).ok()?;
                Some(Cow::Owned(bytes))
            })
            .collect();
        Ok(loaded_fonts)
    }
}
