################################################################################
# re: the regexp replacing tool

The command line tool for finding and replacing in text.

## Purpose
There are many code editors with find and replace functionality, but sometimes
you just want a simple tool that can do it at the command line, irregardless
of whether it's source code or plain text. Enter **replace**.

**replace** uses what you already know, regular expression groups, to do the
actual replacing. The syntax is:

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
