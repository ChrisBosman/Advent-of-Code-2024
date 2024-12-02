@echo off
if exist "inputs/day%1.txt" (
    echo file exists
) else (
    echo Creating input file
    type nul > "inputs/day%1.txt"
)
@echo off
if exist "inputs_tests/day%1.txt" (
    echo file exists
) else (
    echo Creating test input file
    type nul > "inputs_tests/day%1.txt"
)