extern crate iron;
extern crate router;
extern crate git2;

use iron::status;
use iron::IronResult;
use iron::Response;
use iron::Request;
use router::Router;
use iron::Url;
use iron::modifiers::Redirect;
use webutil::util::open_dit_repo;
use webutil::util::get_base_url;
use webutil::cat_files::*;
use libgitdit::RepositoryExt;
use iron::prelude::*;
// use this feature to live reload changes ind the handlebar template files
// useful for development
#[cfg(feature = "watch")]
use hbs::Watchable;
use hbs::Template;
use urlencoded::{UrlEncodedBody};


// TO do pass an argument with req if empty show default if has value show chossen list
pub fn index(_ : &mut Request) -> IronResult<Response> {
	let mut resp = Response::new();

	let data = list_issues();
    
    resp.set_mut(Template::with(include_str!("templates/lists/issue_list.hbs"), data))
        .set_mut(status::Ok);
    Ok(resp)
}

pub fn show_issue(req :&mut Request) -> IronResult<Response> {
    use std::collections::BTreeMap;  
    let mut resp = Response::new();
    let ref message_id = req.extensions.get::<Router>().unwrap().find("message_id").unwrap_or("/");
    let ref order = req.extensions.get::<Router>().unwrap().find("order").unwrap_or("/");
    let issue_id = git2::Oid::from_str(message_id).unwrap();
    let repo = open_dit_repo();
    let issue = repo.find_issue(issue_id).unwrap();
    let mut bt_data = BTreeMap::new();

    bt_data.insert("issue", issue_to_json(issue, order));

    resp.set_mut(Template::with(include_str!("templates/lists/show_issue_tree.hbs"), bt_data))
        .set_mut(status::Ok);
    Ok(resp)
}

/// Returns a list of all the  Issues of this Repository
/// If this Handler ist supplied with the Option tree, the content will be displayed through fancytree.js.
pub fn list_handler(_: &mut Request) -> IronResult<Response> {
	let mut resp = Response::new();
	let data = list_issues();

    resp.set_mut(Template::with(include_str!("templates/lists/issue_list.hbs"), data))
        .set_mut(status::Ok);
    Ok(resp)
}

pub fn get_tagged_issues(req: &mut Request) -> IronResult<Response> {
    let mut resp = Response::new();
	use webutil::tags::*;
	//let resp = Response::new();
	let tag = req.extensions.get::<Router>().unwrap().find("tag").unwrap_or("/");

    // Shows a list with Issues containing the searched for tag
	let data = list_issues_oid(list_tagged_issue(String::from(tag)));

    resp.set_mut(Template::with(include_str!("templates/lists/issue_list.hbs"), data))
        .set_mut(status::Ok);
    Ok(resp)
}

pub fn remote_issues_handler(req: &mut Request) -> IronResult<Response> {
    // Shows a list with Issues containing the searched for tag
    index(req)
}

pub fn local_issues_handler(req: &mut Request) -> IronResult<Response> {
    index(req)
}

/// Tags the transmitted target Issue, with the transmitted tag. 
/// The tag can only be one of the String representation of a state definend in 
/// enum IssueCategories  
pub fn new_cat_tag_handler(req :&mut Request) -> IronResult<Response> {
    use webutil::tags::*;
    let ref tag = req.extensions.get::<Router>().unwrap().find("tag").unwrap_or("/");
    let ref target = req.extensions.get::<Router>().unwrap().find("target").unwrap_or("/");
    new_cat_tag(String::from(*tag),
                String::from(*target));

    let url : iron::Url  = Url::parse(&get_base_url()).unwrap();
    Ok(Response::with((status::Found, Redirect(url.clone()))))
}

/// Creates a new Issue through the Data send in the Form.
pub fn new_issue_handler(req :&mut Request) -> IronResult<Response> {
    // Extract the decoded data as hashmap, using the UrlEncodedQuery plugin.
 
    match req.get_ref::<UrlEncodedBody>() {    
        Ok(ref hashmap) => {
            //Redirect to the Issue after apending the Message. The Problem is that the redirected Message will be displayed,
            // in the context of the last. To circumvent this Redirects will happen to the original page and not to the issue.
                
            new_issue(*hashmap);
        },
        Err(ref e) => {
            info!("Issue could not be created {}", e);
        }
    }; 

    let url : iron::Url  = Url::parse(&get_base_url()).unwrap();
    Ok(Response::with((status::Found, Redirect(url.clone()))))
}

pub fn new_message_handler(req :&mut Request) -> IronResult<Response> {
    //Initialize the Url to redirect the call after being successful
    let mut target_oid = "";
    
    // Extract the decoded data as hashmap, using the UrlEncodedQuery plugin.
    match req.get_ref::<UrlEncodedBody>() {    
        Ok(ref hashmap) => {
            //Redirect to the Issue after apending the Message. The Problem is that the redirected Message will be displayed,
            // in the context of the last. To circumvent this Redirects will happen to the original page and not to the issue.
            target_oid= &hashmap.get("target").unwrap()[0];                
            new_message(*hashmap);
        },
        Err(ref e) => {
            info!("Message could not be created {}", e);
        }
    }; 

    //Construct the Url the URL to be redirected to 
    let url_string = &format!("{}show_issue/{}/newest_first", &get_base_url(), target_oid);
    let url = Url::parse(url_string).unwrap();

    Ok(Response::with((status::Found, Redirect(url.clone()))))    
}

pub fn new_reply_handler(req :&mut Request) -> IronResult<Response> {
    //Initialize the Url to redirect the call after being successful  
    // Extract the decoded data as hashmap, using the UrlEncodedQuery plugin.
    match req.get_ref::<UrlEncodedBody>() {    
        Ok(ref hashmap) => {
            //Redirect to the Issue after apending the Message. The Problem is that the redirected Message will be displayed,
            // in the context of the last. To circumvent this Redirects will happen to the original page and not to the issue.

            new_reply(*hashmap);
        }
        ,
        Err(ref e) => {
            info!("Message could not be created {}", e);
        }
    }; 

    //Construct the Url the URL to be redirected to 
    let url_string = &format!("{}", &get_base_url());
    let url = Url::parse(url_string).unwrap();
    Ok(Response::with((status::Found, Redirect(url.clone()))))    
}

pub fn reply_message_handler(req :&mut Request) -> IronResult<Response> {
    let ref message_id = req.extensions.get::<Router>().unwrap().find("message_id").unwrap_or("/");

	let mut resp = Response::new();
	let data = reply_message(message_id);

    resp.set_mut(Template::with(include_str!("templates/lists/submit_reply.hbs"), data))
        .set_mut(status::Ok);
    Ok(resp)
}