use crate::{loader::Resolver, Ctx, Error, Result};
use alloc::collections::BTreeSet;
use alloc::string::{String, ToString};
use unix_path::Path;

/// The builtin module resolver
///
/// This resolver can also be used as the nested backing resolver in user-defined resolvers.
#[derive(Debug, Default)]
pub struct BuiltinResolver {
    modules: BTreeSet<String>,
}

impl BuiltinResolver {
    /// Add builtin module
    pub fn add_module<P: Into<String>>(&mut self, path: P) -> &mut Self {
        self.modules.insert(path.into());
        self
    }

    /// Add builtin module
    #[must_use]
    pub fn with_module<P: Into<String>>(mut self, path: P) -> Self {
        self.add_module(path);
        self
    }
}

impl Resolver for BuiltinResolver {
    fn resolve<'js>(&mut self, _ctx: &Ctx<'js>, base: &str, name: &str) -> Result<String> {
        let full = if !name.starts_with('.') {
            name.to_string()
        } else {
            let base = Path::new(base);
            if let Some(dir) = base.parent() {
                dir.join(name).to_str().unwrap().to_string()
            } else {
                name.to_string()
            }
        };

        if self.modules.contains(&full) {
            Ok(full)
        } else {
            Err(Error::new_resolving(base, name))
        }
    }
}
