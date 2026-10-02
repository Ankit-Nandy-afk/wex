Initial Findings-
Wine:
1) Normal close Gives status : 0
2) Missing File Gives status : 1
3) Invalid File(Bad Format) Gives status : 1
4) A child killed by a signal has no normal exit code, which is why Rust's ExitStatus::code() returns an Option

Summary - Different failures  gives same status as 1


PREFIX:

1) Read XDG_DATA_HOME.
2) If its set, nonempty, and an absolute path, use it as the base.
3) Otherwise, use $HOME/.local/share.
4) If HOME is also missing, return an error.
5) Append wex/prefixes/<game-name>, create it, and set WINEPREFIX to it.

