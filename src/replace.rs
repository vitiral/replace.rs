//! logic for actually replacing text in files

use std::str;

use regex::bytes::{Replacer, Captures};

use types::*;

struct Replace<'a> {
    pub groups: &'a Groups<'a>,
}

impl<'a> Replace<'a> {
    fn _replace_append_pos(&mut self, groups: &PosGroups<'a>, 
                           caps: &Captures, dst: &mut Vec<u8>) {
        // first get our MatchedGroup objects and sort them
        let mut raw_matched: Vec<MatchedGroup> = Vec::new();
        for (i, m) in caps.iter().enumerate().skip(1) {
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
        let mut iter_matched = raw_matched.iter();
        let mut prev = match iter_matched.next() {
            Some(m) => m,
            None => return,  // no matches we can replace
        };

        let mut matched: Vec<&MatchedGroup> = vec![prev];
        for m in iter_matched {
            if prev.is_superset(m) {
                // ignore
            } else if m.is_superset(&prev) {
                // the new one is a superset, take off the last one
                matched.pop();
                matched.push(m);
            } else {
                matched.push(m)
            }
        }

        // we now have an ordered list list of matched objects with their
        // replacements... start replacing shit!
        let g0 = caps.get(0).unwrap();
        let whole = &caps[0];
        let mut loc = 0;
        for m in matched {
            if m.mat.start() > loc {
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
}

impl<'a> Replacer for Replace<'a> {
    
    fn replace_append(&mut self, caps: &Captures, dst: &mut Vec<u8>) {
        match self.groups {
            &Groups::Pos(ref g) => self._replace_append_pos(g, caps, dst),
            &Groups::Named(ref g) => panic!(),
        }
    }
}

fn replace<'a>(cmd: &Cmd, data: &[u8]) -> Vec<u8> {
    let r = Replace { groups: &cmd.groups };

    Vec::from(cmd.regex.replace_all(data, r))
}

#[test]
fn test_replace() {
    let data = b"start. group1 is g1, group2 is g2. end";
    let pat = "group1 is (g1), group2 is (g2)";
    let groups: PosGroups = hashmap!{1 => "r1".as_bytes(), 2 => "r2".as_bytes()};
    let cmd = Cmd::simple(pat, groups);

    let do_replace = |c, d| {
        str::from_utf8(&replace(c, d)).unwrap().to_string()
    };

    let expected = "start. group1 is r1, group2 is r2. end";
    let result = do_replace(&cmd, data);
    assert_eq!(result, expected);
}
