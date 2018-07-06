#![allow(trivial_casts)]

use toml;
use std::{io, fmt, time};

use project;
// use project::error::ProjectError;
use storage::error::StorageError;

#[allow(missing_doc)]
error_chain!{
    types {
        Error, ErrorKind, ResultExt, Result;
    }

    links { }

    foreign_links {
        Io(io::Error);
        Fmt(fmt::Error);
        Time(time::SystemTimeError);
        Toml(toml::de::Error);
        Project(project::error::Error);
        Storage(StorageError);
    }

    errors {
        ActionError{
            description("unexpected response from service")
        }
        AddingFailed{
            description("Adding Failed")
        }
    }
}

#[allow(missing_doc)]
pub mod failure {
    use toml;
    use std::{io, fmt, time};

    use project;
    // use project::error::ProjectError;
    use storage::error::StorageError;

    #[derive(Debug, Fail)]
    pub enum Error {

        #[fail(display = "unexpected response from service")]
        ActionError,

        #[fail(display = "Adding Failed")]
        AddingFailed,

        #[fail(display = "{}", _0)]
        Io(io::Error),

        #[fail(display = "{}", _0)]
        Fmt(fmt::Error),

        #[fail(display = "{}", _0)]
        Time(time::SystemTimeError),

        #[fail(display = "{}", _0)]
        Toml(toml::de::Error),

        #[fail(display = "{}", _0)]
        Project(project::error::Error),

        // #[fail(display = "{}", _0)]
        // Storage(StorageError),
    }
}