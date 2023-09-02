pub struct Args {
    pub in_file_path: String,
    pub out_file_path: String,
}

impl Args {
    pub fn build(args: &[String]) -> Result<Args, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let in_file_path = args[1].clone();
        let out_file_path = args[2].clone();
        
        Ok(Args { in_file_path, out_file_path })
    }
}