#[macro_use]
extern crate log;
#[macro_use]
extern crate serde_derive;

extern crate chrono;
extern crate clap;
extern crate env_logger;
extern crate git2;
extern crate handlebars_iron as hbs;
extern crate iron;
extern crate libgitdit;
extern crate mount;
extern crate regex;
extern crate router;
extern crate serde;
extern crate serde_json;
extern crate staticfile;
extern crate urlencoded;

use hbs::{DirectorySource, HandlebarsEngine};
use hbs::handlebars::{Handlebars, Helper, RenderContext, RenderError};
use iron::Iron;
use iron::middleware::Chain;
use router::Router;
use mount::Mount;
mod error;
use staticfile::Static;
mod system;
//use gitext::RemotePriorization;
//mod filters;

use std::io;
use std::io::Write;
use log::LogLevel;
use std::path::Path;

#[macro_use]
mod webutil;

use webutil::iron_handlers::*;
use webutil::git_info::*;

fn main() {
    if let Err(err) = system::Logger::init(LogLevel::Warn) {
        writeln!(io::stderr(), "Could not initialize logger: {}", err).ok();
    }

    let mut hbse = HandlebarsEngine::new();
    let source = Box::new(DirectorySource::new("./webutil/templates/", ".hbs"));
    hbse.add(source);

    if let Err(r) = hbse.reload() {
        panic!("{}", r);
    }

    hbse.handlebars_mut().register_helper(
        "some_helper",
        Box::new(
            |_: &Helper, _: &Handlebars, _: &mut RenderContext| -> Result<(), RenderError> {
                Ok(())
            },
        ),
    );

    let mut router = Router::new();


    hbse.handlebars_mut()
        .register_template_file(
            "header",
            &Path::new("./src/webutil/templates/base_templates/header.hbs"),
        )
        .ok()
        .unwrap();
    hbse.handlebars_mut()
        .register_template_file("index", "./src/webutil/templates/base_templates/index.hbs")
        .ok()
        .unwrap();
    hbse.handlebars_mut()
        .register_template_file(
            "sidenav",
            "./src/webutil/templates/base_templates/sidenav.hbs",
        )
        .ok()
        .unwrap();
    hbse.handlebars_mut()
        .register_template_file(
            "submitIssue",
            "./src/webutil/templates/forms/submitIssue.hbs",
        )
        .ok()
        .unwrap();
    hbse.handlebars_mut()
        .register_template_file(
            "submitMessage",
            "./src/webutil/templates/forms/submitMessage.hbs",
        )
        .ok()
        .unwrap();
    hbse.handlebars_mut()
        .register_template_file(
            "navbar",
            "./src/webutil/templates/base_templates/navbar.hbs",
        )
        .ok()
        .unwrap();
    hbse.handlebars_mut()
        .register_template_file("assign", "./src/webutil/templates/util/assign_tag.hbs")
        .ok()
        .unwrap();

    // Routes to the Handlers through the dynamic URL given as first parameter.
    // For a description of what each route does, look at
    router.get("/", index, "index");
    router.get("/show_issue/:message_id/:order", show_issue, "show_issue");
    router.post(
        "/new_cat_tag_handler/:tag/:target",
        new_cat_tag_handler,
        "new_cat_tag_handler",
    );
    router.get("/list_handler", list_handler, "list_handler");
    router.get("/get_info", get_info_handler, "get_info_handler");
    router.get(
        "/get_tagged_issues/:tag",
        get_tagged_issues,
        "get_tagged_issues",
    );
    router.get("/local_issues", local_issues_handler, "local_issues");
    router.get("/remote_issues", remote_issues_handler, "remote_issues");

    router.get(
        "/reply_message/:message_id",
        reply_message_handler,
        "reply_message_handler",
    );

    // Every function with a Messagebody is trasnmitted
    router.post("new_reply_handler", new_reply_handler, "new_reply_handler");
    router.post(
        "new_message_handler",
        new_message_handler,
        "new_message_handler",
    );
    router.post("new_issue_handler", new_issue_handler, "new_issue_handler");

    //just for fun to expirement with stuff
    let mut mount = Mount::new();
    mount.mount("/", router);

    //Is there to load css js html doesen't quiet work yet but a fix is only of "cosmetical" priority for now
    mount.mount("/assets/", Static::new(Path::new("src/webutil/templates/")));

    let mut chain = Chain::new(mount);
    chain.link_after(hbse);
    Iron::new(chain).http("localhost:3000").unwrap();
}
