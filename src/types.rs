pub use std::io::{Read, Write};
pub use std::rc::Rc;

pub use std::path::{Path, PathBuf};
pub use std::collections::HashMap;

pub use regex::bytes::{Regex, Match};

// private imports
use std::cmp::Ordering;

/// Library Error and Result types
error_chain! {
    types {
        Error, ErrorKind, ResultExt, Result;
    }

    // Automatic conversions between this error chain and others
    links {
    }

    // Automatic conversions between this error chain and other
    // error types not defined by the `error_chain!`.
    foreign_links {
        Fmt(::std::fmt::Error);
        Io(::std::io::Error);
    }

    // Define additional `ErrorKind` variants. The syntax here is
    // the same as `quick_error!`, but the `from()` and `cause()`
    // syntax is not supported.
    errors {
        Cmd(t: String) {
            description("error calling cmd")
            display("ERROR: {}", t)
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct NamedGroup<'a> {
    pub id: usize,
    pub replace: &'a [u8],
}

pub type NamedGroups<'a> = HashMap<&'a str, NamedGroup<'a>>;
pub type PosGroups<'a> = HashMap<usize, &'a [u8]>;

#[derive(Debug, PartialEq)]
pub enum Groups<'a> {
    Named(NamedGroups<'a>),
    Pos(PosGroups<'a>),
}

// Cmd types

#[derive(Debug)]
pub struct Cmd<'a> {
    pub regex: Regex,
    pub paths: Vec<&'a Path>,
    pub groups: Groups<'a>,
}

#[cfg(test)]
impl<'a> Cmd<'a> {
    pub fn simple(regex: &str, pos: PosGroups<'a>) -> Cmd<'a> {
        Cmd {
            regex: Regex::new(regex).unwrap(),
            paths: Vec::new(),
            groups: Groups::Pos(pos),
        }
    }
}

// File system

/// representation of loaded file
pub struct File {
    pub path: Rc<PathBuf>,
    pub data: Vec<u8>,
}

// Replacing helper types

#[derive(Debug)]
pub struct MatchedGroup<'a> {
    pub mat: Match<'a>,
    pub replace: &'a [u8],
    pub group_id: usize,
}

impl<'a> MatchedGroup<'a> {
    /// return whether this group encompases another
    pub fn is_superset(&self, other: &MatchedGroup) -> bool {
        if self.mat.start() <= other.mat.start() &&
                self.mat.end() >= other.mat.end() &&
                self.group_id < other.group_id {
            true
        } else {
            false
        }
    }  
}

impl<'a> PartialEq for MatchedGroup<'a> {
    fn eq(&self, other: &MatchedGroup<'a>) -> bool {
        self.mat.start() == other.mat.start()
    }
}

impl<'a> Eq for MatchedGroup<'a> {}

impl<'a> Ord for MatchedGroup<'a> {
    fn cmp(&self, other: &MatchedGroup) -> Ordering {
        self.mat.start().cmp(&other.mat.start())
    }
}

impl<'a> PartialOrd for MatchedGroup<'a> {
    fn partial_cmp(&self, other: &MatchedGroup<'a>) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
