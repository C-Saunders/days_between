use inputs::Inputs;

pub fn print_output(inputs: &Inputs) {
    if inputs.end.is_some() {
        println!("{}", inputs.end.unwrap().signed_duration_since(inputs.start.unwrap()).num_days())
    } else if inputs.plus_days.is_some() {
        println!("not implemented")
    } else if inputs.plus_days.is_some() {
        println!("not implemented")
    } else {
        eprintln!("Invalid input")
    }
}
