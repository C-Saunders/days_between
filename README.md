# Days Between
### A command line utility to give the number of days between two dates, or the date a number of days offset from a start date.

## API
`./days_between <start_date> [<end_date> OR --offset=integer]`

Date format: `YYYYMMDD` or `YYYY-MM-DD`. The output is `YYYYMMDD` for offset calculations.

## Setup

1. `git clone`
1. `cd days_between`
1. `cargo build --release`
1. The executable will be `days_between/target/release/days_between`

## Examples
```bash
# Use YYYYMMDD or YYYY-MM-DD
$ <path>/days_between 20180101 2018-01-01 => 0
```
```bash
$ <path>/days_between 19700101 20170901 => 17410
```
```bash
$ # -o is the short version of --offset
$ <path>/days_between 2018-02-09 -o=3 => 20180212
```
```bash
$ <path>/days_between 2018-02-09 -o=-3 => 20180206
```

## Tips
If you commonly need to know the days since a particular date, make an alias.

Bash - put this in your .bashrc/.bash_profile/etc.
```bash
alias dse='<path>/days_between 19700101'
```

Fish - run in terminal
```bash
$ abbr --add dse '<path>/days_between 19700101'
```
