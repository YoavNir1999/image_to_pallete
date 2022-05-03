use std::env;

fn return_args() -> Vec<String> {
    return env::args().collect()
}

pub fn parse_args() -> Operation {
    let args = return_args();
    let args_len = args.len();
    let mut op : Operation = Operation::MatchFromScheme;

    for idx in 1..args_len {
        if args[idx] == "extract" {
            if args_len>=idx+2 {
                op = Operation::Extract(args[idx+1].parse::<usize>().unwrap_or(5))
            } else {
                op = Operation::Extract(5)
            }
        
        } else if args[idx] == "match-image" {
            if args_len>=idx+2 {
                op = Operation::ExtractAndMatch(args[idx+1].parse::<usize>().unwrap_or(5))
            } else {
                op = Operation::ExtractAndMatch(5)
            }
        }
        
    }

    return op
}

#[derive(Debug)]
pub enum Operation {
    Extract(usize),
    ExtractAndMatch(usize),
    MatchFromScheme
}