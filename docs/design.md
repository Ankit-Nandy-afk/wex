Initial Findings-
Wine:
1) Normal close Gives status : 0
2) Missing File Gives status : 1
3) Invalid File(Bad Format) Gives status : 1
4) A child killed by a signal has no normal exit code, which is why Rust's ExitStatus::code() returns an Option

Summary - Different failures  gives sane status as 1



