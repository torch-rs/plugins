extern crate inflector;
extern crate raise_window;

use self::inflector::Inflector;

use std::path;
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

fn titlecase_filename_filter(candidates: Vec<String>) -> Vec<String> {
    let mut filtered_candidates = Vec::new();
    for candidate in &candidates {
        let p = path::Path::new(candidate);
        if let Some(filename) = p.file_stem() {
            filtered_candidates.push(filename.to_string_lossy().into_owned().to_title_case());
        }
    }
    filtered_candidates
}

impl Plugin for AppPlugin {

    fn can_handle(search_term: String) -> bool {
        search_term.starts_with(SEARCH_PREFIX)
    }

    fn description(&self) -> &'static str {
        DESCRIPTION
    }

    fn execute_primary_action(&self, input: String) -> bool {
        let mut sorter = FrequencySorter::new(String::from("app"));
        sorter.increment_weight(input.clone());
        sorter.save();

        let input = input.to_lowercase().as_str().replace(" ", "-");
        if cfg!(target_os="linux") {
            if let Err(_e) = raise_window::raise_window_by_class(input.to_title_case()) {
                thread::spawn(move || {
                    process::Command::new("gtk-launch")
                        .arg(input)
                        .spawn()
                        .expect("Unable to run app!");
                });
            }
            true
        } else if cfg!(target_os="macos") {
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

    fn execute_secondary_action(&self, input: String) -> bool {
        let mut sorter = FrequencySorter::new(String::from("app"));
        sorter.increment_weight(input.clone());
        sorter.save();

        let input = input.to_lowercase().as_str().replace(" ", "-");
        if cfg!(target_os="linux") {
            thread::spawn(move || {
                process::Command::new("gtk-launch")
                    .arg(input)
                    .spawn()
                    .expect("Unable to run app!");
            });
            true
        } else if cfg!(target_os="macos") {
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

    fn get_search_result(&self, search_term: String) -> Result<Vec<String>, ()> {
        if !Self::can_handle(search_term.clone()) {
            return Err(());
        }
        let search_term = &search_term[SEARCH_PREFIX.chars().count()..];
        let candidates = AppSearcher::search();
        let filename_candidates = titlecase_filename_filter(candidates);
        let filtered_candidates = IgnoreCaseSubstringFilter::filter(filename_candidates, search_term.to_string());

        let sorter = FrequencySorter::new(String::from("app"));
        let sorted_candidates = sorter.sort(&filtered_candidates);
        Ok(sorted_candidates)
    }
    
}

#[cfg(test)]
mod tests {

    use Plugin;
    use app_plugin::AppPlugin;

    #[test]
    fn run_linux_app() {
        let search_result = AppPlugin.get_search_result(String::from(":app fire"));
        assert!(search_result.is_ok());
        let unwrapped_search_result = search_result.unwrap();
        assert_eq!(unwrapped_search_result.len(), 1);
        let candidate = unwrapped_search_result[0].clone();
        assert_eq!(candidate, "Firefox");
        assert!(AppPlugin.execute_primary_action(candidate.to_string()));
    }

}
