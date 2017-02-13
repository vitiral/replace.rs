//! cmd definition and parsing

use std::str;
use std::iter::Iterator;

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


fn parse_groups<'a, I>(raw: I) -> Result<Groups<'a>> 
        where I: Iterator<Item=&'a str> {
    let mut pos = PosGroups::new();
    let mut named = NamedGroups::new();
    for (i, r) in raw.enumerate() {
        let mut split = r.splitn(2, "=");
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

        if let Ok(p) = name.parse::<usize>() {
            pos.insert(p, value.as_bytes());
        } else {
            let g = NamedGroup { id: i, replace: value.as_bytes() };
            named.insert(name, g);
        }
    }

    if !named.is_empty() && !pos.is_empty() {
        let msg = "cannot replace both positional and named groups".to_string();
        Err(ErrorKind::Cmd(msg).into())
    } else if !named.is_empty() {
        panic!("ERROR: named not yet implemented");
        //Groups::Named(named)
    } else if !pos.is_empty() {
        Ok(Groups::Pos(pos))
    } else {
        let msg = "must replace one of positional or named groups".to_string();
        Err(ErrorKind::Cmd(msg).into())
    }
}

fn parse_trail<'a>(trail: Vec<&'a str>) -> Result<(Vec<&'a str>, Groups<'a>)> {
    let mut itrail = trail.iter();
    let paths: Vec<&str> = (&mut itrail).take_while(|s| **s != "--").map(|s| *s).collect();
    let groups = parse_groups(itrail.map(|s| *s))?;
    Ok((paths, groups))
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
    let (paths, groups) = parse_trail(trail)?;
    println!("paths: {:?}\ngroups: {:?}", paths, groups);


    Ok(Cmd {
        regex: regex, 
        paths: paths,
        groups: groups,
    })
}

#[test]
fn test_parse() {
    // test some invalid args
    {
        // mixed args
        //let result = parse_groups(vec!["0=all", "foo=bar"].iter().map(|s| *s));
        //assert!(result.is_err(), "{:?}", result);
        
        // no equal
        let result = parse_groups(vec!["foo", "0=bar"].iter().map(|s| *s));
        assert!(result.is_err(), "{:?}", result);
    }
    
    // positional args
    {
        let args = vec!["0=all", "1=a", "2=b"];
        let result = parse_groups(args.iter().map(|s| *s)).unwrap();
        let expected = Groups::Pos(hashmap!{
            0 => "all".as_bytes(), 
            1 => "a".as_bytes(), 
            2 => "b".as_bytes()
        });
        assert_eq!(result, expected);
    }

    // parse trail
    {
        let trail = vec!["file1", "file2", "--", "1=a"];
        let (files, groups) = parse_trail(trail).unwrap();
        let expected = Groups::Pos(hashmap!{
            1 => "a".as_bytes(), 
        });
        assert_eq!(groups, expected);
        assert_eq!(files, vec!["file1", "file2"]);
    }
}
