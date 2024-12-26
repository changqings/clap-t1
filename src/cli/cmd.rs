use clap::{value_parser, Arg, ArgMatches, Command};
pub fn aws() -> Command {
    // define command
    Command::new("aws-cli")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .about("aws")
}

pub fn aws_s3() -> Command {
    Command::new("s3")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommands([
            Command::new("get")
                .alias("g")
                .about("get")
                .arg_required_else_help(true)
                .args([
                    Arg::new("name")
                        // .value_parser(value_parser!(String))
                        .default_value(String::from("hello_world"))
                        .help("s3 name oss://xx")
                        .short('n'),
                    Arg::new("age")
                        .value_parser(value_parser!(String))
                        .default_value(String::from("20"))
                        .help("created time")
                        .short('a'),
                    Arg::new("enable")
                        .value_parser(value_parser!(bool))
                        .default_value("false")
                        .help("enable")
                        .short('e'),
                ]),
            Command::new("list")
                .alias("l")
                .about("list")
                .arg_required_else_help(true)
                .args([Arg::new("ap").short('a')]),
        ])
}
pub fn aws_s3_handler(arg_match: &ArgMatches) -> Option<()> {
    // s3
    if let Some(s3) = arg_match.subcommand_matches("s3") {
        if let Some(("get", args)) = s3.subcommand() {
            let name = args.get_one::<String>("name").unwrap();
            println!("ok, s3 get name = {}", name);
            let age = args.get_one::<String>("age").unwrap();
            println!("ok, s3 get age = {}", age);
            let enable = args.get_one::<bool>("enable").unwrap();
            println!("ok, s3 enable = {}", enable);
        }
        if let Some(("list", args)) = s3.subcommand() {
            let ap = args.get_one::<String>("ap").unwrap();
            println!("ok, s3 list ap = {}", ap);
        }
        return Some(());
    }
    None
}

pub fn aws_ec2() -> Command {
    Command::new("ec2")
        .alias("e")
        .about("usage: ec2, e")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommands([Command::new("tag")
            .alias("t")
            .subcommand_required(true)
            .arg_required_else_help(true)
            .subcommands([
                Command::new("get")
                    .alias("g")
                    .subcommand_required(true)
                    .arg_required_else_help(true)
                    .args([Arg::new("example").short('e')]),
                Command::new("update")
                    .alias("u")
                    .subcommand_required(true)
                    .arg_required_else_help(true)
                    .args([Arg::new("ips").short('i'), Arg::new("tags").short('t')]),
            ])])
}
pub fn aws_ec2_handler(arg_match: &ArgMatches) -> Option<()> {
    // ec2
    if let Some(ec2) = arg_match.subcommand_matches("ec2") {
        if let Some(("tag", tag)) = ec2.subcommand() {
            if let Some(("update", args)) = tag.subcommand() {
                let ips = args.get_one::<String>("ips").unwrap();
                let tags = args.get_one::<String>("tags").unwrap();
                println!("ok, ec2 stg get ips = {}, tags = {}", ips, tags);
            }

            if let Some(("get", args)) = tag.subcommand() {
                let example = args.get_one::<String>("example").unwrap();
                println!("ok, ec2 get example = {}", example);
            }
        }
        return Some(());
    }
    None
}
pub fn aws_cmd() {
    let root_cmd = aws();
    let s3_cmd = aws_s3();
    let ec2_cmd = aws_ec2();

    let matches = root_cmd.subcommands([s3_cmd, ec2_cmd]).get_matches();

    if let Some(_) = aws_s3_handler(&matches) {
        return;
    }
    if let Some(_) = aws_ec2_handler(&matches) {
        return;
    }
}
