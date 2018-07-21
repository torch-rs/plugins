extern crate raise_window;

use Plugin;
use filters::Filter;
use filters::ignore_case_substring_filter::IgnoreCaseSubstringFilter;
use searchers::Search;
use searchers::windows_searcher::WindowsSearcher;

static SEARCH_PREFIX: &'static str = ":window ";
pub static DESCRIPTION: &'static str = "A switching windows plugin";

pub struct WindowsPlugin;

impl Plugin for WindowsPlugin {
    
    fn can_handle(search_term: String) -> bool {
        search_term.starts_with(SEARCH_PREFIX)
    }

    fn description(&self) -> &'static str {
        DESCRIPTION
    }

    fn execute_primary_action(&self, input: String) -> bool {
        raise_window::raise_window_by_name(input);
        true
    }

    fn execute_secondary_action(&self, input: String) -> bool {
        false
    }
    
    fn get_search_result(&self, search_term: String) -> Result<Vec<String>, ()> {
        if !Self::can_handle(search_term.clone()) {
            return Err(());
        }
        let search_term = &search_term[SEARCH_PREFIX.chars().count()..];
        let candidates = WindowsSearcher::search();
        let filtered_candidates = IgnoreCaseSubstringFilter::filter(candidates, search_term.to_string());
        Ok(filtered_candidates)
    }

}

#[cfg(test)]
mod tests {

    use Plugin;
    use windows_plugin::WindowsPlugin;

    #[test]
    fn run_linux_app() {
        let search_result = WindowsPlugin.get_search_result(String::from(":window term"));
        assert!(search_result.is_ok());
        let unwrapped_search_result = search_result.unwrap();
        assert_eq!(unwrapped_search_result.len(), 1);
        let candidate = unwrapped_search_result[0].clone();
        assert_eq!(candidate, "termite");
        assert!(WindowsPlugin.execute_primary_action(candidate.to_string()));
    }

}
