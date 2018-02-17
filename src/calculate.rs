use inputs::Inputs;
use chrono::{Date, Duration, Utc};
use DateFormat;

struct Range {
    start: Date<Utc>,
    end: Date<Utc>,
    current: Date<Utc>,
}

impl Range {
    fn new(start: Date<Utc>, end: Date<Utc>) -> Range {
        Range {
            start,
            end,
            current: start,
        }
    }

    fn difference(&self) -> i64 {
        self.end.signed_duration_since(self.start).num_days()
    }
}

impl Iterator for Range {
    type Item = Date<Utc>;
    fn next(&mut self) -> Option<Self::Item> {
        let start = self.start;
        let end = self.end;
        let one_day = Duration::days(1);

        if start.lt(&end) {
            let old_current = self.current;
            self.current = self.current.checked_add_signed(one_day).unwrap();

            if old_current.le(&end) {
                Some(old_current)
            } else {
                None
            }
        } else {
            let old_current = self.current;
            self.current = self.current.checked_sub_signed(one_day).unwrap();

            if old_current.ge(&end) {
                Some(old_current)
            } else {
                None
            }
        }
    }
}

fn calculate_range(inputs: &Inputs) -> Range {
    let start = inputs.start.unwrap();
    let end = if inputs.end.is_some() {
        inputs.end.unwrap()
    } else {
        let to_add = Duration::days(inputs.offset.unwrap());
        start.checked_add_signed(to_add).unwrap()
    };

    Range::new(start, end)
}

pub fn print_output(inputs: Inputs) {
    let range = calculate_range(&inputs);
    let output_format = match inputs.format_type {
        DateFormat::NoDashes => "%Y%m%d",
        DateFormat::Dashes => "%Y-%m-%d",
        DateFormat::Custom(ref val) => &val,
    };

    if inputs.list_output {
        for date in range {
            println!("{}", date.format(output_format))
        }
    } else {
        if inputs.end.is_some() {
            println!("{}", range.difference())
        } else {
            println!("{}", range.end.format(output_format));
        }
    }
}
