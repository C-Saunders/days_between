# Days Between
### A command line utility to give the number of days between two dates.

## API
`./days_between <start_date> <end_date>`

Date format: `YYYYMMDD`

## Setup

1. `git clone`
1. `cd days_between`
1. `cargo build --release`
1. The executable will be `days_between/target/release/days_between`

## Examples
```
$ <path>/days_between 19700101 19700101 => 0
```
```
$ <path>/days_between 19700101 20170901 => 17410
```

## Tips
If you commonly need to know the days since a particular date, make an alias.

Bash- put this in your .bashrc/.bash_profile/etc.
```
alias dase='<path>/days_between 19700101'
```

Fish- run in terminal
```
$ abbr --add dse '<path>/days_between 19700101'
```
