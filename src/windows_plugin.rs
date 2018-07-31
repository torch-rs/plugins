extern crate raise_window;
extern crate search_candidate;

use filters::Filter;
use filters::ignore_case_substring_filter::IgnoreCaseSubstringFilter;
use Plugin;
use searchers::Search;
use searchers::windows_searcher::WindowsSearcher;
use self::search_candidate::SearchCandidate;

static SEARCH_PREFIX: &'static str = ":window ";
pub static DESCRIPTION: &'static str = "A switching windows plugin";

pub struct WindowsPlugin;

impl Plugin for WindowsPlugin {
    
    fn can_handle(search_term: &str) -> bool {
        search_term.starts_with(SEARCH_PREFIX)
    }

    fn description(&self) -> &'static str {
        DESCRIPTION
    }

    fn execute_primary_action(&self, input: &str) -> bool {
        if let Ok(_) = raise_window::raise_window_by_name(&input) {
            return true;
        }
        false
    }

    fn execute_secondary_action(&self, input: &str) -> bool {
        false
    }
    
    fn get_search_result(&self, search_term: &str) -> Result<Vec<SearchCandidate>, ()> {
        if !Self::can_handle(search_term) {
            return Err(());
        }
        let search_term = &search_term[SEARCH_PREFIX.chars().count()..];
        let candidates = WindowsSearcher::search();
        let filtered_candidates = IgnoreCaseSubstringFilter::filter(candidates, search_term);
        Ok(filtered_candidates)
    }

}

#[cfg(test)]
mod tests {

    extern crate search_candidate;

    use Plugin;
    use self::search_candidate::Key;
    use windows_plugin::WindowsPlugin;
    
    #[test]
    fn run_linux_app() {
        let search_result = WindowsPlugin.get_search_result(":window term");
        assert!(search_result.is_ok());
        let unwrapped_search_result = search_result.unwrap();
        assert_eq!(unwrapped_search_result.len(), 1);
        let candidate = unwrapped_search_result[0].clone();
        assert_eq!(candidate.get_value(Key::DisplayText), "termite");
        assert!(WindowsPlugin.execute_primary_action(&candidate.get_value(Key::DisplayText)));
    }

}
