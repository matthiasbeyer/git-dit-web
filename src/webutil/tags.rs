use std::fmt;
use webutil::util::*;
use git2::*;

#[allow(dead_code)]
#[derive(Debug)]
pub enum IssueCategories {
    New,
    Critical,
    Trivial,
    Subsribed,
    Deleted,
}

impl fmt::Display for IssueCategories {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

// Helper function for list_tagged_issue
// Returns a vector with the issue Oid
fn find_tags(tag_names: Vec<&str>) -> Vec<Oid> {
    println!("find_tags running");

    let repo = open_dit_repo();

    // Oid of tagged commits has to be done this way because of lifetime of repo
    let mut tagged_commits_id: Vec<Oid> = Vec::new();
    for tag_name in tag_names {
        let obj = repo.revparse_single(tag_name).unwrap();
        println!("{}", obj.id());

        if obj.kind() == Some(ObjectType::Commit) {
            tagged_commits_id.push(obj.as_commit().unwrap().id());
        }
    }
    tagged_commits_id
}

/// Get all issues tagged with the given IssueCategory
pub fn list_tagged_issue(is_cat: String) -> Vec<Oid> {
    let repo = open_dit_repo();

    // Construct a fn_pattern to search for issues which are tagged by this category
    let mut fn_pattern: String = String::new();
    fn_pattern.push_str(&is_cat);
    fn_pattern.push_str("*");

    let issue_list = repo.tag_names(Some(&fn_pattern)).unwrap();
    let mut matched_tags: Vec<&str> = Vec::new();

    //list_tagged_issue
    for i in issue_list.iter() {
        matched_tags.push(i.unwrap());
    }
    find_tags(matched_tags)
}

/// Deletes the category of the tag
fn delete_cat_tag(target: &String) {
    let repo = open_dit_repo();

    let remove = move |tag: IssueCategories| {
        let mut tagname: String = String::new();
        tagname.push_str(&tag.to_string());
        tagname.push_str(&format!("{}", target));
        repo.tag_delete(&tagname)
    };

    remove(IssueCategories::New).unwrap();
    remove(IssueCategories::Critical).unwrap();
    remove(IssueCategories::Trivial).unwrap();
    remove(IssueCategories::Subsribed).unwrap();
}

/// Tag the target, with the given Category
pub fn new_cat_tag(tag: String, target: String) {
    println!("new_cat _  tag running");
    let repo = open_dit_repo();
    delete_cat_tag(&target);

    let mut name = tag.to_string();
    let obj = repo.revparse_single(&target).unwrap();
    name.push_str(&target);
    repo.tag_lightweight(&name, &obj, true)
        .expect("Could not create tag");
}
