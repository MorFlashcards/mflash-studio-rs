use crate::plugin::MFlashPlugin;

pub fn get_active_plugins(_enabled_list: &[String]) -> Vec<Box<dyn MFlashPlugin>> {
    vec![] // Empty as requested, no timer by default
}
