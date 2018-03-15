use std::fmt;
use webutil::util::*;


//Experimental not fully tested or implemented

// create a new issue
use std::collections::HashMap;
pub fn new_issue(hashmap: &HashMap<String, Vec<String>>) {
    use libgitdit::message::LineIteratorExt;

    let repo = open_dit_repo();
    let new_message: String = hashmap
        .get("new_message")
        .unwrap()
        .into_iter()
        .collect_string();

    let date: DateTime<Local> = Local::now();
    let time = git2::Time::new(date.timestamp(), date.offset().local_minus_utc() / 60);

    let name: &str = &hashmap.get("name").unwrap()[0];
    let email: &str = &hashmap.get("email").unwrap()[0];

    let author: git2::Signature = git2::Signature::new(name, email, &time).unwrap();


    let committer = author.clone();
    let tree = repo.empty_tree().unwrap();

    let id = repo.create_issue(&author, &committer, new_message.trim(), &tree, Vec::new())
        .unwrap();
}

pub fn new_message(hashmap: &HashMap<String, Vec<String>>) {
    use libgitdit::message::LineIteratorExt;

    let repo = open_dit_repo();
    let new_message: String = hashmap
        .get("new_message")
        .unwrap()
        .into_iter()
        .collect_string();

    let date: DateTime<Local> = Local::now();
    let time = git2::Time::new(date.timestamp(), date.offset().local_minus_utc() / 60);

    let name: &str = &hashmap.get("name").unwrap()[0];
    let email: &str = &hashmap.get("email").unwrap()[0];

    let author: git2::Signature = git2::Signature::new(name, email, &time).unwrap();


    let committer = author.clone();
    let tree = repo.empty_tree().unwrap();

    let id = repo.create_issue(&author, &committer, new_message.trim(), &tree, Vec::new())
        .unwrap();
}

pub fn amend_issue(hashmap: &HashMap<String, Vec<String>>) {
    use libgitdit::message::LineIteratorExt;

    let repo = open_dit_repo();
    let new_message: String = hashmap
        .get("new_message")
        .unwrap()
        .into_iter()
        .collect_string();

    let date: DateTime<Local> = Local::now();
    let time = git2::Time::new(date.timestamp(), date.offset().local_minus_utc() / 60);

    let name: &str = &hashmap.get("name").unwrap()[0];
    let email: &str = &hashmap.get("email").unwrap()[0];

    let author: git2::Signature = git2::Signature::new(name, email, &time).unwrap();


    let committer = author.clone();
    let tree = repo.empty_tree().unwrap();


    let id = repo.create_issue(&author, &committer, new_message.trim(), &tree, Vec::new())
        .unwrap();
}

pub fn amend_message(hashmap: &HashMap<String, Vec<String>>) {
    use libgitdit::message::LineIteratorExt;

    let repo = open_dit_repo();
    let new_message: String = hashmap
        .get("new_message")
        .unwrap()
        .into_iter()
        .collect_string();

    let date: DateTime<Local> = Local::now();
    let time = git2::Time::new(date.timestamp(), date.offset().local_minus_utc() / 60);

    let name: &str = &hashmap.get("name").unwrap()[0];
    let email: &str = &hashmap.get("email").unwrap()[0];

    let author: git2::Signature = git2::Signature::new(name, email, &time).unwrap();


    let committer = author.clone();
    let tree = repo.empty_tree().unwrap();

    let id = repo.create_issue(&author, &committer, new_message.trim(), &tree, Vec::new())
        .unwrap();
}
