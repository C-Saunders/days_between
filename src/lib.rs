pub struct Inputs {
    pub start: String,
    pub end: String,
}

impl Inputs {
    pub fn new(mut args: std::env::Args) -> Result<Inputs, &'static str> {
        args.next();

        let start = match args.next() {
            Some(value) => value,
            None => return Err("Did not get a start date"),
        };

        let end = match args.next() {
            Some(value) => value,
            None => return Err("Did not get an end date"),
        };

        Ok(Inputs{
          start,
          end,
        })
    }
}