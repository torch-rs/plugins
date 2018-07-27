extern crate search_candidate;

use Plugin;
use filters::Filter;
use filters::substring_filter::SubstringFilter;
use searchers::Search;
use searchers::wordlist_searcher::WordlistSearcher;
use self::search_candidate::SearchCandidate;

pub static DESCRIPTION: &'static str = "A wordlist plugin";

pub struct WordlistPlugin;

impl Plugin for WordlistPlugin {

    fn can_handle(search_term: String) -> bool {
        search_term.starts_with(":wordlist ")
    }

    fn description(&self) -> &'static str {
        DESCRIPTION
    }

    fn execute_primary_action(&self, _input: String) -> bool {
        false
    }

    fn execute_secondary_action(&self, _input: String) -> bool {
        false
    }

    fn get_search_result(&self, search_term: String) -> Result<Vec<SearchCandidate>, ()> {
        if !Self::can_handle(search_term.clone()) {
            return Err(());
        } 
        let search_term = &search_term[":wordlist ".chars().count()..];
        let candidates = WordlistSearcher::search();
        let filtered_candidates = SubstringFilter::filter(candidates, search_term.to_string());
        Ok(filtered_candidates)
    }
    
}

#[cfg(test)]
mod tests {

    extern crate search_candidate;

    use Plugin;
    use wordlist_plugin::WordlistPlugin;
    use self::search_candidate::Key;

    #[test]
    fn simple_search() {
        let actual_results = vec!["asssembler", "bossship", "demigoddessship", "earlesss", "goddessship",
                                  "headmistressship", "passsaging", "patronessship"];
        let search_candidates = WordlistPlugin.get_search_result(String::from(":wordlist sss")).unwrap();
        for i in 0..search_candidates.len() {
            assert_eq!(search_candidates[i].get_value(Key::DisplayText), actual_results[i]);
        }
    }

    #[test]
    fn missing_prefix_search() {
        assert!(WordlistPlugin.get_search_result(String::from("sss")).is_err());
    }

    #[test]
    fn wrong_prefix_search() {
        assert!(WordlistPlugin.get_search_result(String::from(":wrongprefix sss")).is_err());
    }

}
