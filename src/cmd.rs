//! cmd definition and parsing

use std::str;

use clap as cl;

use types::*;


static HELP: &'static str = r#"replace

USAGE: replace PATTERN [FLAGS] FILES -- 1=g1 2=g2

PATTERN:
    PATTERN is a regular expression pattern as defined by
    https://doc.rust-lang.org/regex/regex/index.html#syntax

FLAGS: 
    -d --diff   only display diff and exit.

FILES:
    These are the files you wish to replace the text in

ADDITIONAL:
    Additional arguments after `--` are used to fill out either the indexed
    or named groups of the form `1=value` for positional groups or
    `name=value` for named groups.

EXAMPLE: 
```
set PATTERN = 'The (cat) went over the (hill) to find his friend (?P<friend>Mittens)`
replace $PATTERN path1 path2 -- dog bridge --friend=Scrappy
```

Breaking this down:
- `set PATTERN = ...`: this is the regular expression pattern we will use
- `path1 path2`: these are the files/directories to search for replacing
- `$PATTERN`: using our pattern in our command
- `[FLAGS]`: there are several flags you can pass:
- `--0=dog`: the regex group at index 0 should be replaced with `dog`.
- `--1=bridge`: the regex group at index 1 should be replaced with `bridge`.
- `--friend=Scrappy`: the regex group named `friend` should be replaced with
    `Scrappy`

Groups that are not specified will be ignored.

The given example would replace:
    The cat went over the hill to find his friend Mittens

With:
    The dog went over the bridge to find his friend Scrappy
"#;


pub fn get_app<'a, 'b>() -> cl::App<'a, 'b> where 'a: 'b {
    cl::App::new("replace")
        .version("0.1.0")
        .author("Garrett Berg <vitiral@gmail.com>")
        .about("find and replace text using regular expressions. https://github.com/vitiral/replace.rs")
        .help(HELP)
        .setting(cl::AppSettings::TrailingVarArg)
        .arg(cl::Arg::with_name("pattern")
             .value_name("PATTERN")
             .help("regular expression pattern")
            .required(true))
        .arg(cl::Arg::with_name("other")
             .multiple(true)
             .required(true))
}


fn parse_named_groups<'a>(raw: Vec<&'a str>) -> Result<NamedGroups<'a>> {
    let mut groups  = NamedGroups::new();
    for r in raw {
        let (_, s) = r.split_at(2);
        let mut split = s.splitn(2, "=");
        let name = match (&mut split).next() {
            Some(n) => n,
            None => {
                let msg = format!("Named group must be of form \"name=value\": {:?}", r);
                return Err(ErrorKind::Cmd(msg).into());
            }
        };
        let value = match (&mut split).next() {
            Some(n) => n,
            None => {
                let msg = format!("Named group must be of form \"name=value\": {:?}", r);
                return Err(ErrorKind::Cmd(msg).into());
            }
        };
        groups.insert(name, value.as_bytes());
    }
    Ok(groups)
}

pub fn get_cmd<'a>(matches: &'a cl::ArgMatches) -> Result<Cmd<'a>> {
    let regex = match Regex::new(matches.value_of("pattern").unwrap()) {
        Ok(r) => r,
        Err(e) => {
            let msg = format!("Invalid regex pattern: {}", e);
            return Err(ErrorKind::Cmd(msg).into());
        }
    };

    let trail: Vec<&str> = matches.values_of("other").unwrap().collect();
    let mut itrail = trail.iter();
    let paths: Vec<&str> = (&mut itrail).take_while(|s| **s != "--").map(|s| *s).collect();

    let mut named: Vec<&str> = Vec::new();
    let mut positional: Vec<&[u8]> = Vec::new();

    for g in itrail {
        if g.starts_with("--") {
            named.push(g);
        } else {
            positional.push(g.as_bytes());
        }
    }
    let named = parse_named_groups(named)?;

    println!("paths: {:?}\nnamed: {:?}\npos:  {:?}", paths, named, positional);

    let groups = if !named.is_empty() && !positional.is_empty() {
        let msg = "cannot replace both positional and named groups".to_string();
        return Err(ErrorKind::Cmd(msg).into());
    } else if !named.is_empty() {
        Groups::Named(named)
    //} else if !positional.is_empty() {
    //    Groups::Pos(positional)
    } else {
        let msg = "must replace one of positional or named groups".to_string();
        return Err(ErrorKind::Cmd(msg).into());
    };

    Ok(Cmd {
        regex: regex, 
        paths: paths,
        groups: groups,
    })
}
