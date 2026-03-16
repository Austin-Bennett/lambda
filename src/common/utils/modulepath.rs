use std::fmt::{Debug, Display, Formatter};
use std::hash::{Hash, Hasher};
use std::path::{Path, PathBuf};

#[derive(Eq, PartialEq, Clone)]
pub struct ModulePath {
    components: Vec<String>
}

impl ModulePath {
    pub fn from_path(path: impl AsRef<Path>) -> Self {
        let path = path.as_ref();
        Self{
            components: path.components().map(|c| c.as_os_str().to_string_lossy().to_string()).collect()
        }
    }

    pub fn from_module_path(path: impl AsRef<str>) -> Self {
        let path = path.as_ref();

        Self{
            components: path.split("::").map(|c| c.to_owned()).collect()
        }
    }

    pub fn to_path(&self) -> PathBuf {
        let mut res = String::new();

        let mut first = true;
        for c in &self.components {
            if !first {
                res.push('/');
            }
            first = false;
            res += c;
        }

        res += ".lm";

        res.into()
    }
}

impl Hash for ModulePath {
    fn hash<H: Hasher>(&self, state: &mut H) {
        for c in &self.components {
            c.hash(state);
        }
    }
}

impl Debug for ModulePath {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut first = true;
        for c in &self.components {
            if !first {
                f.write_str("::")?;
            }
            first = false;
            write!(f, "{}", c)?;
        }

        Ok(())
    }
}
impl Display for ModulePath {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(self, f)
    }
}