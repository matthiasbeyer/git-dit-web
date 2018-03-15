extern crate git2;

use git2::Commit;
use git2::*;
use system::Abortable;
use libgitdit::issue::*;
use webutil::util::*;
use libgitdit::RepositoryExt;

#[derive(Serialize, Debug)]
pub struct MessageBody {
    pub message : String,
}

#[derive(Serialize, Debug)]
pub struct MessageToJson {
    pub head: String, // All in one Information for the beginning of the Message
    pub hash_id:  String,/*git2::Oid*/
    pub author: SignatureToJson,
    //Date: chrono date Datatype or String
    pub tags : String,
    //Either 
    pub subject : String,
    pub message_body: Vec<MessageBody>, 
}

#[derive(Serialize, Debug)]
pub struct IssueToJson {
    pub id: String,
    pub head: String, // All in one Header Information of the initial issue message  
    pub messages: Vec<MessageToJson>,
}

#[derive(Serialize, Debug)]
pub struct SignatureToJson {
    pub name : String,
    pub email : String,
    pub when : String,
}

#[derive(Serialize, Debug)]
pub struct IssueList {
    pub issues: Vec<IssueToJson>,
}

pub fn list_issues() -> IssueList{
    let repo = open_dit_repo();
    let mut json_issues : Vec<IssueToJson> = Vec::new();

    let mut issues : Vec<Issue> = repo
        .issues()
        .unwrap_or_abort()
        .into_iter()
        .collect();

    // descending order
    let mut sort_key : Box<FnMut(&Issue) -> git2::Time> = Box::new(|ref issue| issue
        .initial_message()
        .unwrap_or_abort()
        .time());
        issues.sort_by(|a, b| sort_key(b).cmp(&sort_key(a)));

    for issue in issues {
        let init_commit = issue.initial_message().unwrap();
        let issue_id = format!("{}", issue.id());
        //let head = get_message_header(init_commit); 

        let head  = message_to_json(init_commit);

        let messages_lol : Vec<MessageToJson> = vec![head];
        let json_issue = IssueToJson {
            id : issue_id,
            head : String::new(),
            messages : messages_lol,
        };
        json_issues.push(json_issue);
    }

    let json_list : IssueList = IssueList {
        issues : json_issues,
    };
    json_list
}

//use webutil::tags::*;

//list only tag specific issues 
pub fn list_issues_oid(issue_oid : Vec<Oid>) -> IssueList{
    let repo = open_dit_repo();
    let mut json_issues : Vec<IssueToJson> = Vec::new();

    let mut issues : Vec<Issue>  = Vec::new();

    for id in issue_oid {
        issues.push(repo.find_issue(id).unwrap_or_abort());
    }

    for issue in issues {
            let issue_id = format!("{}", issue.id());
            let init_commit  = issue.initial_message().unwrap();
            let head  = message_to_json(init_commit);
            let messages_lol : Vec<MessageToJson> = vec![head];

            let json_issue = IssueToJson {
                id : issue_id,
                head : String::new(),
                messages : messages_lol,
            };
            json_issues.push(json_issue);
    }

    let json_list : IssueList = IssueList {
        issues : json_issues,
    };
    json_list
}

pub fn message_to_json(message: Commit) -> MessageToJson{
	use chrono::{FixedOffset, TimeZone};
    let time = FixedOffset::east(message.author().when().offset_minutes()*60)
    	.timestamp(message.author().when().seconds(), 0)
        .to_string();
    let author = SignatureToJson{
        name : message.author().name().unwrap().to_string(),
        email : message.author().email().unwrap().to_string(),
        when : time
    };
    let hash_id = format!("{}", message.id());
    let mut message_body : Vec<MessageBody> = Vec::new();

    let mut get_subject : bool = true; 
    let mut subject : String = String::new();
    for lines in message.message(){
        for line in lines.lines() {
            // Get the First line of a Message as Subject 
            if get_subject == true {
                subject = line.to_string();
                get_subject = false;
            } else {
            // Push every line has its own struct, so that handlebars can access each line seperately 
                message_body.push(
                    MessageBody{
                        message : line.to_string()
                    }
                );
            }
        }
    }

    let json_message = MessageToJson {
        hash_id : hash_id,
        author : author,
        tags : String::new(),
        subject : subject,
        message_body : message_body,
        head :  get_message_header(message),
    };

    json_message
}

///Transforms an libgit Issue into a iron_handlebar serialized json struct.
pub fn issue_to_json(issue: Issue, order: &str) ->  IssueToJson {
    let mut messages_lol : Vec<MessageToJson> = Vec::new();  

    let mut messages : Vec<git2::Commit> = issue
        .messages()
        .unwrap()
        .map(|s| s.unwrap())
        .collect();
    //Sort messages from oldest message to newest
    messages.sort_by_key(|a| a.time().seconds());

    //reverse the order so the newest is shown first
    if order.to_string() == "newest_first".to_string() {
        messages.reverse();
    }

    for commit in messages {
        let message = message_to_json(commit);
        messages_lol.push(message);
    }
    
    let init_commit = issue.initial_message().unwrap();
    let head = get_message_header(init_commit); 

    let result = IssueToJson {
        id : format!("{}", issue.id()),
        head : head,
        messages : messages_lol
    };
    result
}

// small helper function to display list header 
fn get_message_header(message: Commit) -> String { 
    let mut head = String::with_capacity(1000);
    //Neccessary to split into two statements because of unwrap() consuming lifetime
    head.push_str(message.author().name().unwrap());

    //Same as above
    head.push_str(message.author().email().unwrap());

	use chrono::{FixedOffset, TimeZone};
	    
    let form_time = FixedOffset::east(message.author().when().offset_minutes()*60)
    	.timestamp(message.author().when().seconds(), 0)
        .to_string();
    head.push_str(&form_time);
    head
}

// create a new issue
use std::collections::HashMap;
pub fn new_issue(hashmap : &HashMap<String, Vec<String>>) {
 	use libgitdit::message::LineIteratorExt;
 
    let repo = open_dit_repo();
	let mut  new_message : String = String::new();
    
    new_message.push_str(&hashmap.get("subject")
                        .unwrap() 
                        .clone()
						.into_iter()
						.collect_string());

    new_message.push_str(&hashmap.get("message")
                        .unwrap() 
                        .clone()
						.into_iter()
						.collect_string());

    let author = repo.signature().unwrap_or_abort();
	let committer = author.clone();
    let tree = repo.empty_tree().unwrap();
	
    repo
        .create_issue(&author, &committer, new_message.trim(), &tree, Vec::new())
        .unwrap();
	}

pub fn new_message(hashmap : &HashMap<String, Vec<String>>) -> Oid {
    let repo = open_dit_repo();
    let committer = repo.signature().unwrap_or_abort();
    let author = repo.signature().unwrap_or_abort();
    let issue_id = hashmap.get("target").unwrap().clone();
    let issue_oid : Oid = git2::Oid::from_str(&issue_id[0]).unwrap();
    let issue = repo.find_issue(issue_oid).unwrap_or_abort();
    let parent = repo.find_commit(issue_oid).unwrap_or_abort();

    // extract the subject and tree from the parent
    let tree = parent.tree().unwrap_or_abort();

    //let parent_refs  = parent.parents();
    let mut message : String = String::new();
    let subject : &str = &hashmap.get("subject").unwrap()[0];

    message.push_str(subject);
    message.push_str("\n");

	let message_body = hashmap.get("message").unwrap().clone();

    let temp = split_line(message_body);
    message.push_str(&temp);
    
    let parent_refs = Some(&parent).into_iter(); /*chain(Some(issue.messages().unwrap())) */ 

    issue.add_message(&author, &committer, message.trim(), &tree, parent_refs)
        .unwrap_or_abort(); 
         
    issue_oid         
}

#[derive(Serialize, Debug)]
pub struct ReplyJson {
    pub message_id: String, 
}

pub fn reply_message(message_id : &str) -> ReplyJson{
    let result = ReplyJson {
        message_id : message_id.to_string(),    
    };
    result
}

fn split_line(message_body : Vec<String>) -> String {
    let mut message : String = String::new();

    for line in message_body {
        if line.len() < 80 {
            message.push_str(&line);
        } else {
            let words = line.split_whitespace();
            for word in words {
                let mut linecount = 0;
                while  linecount+word.len() < 80 {
                    linecount += word.len();
                    message.push_str(word);
                    message.push_str(" ");
                }
                message.push_str("\n");
            }
        }
    }
    message
}

//Stub has to be implemented
pub fn new_reply( hashmap : &HashMap<String, Vec<String>>) -> Oid {
    let message_oid : Oid = git2::Oid::from_str(&hashmap.get("message_id").unwrap()[0]).unwrap();
         
    message_oid         
}