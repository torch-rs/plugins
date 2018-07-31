extern crate inflector;
extern crate raise_window;
extern crate search_candidate;

use self::inflector::Inflector;
use self::search_candidate::SearchCandidate;

use std::process;
use std::thread;

use Plugin;
use filters::Filter;
use filters::ignore_case_substring_filter::IgnoreCaseSubstringFilter;
use searchers::Search;
use searchers::app_searcher::AppSearcher;
use sorters::WeightedSort;
use sorters::frequency_sorter::FrequencySorter;

static SEARCH_PREFIX: &'static str = ":app ";
pub static DESCRIPTION: &'static str = "A app plugin";

pub struct AppPlugin;

impl Plugin for AppPlugin {

    fn can_handle(search_term: &str) -> bool {
        search_term.starts_with(SEARCH_PREFIX)
    }

    fn description(&self) -> &'static str {
        DESCRIPTION
    }

    fn execute_primary_action(&self, input: &str) -> bool {
        let mut sorter = FrequencySorter::new(String::from("app"));
        sorter.increment_weight(input.to_string());
        sorter.save();

        if cfg!(target_os="linux") {
            if let Err(_e) = raise_window::raise_window_by_class(&input.to_title_case()) {
                let input = input.to_string();
                thread::spawn(move || {
                    process::Command::new("gtk-launch")
                        .arg(input)
                        .spawn()
                        .expect("Failed to run command");
                });
            }
            true
        } else if cfg!(target_os="macos") {
            let input = input.to_string();
            thread::spawn(move || {
                process::Command::new("open")
                    .arg("-a")
                    .arg(input)
                    .spawn()
                    .expect("Unable to run app!");
            });
            true
        } else if cfg!(target_os="windows") {
            false
        } else {
            false
        }
    }

    fn execute_secondary_action(&self, input: &str) -> bool {
        let mut sorter = FrequencySorter::new(String::from("app"));
        sorter.increment_weight(input.to_string());
        sorter.save();

        let input = input.to_lowercase().as_str().replace(" ", "-");
        if cfg!(target_os="linux") {
            let input = input.to_string();
            thread::spawn(move || {
                process::Command::new("gtk-launch")
                    .arg(input)
                    .spawn()
                    .expect("Unable to run app!");
            });
            true
        } else if cfg!(target_os="macos") {
            let input = input.to_string();
            thread::spawn(move || {
                process::Command::new("open")
                    .arg("-a")
                    .arg("-n")
                    .arg(input)
                    .spawn()
                    .expect("Unable to run app!");
            });
            true
   } else if cfg!(target_os="windows") {
            false
        } else {
            false
        }
    }

    fn get_search_result(&self, search_term: &str) -> Result<Vec<SearchCandidate>, ()> {
        if !Self::can_handle(search_term.clone()) {
            return Err(());
        }
        let search_term = &search_term[SEARCH_PREFIX.chars().count()..];
        let search_candidates = AppSearcher::search();
        let filtered_candidates = IgnoreCaseSubstringFilter::filter(search_candidates, search_term);

        let sorter = FrequencySorter::new(String::from("app"));
        let sorted_candidates = sorter.sort(&filtered_candidates);
        Ok(sorted_candidates)
    }
    
}

#[cfg(test)]
mod tests {

    extern crate search_candidate;

    use Plugin;
    use app_plugin::AppPlugin;
    use self::search_candidate::Key;

    #[test]
    fn run_linux_app() {
        let search_result = AppPlugin.get_search_result(":app fire") ;
        assert!(search_result.is_ok());
        let unwrapped_search_result = search_result.unwrap();
        assert_eq!(unwrapped_search_result.len(), 1);
        let candidate = unwrapped_search_result[0].clone();
        assert_eq!(candidate.get_value(Key::DisplayText), "Firefox");
        assert!(AppPlugin.execute_primary_action(&candidate.get_value(Key::DisplayText)));
    }

    #[test]
    fn test_thunar() {
        let search_result = AppPlugin.get_search_result(":app thun");
        assert!(search_result.is_ok());
        let unwrapped_search_result = search_result.unwrap();
        let candidate = unwrapped_search_result[0].clone();
        assert_eq!(candidate.get_value(Key::DisplayText), "Thunar");
        assert!(AppPlugin.execute_primary_action(&candidate.get_value(Key::DisplayText)));
    }

}
