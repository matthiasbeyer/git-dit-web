use iron::prelude::*;
use iron::status;
use iron::IronResult;
use iron::Response;
use iron::Request;

use webutil::util::*;
use hbs::Template;


#[derive(Serialize, Debug)]
pub struct Entry {
    pub name : String,
    pub value : String,
}

#[derive(Serialize, Debug)]
pub struct GitEntries {
    pub filename : String,
    pub all_entries: Vec<Entry>, 
}



fn show_git_config() -> GitEntries {
    let repo = open_dit_repo();
    let cfg = repo.config().unwrap();
    let workdir = format!("{:?}", repo.workdir().unwrap()).to_string();
    let mut entries : Vec<Entry> = Vec::new();

    for entry in &cfg.entries(None).unwrap() {
        let entry = entry.unwrap();
        entries.push( Entry{
            name : entry.name().unwrap().to_string(),
            value : entry.value().unwrap().to_string(),
        })
    }
    let git_entries : GitEntries = GitEntries {
        filename : workdir,
        all_entries : entries,
    };
    git_entries
}


pub fn get_info_handler(_: &mut Request) -> IronResult<Response> {
    use std::collections::BTreeMap;
	let mut resp = Response::new();
    let mut bt_data = BTreeMap::new();

    bt_data.insert("git_entries", show_git_config());


    resp.set_mut(Template::with(include_str!("templates/lists/info_list.hbs"), bt_data))
        .set_mut(status::Ok);
    Ok(resp)
}