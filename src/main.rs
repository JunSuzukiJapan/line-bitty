#![allow(non_snake_case)]

extern crate rustc_serialize;
extern crate docopt;

use std::env;
use std::path::Path;
use std::process::Command;
use docopt::Docopt;

static USAGE: &'static str = "
Usage:
  line-botty create [--path <path>] [--name <name>]
  line-botty build
  line-botty npm (install | uninstall) <args>...
  line-botty sls install --url <url>
  line-botty sls deploy [--stage <stage>] [--region <region>] [--noDeploy] [--verbose]
  line-botty sls deploy function [--stage <stage>] [--region <region>] [--noDeploy] [--verbose]
  line-botty sls deploy list [--stage <stage>] [--region <region>] [--noDeploy] [--verbose]
  line-botty sls invoke  [--function <name>] [--stage <stage>] [--region <region>] [--path <path>] [--type <type>] [--log] [--data]
  line-botty sls invoke local [--function <name>] [--stage <stage>] [--region <region>] [--path <path>] [--type <type>] [--log] [--data]
  line-botty sls info [--stage <stage>] [--region <region>] [--verbose]
  line-botty sls logs [--function <name>] [--stage <stage>] [--region <region>] [--tail] [--startTime <time>] [--filter <pattern>] [--interval <milliseconds>]
  line-botty sls metrics [--function <function>] [--stage <stage>] [--region <region>] [--startTime <time>] [--endTime <time>]
  line-botty sls remove [--stage <stage>] [--region <region>] [--verbose]
  line-botty sls rollback [--timestamp <timestamp>] [--verbose]
  line-botty sls slstats (--enable | --disable)
  line-botty (--help | --version)

Options:
  -d, --data
  -e, --endTime <time>
  -f, --function <name>
  -h, --help
  -i, --interval <milliseconds>
  -l, --log
  -n, --name <name>
  -p, --path <path>
  -r, --region <region>
  -s, --stage <stage>
  -t, --tail
  -u, --url <url>
  -v, --verbose
      --disable
      --enable
      --filter <filter>
      --noDeploy
      --startTime <time>
      --timestamp <timestamp>
      --type <type>
      --version
";

#[derive(RustcDecodable, Debug)]
pub struct Args {
    cmd_create: bool,
    cmd_build: bool,
    cmd_npm: bool,
    cmd_remove: bool,
    cmd_sls: bool,
    cmd_install: bool,
    cmd_uninstall: bool,
    cmd_deploy: bool,
    cmd_function: bool,
    cmd_invoke: bool,
    cmd_local: bool,
    cmd_info: bool,
    cmd_logs: bool,
    cmd_metrics: bool,
    cmd_rollback: bool,
    cmd_slstats: bool,

    flag_url: String,
    flag_stage: String,
    flag_region: String,
    flag_noDeploy: bool,
    flag_verbose: bool,
    flag_type: String,
    flag_log: bool,
    flag_data: bool,
    flag_tail: bool,
    flag_startTime: String,
    flag_endTime: String,
    flag_filter: String,
    flag_interval: i64,
    flag_path: String,
    flag_name: String,
    flag_function: String,
    flag_timestamp: String,
    flag_enable: bool,
    flag_disable: bool,
    flag_help: bool,
    flag_version: bool,

    arg_args: Vec<String>,
}

fn main() {
    let args: Args = Docopt::new(USAGE)
            .and_then(|d| d.decode())
            .unwrap_or_else(|e| e.exit());
    //println!("{:?}", args);

    if args.cmd_sls {
        sls_commands(args);
        std::process::exit(0);
     } else if args.cmd_npm {
        npm_commands(args);
        std::process::exit(0);
    } else if args.cmd_create {
        create_commands(args);
    } else if args.cmd_build {
        build_commands();
    }
}

fn chdir_src() {
    let root = Path::new("./src");
    if env::set_current_dir(&root).is_err() {
        panic!("didnot change directory to ./src");
    }
}

fn create_commands(args: Args){

}

fn build_commands(){

}

fn npm_commands(args: Args){
    chdir_src();

    let mut cmd = Command::new("npm");
    cmd.arg("--save");

    if args.cmd_install {
        cmd.arg("install");
    } else if args.cmd_uninstall {
        cmd.arg("uninstall");
    }
    for arg in args.arg_args {
        cmd.arg(arg);
    }
    cmd.output()
       .expect("failed to execute process");
}

fn sls_commands(args: Args){
    if args.cmd_install {

    } else if args.cmd_deploy {

    } else if args.cmd_invoke {
    } else if args.cmd_info {

    } else if args.cmd_logs {

    } else if args.cmd_metrics {
    } else if args.cmd_remove {
    } else if args.cmd_rollback {

    } else if args.cmd_slstats {
    }
}