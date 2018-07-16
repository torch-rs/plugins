extern crate open;

use std::thread;

use Plugin;
use filters::Filter;
use filters::substring_filter::SubstringFilter;
use searchers::Search;
use searchers::files_searcher::FilesSearcher;

static SEARCH_PREFIX: &'static str = ":files ";
pub static DESCRIPTION: &'static str = "A files plugin";

pub struct FilesPlugin;

impl Plugin for FilesPlugin {

    fn can_handle(search_term: String) -> bool {
        search_term.starts_with(SEARCH_PREFIX)
    }
    
    fn description(&self) -> &'static str {
        DESCRIPTION
    }

    fn execute_primary_action(&self, input: String) -> bool {
        thread::spawn(move || {
            if open::that(input.clone()).is_err() {
                println!("{}", format!("Failed to open {}!", input));
            }
        });
        true
    }

    fn execute_secondary_action(&self, _input: String) -> bool {
        false
    }

    fn get_search_result(&self, search_term: String) -> Result<Vec<String>, ()> {
        if !Self::can_handle(search_term.clone()) {
            return Err(());
        }
        let search_term = &search_term[SEARCH_PREFIX.chars().count()..];
        let candidates = FilesSearcher::search();
        let filtered_candidates = SubstringFilter::filter(candidates, search_term.to_string());
        Ok(filtered_candidates)
    }

}

#[cfg(test)]
mod tests {

    extern crate dirs;

    use Plugin;
    use files_plugin::FilesPlugin;

    #[test]
    fn simple_search() {
        let homedir = match dirs::home_dir() {
            Some(path) => path.to_string_lossy().into_owned(),
            None => String::from("")
        };
        assert_eq!(FilesPlugin.get_search_result(String::from(":files evil.pdf")),
                   Ok(vec![format!("{}/Downloads/evil.pdf", homedir),
                           format!("{}/OSSetup/EvilEmacs/straight/repos/evil/doc/evil.pdf", homedir)]));
    }
        
}
