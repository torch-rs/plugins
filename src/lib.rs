extern crate filters;
extern crate searchers;
extern crate search_candidate;
extern crate sorters;

pub mod app_plugin;
pub mod files_plugin;
pub mod windows_plugin;
pub mod wordlist_plugin;

use self::search_candidate::SearchCandidate;

pub fn get_plugin(search_term: String) -> Option<Box<Plugin + 'static + Send>> {
    if files_plugin::FilesPlugin::can_handle(search_term.clone()) {
        Some(Box::new(files_plugin::FilesPlugin))
    } else if wordlist_plugin::WordlistPlugin::can_handle(search_term.clone()) {
        Some(Box::new(wordlist_plugin::WordlistPlugin))
    } else if app_plugin::AppPlugin::can_handle(search_term.clone()) {
        Some(Box::new(app_plugin::AppPlugin))
    } else if windows_plugin::WindowsPlugin::can_handle(search_term.clone()) {
        Some(Box::new(windows_plugin::WindowsPlugin))
    } else {
        None
    }
}

pub trait Plugin {

    fn can_handle(search_term: String) -> bool where Self: Sized;
    fn description(&self) -> &'static str; 
    fn execute_primary_action(&self, input: String) -> bool;
    fn execute_secondary_action(&self, input: String) -> bool;
    fn get_search_result(&self, search_term: String) -> Result<Vec<SearchCandidate>, ()>;
    
}

#[cfg(test)]
mod tests {

    extern crate search_candidate;

    use files_plugin;
    use get_plugin;
    use self::search_candidate::Key;

    #[test]
    fn get_valid_plugin() {
        let plugin_option = get_plugin(String::from(":files sss"));
        assert!(plugin_option.is_some());
        let boxed_plugin = plugin_option.unwrap();
        assert_eq!(boxed_plugin.description(), files_plugin::DESCRIPTION);
    }

    #[test]
    fn validate_boxed_plugin_methods() {
        let plugin = get_plugin(String::from(":wordlist sss")).unwrap();
        let actual_results = vec!["asssembler", "bossship", "demigoddessship", "earlesss", "goddessship",
                                  "headmistressship", "passsaging", "patronessship"];
        let search_candidates = plugin.get_search_result(String::from(":wordlist sss")).unwrap();
        for i in 0..search_candidates.len() {
            assert_eq!(search_candidates[i].get_value(Key::DisplayText), actual_results[i]);
        }
        assert!(!plugin.execute_primary_action(String::from("bossship")));
    }

}
