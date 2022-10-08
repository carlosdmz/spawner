use crate::lib;

pub fn help() {
    println!("
usage: spawner [OPTIONS] --mode MODE COMMAND

OPTIONS:

--start-url\tURL where spawner will crawl first
--worker\tAddress to worker with port, if using distributed mode

MODE:

standalone\tSingle node executor
distributed\tMultiple node executor, must have --worker OPT set

COMMAND:

start\tWell, start crawling, right?
    ");
}

fn check_invalid_arg(i: usize, args: &Vec<String>) {
    if args.len() == i + 1 {
        return;
    }
    if args.get(i).unwrap().starts_with("-") && args.get(i + 1).unwrap().starts_with("-") {
        help();
        lib::exit("invalid argument provided");
    }
}

fn get_arg_value(i: usize, args: &Vec<String>) -> &String {
    let ret: &String = args.get(i + 1).unwrap();
    ret
}

pub struct Program {
    pub command: Option<String>,
    pub opts: ProgramOpts
}

impl Program {
    pub fn new(args: &Vec<String>) -> Program {
        if args.len() == 1 {
            help();
            lib::exit("insufficient number of arguments");
        }
        let mut command: Option<String> = Default::default();
        for arg in args {
            match arg.as_ref() {
                "start" => command = Some(arg.to_owned()),
                _ => {},
            }
        }

        match command {
            Some(_) => {},
            None => {
                help();
                lib::exit("no command was provided");
            }
        }

        Program{
            command,
            opts: ProgramOpts::new(args),
        }
    }
}


pub struct ProgramOpts {
    pub start_url: Option<String>,
    pub mode: Option<String>,
    pub worker: Option<String>
}

impl ProgramOpts {
    fn new(args: &Vec<String>) -> ProgramOpts {
        let mut start_url: Option<String> = Default::default();
        let mut mode: Option<String> = Default::default();
        let mut worker: Option<String> = Default::default();

        let unmoved_args: &Vec<String> = args;

        for (i, arg) in args.clone().into_iter().enumerate() {
            if i == 0 {
                continue;
            }
            check_invalid_arg(i, &unmoved_args);
            match arg.as_ref() {
                "--start-url" => {
                    start_url = Some(get_arg_value(i, unmoved_args).clone());
                },
                "--mode" => {
                    mode = Some(get_arg_value(i, unmoved_args).clone());
                },
                "--worker" => {
                    worker = Some(get_arg_value(i, unmoved_args).clone());
                }
                _ => {},
            }
        }

        match mode {
            Some(_) => {},
            None => {
                help();
                lib::exit("mode argument is required");
            }
        }

        match start_url {
            Some(_) => {},
            None => {
                help();
                lib::exit("start_url is required");
            }
        }

        ProgramOpts {
            start_url,
            mode,
            worker
        }
    }
}