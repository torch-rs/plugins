extern crate filters;
extern crate searchers;

pub mod wordlist_plugin;

pub trait Plugin {

    fn can_handle(search_term: String) -> bool;
    fn get_search_result(&self, search_term: String) -> Result<Vec<String>, ()>;
    
}
