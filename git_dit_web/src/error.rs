// git-dit - the distributed issue tracker for git
// Copyright (C) 2017 Matthias Beyer <mail@beyermatthias.de>
// Copyright (C) 2017 Julian Ganz <neither@nut.email>
//
// This program is free software; you can redistribute it and/or modify
// it under the terms of the GNU General Public License version 2 as
// published by the Free Software Foundation.
//

use std::error::Error as EError;


/// Convenience trait for logging error types
///
/// Logs all layers of an error using the `error!` macro.
///
pub trait LoggableError {
    fn log(&self);
}

impl<E> LoggableError for E
    where E: EError
{
    fn log(&self) {
        let mut current = Some(self as &EError);
        while let Some(err) = current {
            error!("{}", err);
            current = err.cause();
        }
    }
}

