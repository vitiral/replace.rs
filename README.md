################################################################################
# re: the regexp replacing tool

The command line tool for finding and replacing in text.

## Purpose
There are many code editors with find and replace functionality, but sometimes
you just want a simple tool that can do it at the command line, irregardless
of whether it's source code or plain text. Enter **re**.

**re** uses what you already know, regular expression groups, to do the
actual replacing. The syntax is:

```
replace path1 path2 path3 ... REGEX [FLAGS] --0=dog --1=bridge --friend=Scrappy
```

Let's break this down:
- `path1 path2 path3`: these are the files/directories to search for replacing
- `REGEX`: this is regular expression with groups to replace. For example:
    `the (cat) went over the (hill) to find his friend (?P<friend>Mittens)`
- `[FLAGS]`: there are several flags you can pass:
    - `-i --inplace`: replace the files in-place, do not print to stdout.
    - `-r --recursive`: recurse into directories
- `--0=dog`: the regex group at index 0 should be replaced with `dog`.
- `--1=bridge`: the regex group at index 1 should be replaced with `bridge`.
- `--friend=Scrappy`: the regex group named `friend` should be replaced with
    `Scrappy`

Groups that are not specified will be ignored.
