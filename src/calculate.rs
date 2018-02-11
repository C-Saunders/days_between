use inputs::Inputs;
use chrono::Duration;
use ::DateFormat;

pub fn print_output(inputs: &Inputs) {
    let start_date = inputs.start.unwrap();

    if inputs.end.is_some() {
        println!("{}", inputs.end.unwrap().signed_duration_since(start_date).num_days())
    } else if inputs.offset.is_some() {
        let to_add = Duration::days(inputs.offset.unwrap());
        let output_format = match inputs.format_type {
            DateFormat::NoDashes => "%Y%m%d",
            DateFormat::Dashes => "%Y-%m-%d" ,
        };
        println!("{}", start_date.checked_add_signed(to_add).unwrap().format(output_format));
    } else {
        eprintln!("Invalid input")
    }
}
