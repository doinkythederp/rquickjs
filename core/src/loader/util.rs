use alloc::string::{String, ToString};
use unix_path::Path;

pub fn resolve_simple(base: &str, name: &str) -> String {
    if name.starts_with('.') {
        let path = Path::new(base);
        if let Some(dir) = path.parent() {
            return dir.join(name).to_str().unwrap().to_string();
        }
    }
    name.into()
}

pub fn check_extensions(name: &str, extensions: &[String]) -> bool {
    let path = Path::new(name);
    path.extension()
        .map(|extension| {
            extensions
                .iter()
                .any(|known_extension| known_extension == extension.to_str().unwrap())
        })
        .unwrap_or(false)
}
