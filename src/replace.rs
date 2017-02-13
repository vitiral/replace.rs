//! logic for actually replacing text in files

use std::str;

use regex::bytes::{Replacer, Captures};

use types::*;

struct Replace<'a> {
    pub groups: &'a Groups<'a>,
}

/// given groups and captures, return the MatchedGroup objects
fn get_matched_groups<'a>(groups: &PosGroups<'a>, caps: &Captures<'a>) 
        -> Vec<MatchedGroup<'a>> {
    let mut raw_matched: Vec<MatchedGroup> = Vec::new();
    for (i, m) in caps.iter().enumerate() {
        // if the match was good and we have a replacement, store it
        if let Some(m) = m {
            if let Some(g) = groups.get(&i) {
                raw_matched.push(MatchedGroup {
                    mat: m,
                    replace: g,
                    group_id: i,
                })
            }
        }
    }


    raw_matched.sort();
    let mut matched: Vec<MatchedGroup> = Vec::new();
    for m in raw_matched.drain(0..) {
        let mut pop = false;
        let mut push = false;
        if let Some(p) = matched.last() {
            if p.is_superset(&m) {
                // ignore
            } else if m.is_superset(&p) {
                // the new one is a superset, take off the last one
                pop = true;
                push = true;
            } else {
                push = true;
            }
        }  else {
            push = true;
        }
        if pop { matched.pop(); }
        if push { matched.push(m) }
    }

    matched
}

fn replace_append_pos<'a>(groups: &PosGroups<'a>, caps: &Captures, dst: &mut Vec<u8>) {
    // first get our MatchedGroup objects and sort them
    let matched = get_matched_groups(groups, caps);

    // we now have an ordered list list of matched objects with their
    // replacements... start replacing shit!
    let g0 = caps.get(0).unwrap();
    let whole = &caps[0];
    // Notes: loc is relative to `whole`
    // mat.start() is relative to source text
    let mut loc = 0;
    for m in matched {
        if m.mat.start() > g0.start() + loc {
            // write the raw text *between* matches
            let (w, _) = whole.split_at(m.mat.start() - g0.start());
            let (_, w) = w.split_at(loc);
            dst.extend(w);
        }
        dst.extend(m.replace);
        loc = m.mat.end() - g0.start();
    }
    let (_, w) = whole.split_at(loc);
    dst.extend(w);
}

impl<'a> Replacer for Replace<'a> {
    
    fn replace_append(&mut self, caps: &Captures, dst: &mut Vec<u8>) {
        match self.groups {
            &Groups::Pos(ref g) => replace_append_pos(g, caps, dst),
            &Groups::Named(ref g) => panic!(),
        }
    }
}

fn replace<'a>(cmd: &Cmd, data: &[u8]) -> Vec<u8> {
    let r = Replace { groups: &cmd.groups };

    Vec::from(cmd.regex.replace_all(data, r))
}


fn do_replace(c: &Cmd, d: &[u8]) -> String {
    str::from_utf8(&replace(c, d)).unwrap().to_string()
}

#[test]
fn test_replace() {
    let data = b"start. group1 is g1, group2 is g2. end";
    let pat = "group1 is (g1), group2 is (g2)";
    let groups: PosGroups = hashmap!{1 => "r1".as_bytes(), 2 => "r2".as_bytes()};
    // both groups existing
    {
        let cmd = Cmd::simple(pat.clone(), groups.clone());
        // replace one group at a time
        let expected = "start. group1 is r1, group2 is r2. end";
        let result = do_replace(&cmd, data);
        assert_eq!(result, expected);
    }

    // only group 1
    {
        let mut groups = groups.clone();
        groups.remove(&2);
        let cmd = Cmd::simple(pat.clone(), groups);
        let expected = "start. group1 is r1, group2 is g2. end";
        let result = do_replace(&cmd, data);
        assert_eq!(result, expected);
    }

    // only group 2
    {
        let mut groups = groups.clone();
        groups.remove(&1);
        let cmd = Cmd::simple(pat.clone(), groups);
        let expected = "start. group1 is g1, group2 is r2. end";
        let result = do_replace(&cmd, data);
        assert_eq!(result, expected);
    }

    // g0 = ALL
    {
        let mut groups = groups.clone();
        groups.insert(0, "ALL".as_bytes());
        let cmd = Cmd::simple(pat.clone(), groups);
        let expected = "start. ALL. end";
        let result = do_replace(&cmd, data);
        assert_eq!(result, expected);
    }
}
