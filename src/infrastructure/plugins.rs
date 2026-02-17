use libloading::{Library, Symbol};
use anyhow::Result;

pub trait Plugin {
    fn name(&self) -> &str;
    fn execute(&self, input: &str) -> String;
}

type PluginCreate = unsafe fn() -> Box<dyn Plugin>;

pub fn load_plugins(path: &str) -> Result<Vec<Box<dyn Plugin>>> {
    let mut plugins = Vec::new();

    for entry in std::fs::read_dir(path)? {
        let entry = entry?;
        let lib = unsafe { Library::new(entry.path())? };
        unsafe {
            let constructor: Symbol<PluginCreate> =
                lib.get(b"create_plugin")?;
            plugins.push(constructor());
        }
    }

    Ok(plugins)
}

// #[no_mangle]
// pub unsafe fn create_plugin() -> Box<dyn Plugin> {
//     Box::new(MyPlugin {})
// }
