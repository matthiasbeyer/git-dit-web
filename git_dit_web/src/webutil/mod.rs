pub mod iron_handlers;
pub mod cat_files;
pub mod util;
pub mod tags;
pub mod git_info;

pub use self::iron_handlers::*;
pub use self::cat_files::*;
pub use self::util::open_dit_repo;
pub use self::tags::*;
pub use self::git_info::*;

// Experimental beyond this line 
/*
pub mod cat_files;
pub mod format;
pub mod util;

pub use self::cat_files::show_commit;
pub use self::format::*;
pub use self::util::*;
*/
