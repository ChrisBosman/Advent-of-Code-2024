@echo off
for /l %%i in (1,1,25) do (
    echo use colored^:^:Colorize; > days/day%%i.rs
    echo. >> days/day%%i.rs
    echo pub^(crate^) >> days/day%%i.rs
    echo fn run^(input: String^) -^> ^(usize, usize^){ >> days/day%%i.rs
    echo     println^!^("{}","Day %%i".bright_green^(^).bold^(^)^); >> days/day%%i.rs
    echo     return ^(0, 0^); >> days/day%%i.rs
    echo } >> days/day%%i.rs
)