use clap::ArgMatches;
use git2::{self, Repository};
use std::str::FromStr;

use libgitdit::trailer::Trailer;
use libgitdit::{Issue, RepositoryExt};

use system::abort::Abortable;
 
// Opens local git repo
pub fn open_dit_repo() -> Repository {
    // TODO: access the config and maybe return another repo instead
    Repository::open_from_env().unwrap_or_abort()
}

pub fn get_base_url() -> String{
   "http://localhost:3000/".to_string()
}

pub trait RepositoryUtil<'r> {
    //fn remote_priorization(&self) -> RemotePriorization;
    fn prepare_trailers(&self, matches: &ArgMatches) -> Vec<Trailer>;
    fn value_to_issue(&self, value: &str) -> Issue;
}

impl<'r> RepositoryUtil<'r> for Repository {
     fn prepare_trailers(&self, matches: &ArgMatches) -> Vec<Trailer> {
        let mut trailers = Vec::new();

        if matches.is_present("signoff") {
            let sig = self.signature().unwrap_or_abort().to_string();
            trailers.push(Trailer::new("Signed-off-by", sig.as_str()));
        }

        // append misc metadata
        if let Some(metadata) = matches.values_of("metadata") {
            for trailer in metadata.map(Trailer::from_str) {
                trailers.push(trailer.unwrap_or_abort());
            }
        }
        trailers
    }
    fn value_to_issue(&self, value: &str) -> Issue {
        let id = git2::Oid::from_str(value).unwrap_or_abort();
        self.find_issue(id).unwrap_or_abort()
    }
}