use clap::{value_parser, Arg, Command};

fn main() {
    // define command and args
    let matches = Command::new("rs-cli")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommands([
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
                        .args([
                            Arg::new("ap").short('a')
                        ]),
                ]),
            Command::new("ec2")
                .alias("e")
                .about("usage: ec2, e")
                .subcommand_required(true)
                .arg_required_else_help(true)
                .subcommands([
                    Command::new("tag")
                        .alias("t")
                        .subcommand_required(true)
                        .arg_required_else_help(true)
                        .subcommands([
                            Command::new("get")
                                .alias("g")
                                .subcommand_required(true)
                                .arg_required_else_help(true)
                                .args([
                                    Arg::new("example").short('e')
                                ]),
                            Command::new("update")
                                .alias("u")
                                .subcommand_required(true)
                                .arg_required_else_help(true)
                                .args([
                                    Arg::new("ips").short('i'),
                                    Arg::new("tags").short('t')
                                ]),
                        ])
                ]),
        ])
        .get_matches();
    // parse command and args
    // s3
    if let Some(s3) = matches.subcommand_matches("s3") {
        if let Some(("get", a)) = s3.subcommand() {
            let name = a.get_one::<String>("name").unwrap();
            println!("ok, s3 get name = {}", name);
            let age = a.get_one::<String>("age").unwrap();
            println!("ok, s3 get age = {}", age);
            let enable = a.get_one::<bool>("enable").unwrap();
            println!("ok, s3 enable = {}", enable);
        }
        if let Some(("list", a)) = s3.subcommand() {
            let ap = a.get_one::<String>("ap").unwrap();
            println!("ok, s3 list ap = {}", ap);
        }
    }

    // ec2
    if let Some(ec2) = matches.subcommand_matches("ec2") {
        if let Some(("tag", m)) = ec2.subcommand() {
            if let Some(("update", a)) = m.subcommand() {
                let ips = a.get_one::<String>("ips").unwrap();
                let tags = a.get_one::<String>("tags").unwrap();
                println!("ok, ec2 stg get ips = {}, tags = {}", ips, tags);
            }

            if let Some(("get", a)) = m.subcommand() {
                let example = a.get_one::<String>("example").unwrap();
                println!("ok, ec2 get example = {}", example);
            }
        }
    }
}
