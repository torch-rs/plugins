extern crate filters;
extern crate searchers;
extern crate sorters;

pub mod files_plugin;
pub mod wordlist_plugin;

pub fn get_plugin(search_term: String) -> Option<Box<Plugin + 'static + Send>> {
    if files_plugin::FilesPlugin::can_handle(search_term.clone()) {
        Some(Box::new(files_plugin::FilesPlugin))
    } else if wordlist_plugin::WordlistPlugin::can_handle(search_term.clone()) {
        Some(Box::new(wordlist_plugin::WordlistPlugin))
    } else {
        None
    }
}

pub trait Plugin {

    fn can_handle(search_term: String) -> bool where Self: Sized;
    fn description(&self) -> &'static str; 
    fn execute_primary_action(&self, input: String) -> bool;
    fn execute_secondary_action(&self, input: String) -> bool;
    fn get_search_result(&self, search_term: String) -> Result<Vec<String>, ()>;
    
}

#[cfg(test)]
mod tests {

    use files_plugin;
    use get_plugin;

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
        assert_eq!(plugin.get_search_result(String::from(":wordlist sss")), 
                   Ok(vec!["asssembler".to_string(), "bossship".to_string(), "demigoddessship".to_string(),
                           "earlesss".to_string(), "goddessship".to_string(), "headmistressship".to_string(),
                           "passsaging".to_string(), "patronessship".to_string()]));
        assert!(!plugin.execute_primary_action(String::from("bossship")));
    }

}
