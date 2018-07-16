use Plugin;
use filters::Filter;
use filters::substring_filter::SubstringFilter;
use searchers::Search;
use searchers::wordlist_searcher::WordlistSearcher;

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

    fn get_search_result(&self, search_term: String) -> Result<Vec<String>, ()> {
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

    use Plugin;
    use wordlist_plugin::WordlistPlugin;

    #[test]
    fn simple_search() {
        assert_eq!(WordlistPlugin.get_search_result(String::from(":wordlist sss")),
                   Ok(vec!["asssembler".to_string(), "bossship".to_string(), "demigoddessship".to_string(),
                           "earlesss".to_string(), "goddessship".to_string(), "headmistressship".to_string(),
                           "passsaging".to_string(), "patronessship".to_string()]));
    }

    #[test]
    fn missing_prefix_search() {
        assert_eq!(WordlistPlugin.get_search_result(String::from("sss")),
                   Err(()));
    }

    #[test]
    fn wrong_prefix_search() {
        assert_eq!(WordlistPlugin.get_search_result(String::from(":wrongprefix sss")),
                   Err(()));
    }

}
