use std::env;

fn return_args() -> Vec<String> {
    return env::args().collect()
}

pub fn parse_args() -> (String,f64) {
    let args = return_args();
    let mut percentage = 1.0;
    let mut scheme = "config.txt".to_owned();
    let args_len = args.len();
    if args_len>1 {
        if args[1].contains(".txt") {
            scheme = args[1].clone();
        } else {
            if args_len>2 {
                    if args[2].contains(".txt") {
                        scheme = args[2].clone()
                    }
            }
        }
        if args_len > 2 {
            match (args[1].parse::<f64>(),args[2].parse::<f64>()) {
                (Ok(perc),_) => percentage=perc,
                (_,Ok(perc)) => percentage=perc,
                _ => {},
            }
        } else {
            match args[1].parse::<f64>() {
                Ok(perc) => percentage=perc,
                _ => {},
            }
        }
    }

    return (scheme,percentage)
}