extern crate open;
extern crate search_candidate;

use std::thread;

use Plugin;
use filters::Filter;
use filters::substring_filter::SubstringFilter;
use searchers::Search;
use searchers::files_searcher::FilesSearcher;
use sorters::Sort;
use sorters::files_sorter::FilesSorter;
use self::search_candidate::SearchCandidate;

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

    fn get_search_result(&self, search_term: String) -> Result<Vec<SearchCandidate>, ()> {
        if !Self::can_handle(search_term.clone()) {
            return Err(());
        }
        let search_term = &search_term[SEARCH_PREFIX.chars().count()..];
        let candidates = FilesSearcher::search();
        let filtered_candidates = SubstringFilter::filter(candidates, search_term.to_string());
        let sorted_candidates = FilesSorter::sort(&filtered_candidates);
        Ok(sorted_candidates)
    }

}

#[cfg(test)]
mod tests {

    extern crate dirs;
    extern crate search_candidate;

    use Plugin;
    use files_plugin::FilesPlugin;
    use self::search_candidate::Key;

    #[test]
    fn simple_search() {
        let homedir = match dirs::home_dir() {
            Some(path) => path.to_string_lossy().into_owned(),
            None => String::from("")
        };
        let actual_result = vec![format!("{}/Downloads/evil.pdf", homedir),
                                 format!("{}/OSSetup/EvilEmacs/straight/repos/evil/doc/evil.pdf", homedir)];
        let search_candidates = FilesPlugin.get_search_result(String::from(":files evil.pdf")).unwrap();
        for i in 0..search_candidates.len() {
            assert_eq!(search_candidates[i].get_value(Key::DisplayText), actual_result[i]);
        }
    }

    #[test]
    fn simple_open_file() {
        let homedir = match dirs::home_dir() {
            Some(path) => path.to_string_lossy().into_owned(),
            None => String::from("")
        };
        let search_candidates = FilesPlugin.get_search_result(String::from(":files evil.pdf"));
        assert!(search_candidates.is_ok());
        let unwrapped_search_candidates = search_candidates.unwrap();
        assert_eq!(unwrapped_search_candidates[0].get_value(Key::DisplayText),
                   format!("{}/Downloads/evil.pdf", homedir));
        assert!(FilesPlugin.execute_primary_action(unwrapped_search_candidates[0].get_value(Key::DisplayText)));
    }
        
}
