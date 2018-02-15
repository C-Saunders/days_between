# Days Between
### A command line utility for working with date ranges.

## Examples
```bash
# Calculate the number of days between dates
# Use YYYYMMDD or YYYY-MM-DD
$ <path>/days_between 20180101 2018-01-01 # 0
```
```bash
# A more practical example
$ <path>/days_between 19700101 20170901 # 17410
```
```bash
# Showing the short offset option
$ <path>/days_between 20180209 -o=3 # 20180212
```
```bash
# Offset can be negative. It can also come first.
$ <path>/days_between -o=-3 2018-02-09 # 2018-02-06
```
```bash
# The list option prints start to end
$ <path>/days_between 2018-02-09 -o=3 -l
# 2018-02-09
# 2018-02-10
# 2018-02-11
# 2018-02-12

$ <path>/days_between 2018-02-09 -o=-3 -l
# 2018-02-09
# 2018-02-08
# 2018-02-07
# 2018-02-06

$ <path>/days_between 2017-12-29 2018-01-01 -l
# 2017-12-29
# 2017-12-30
# 2017-12-31
# 2018-01-01

$ <path>/days_between days_between 2018-01-01 2017-12-29 -l
# 2018-01-01
# 2017-12-31
# 2017-12-30
# 2017-12-29
```

## API
* `./days_between <start_date> [<end_date> OR --offset=integer] [options]`
* `./days_between -o=integer <start_date>`

Date format: `YYYYMMDD` or `YYYY-MM-DD`. The output format matches the start date format for offset calculations.

## Options
* -h, --help Print help info
* -l, --list Prints all dates in the calculated range (e.g. for `xargs` input)
* -o, --offset <offset> Calculate the date this many days offset the start date. Positive or negative integer.


## Setup

1. `git clone`
1. `cd days_between`
1. `cargo build --release`
1. The executable will be `days_between/target/release/days_between`

## Tips
If you commonly need to know the days since a particular date, make an alias.

Bash - put this in your .bashrc/.bash_profile/etc. to easily calculate the days since Jan. 1, 1970.
```bash
alias dse='<path>/days_between 19700101'
```

Fish - run in terminal
```bash
$ abbr --add dse '<path>/days_between 19700101'
```
