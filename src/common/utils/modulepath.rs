use std::fmt::{Debug, Display, Formatter};
use std::hash::{Hash, Hasher};
use std::path::{Path, PathBuf};
use std::str::Split;

#[derive(Eq, PartialEq, Clone)]
pub struct ModulePath {
    pub path: String,
}

impl ModulePath {

    pub fn add_namespace(&mut self, namespace: &ModulePath) {
        self.path = namespace.path.clone() + "::" + &self.path;
    }

    pub fn components(&'_ self) -> Split<'_, &str> {
        self.path.split("::")
    }

    pub fn from_path(path: impl AsRef<Path>) -> Self {
        let path = path.as_ref();


        Self{
            path: path.components()
                .map(|c| c.as_os_str().to_string_lossy().to_string() + "::")
                .collect::<String>()
                .strip_suffix(".lm::")
                .unwrap().to_string()
        }
    }

    pub fn from_module_path(path: impl AsRef<str>) -> Self {
        let path = path.as_ref().to_string();


        Self{
            path
        }
    }

    pub fn to_path(&self) -> PathBuf {
        let mut res = String::new();

        let mut first = true;
        for c in self.path.split("::") {
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
        self.path.hash(state)
    }
}

impl Debug for ModulePath {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.path)
    }
}
impl Display for ModulePath {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(self, f)
    }
}