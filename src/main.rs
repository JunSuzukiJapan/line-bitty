#![allow(non_snake_case)]

extern crate rustc_serialize;
extern crate docopt;

use docopt::Docopt;

static USAGE: &'static str = "
Usage:
  line-botty create [--path <path>] [--name <name>]
  line-botty npm (add | remove) [<args>...]
  line-botty build
  line-botty install --url <url>
  line-botty deploy [--stage <stage>] [--region <region>] [--noDeploy] [--verbose]
  line-botty deploy function [--stage <stage>] [--region <region>] [--noDeploy] [--verbose]
  line-botty deploy list [--stage <stage>] [--region <region>] [--noDeploy] [--verbose]
  line-botty invoke  [--function <name>] [--stage <stage>] [--region <region>] [--path <path>] [--type <type>] [--log] [--data]
  line-botty invoke local [--function <name>] [--stage <stage>] [--region <region>] [--path <path>] [--type <type>] [--log] [--data]
  line-botty info [--stage <stage>] [--region <region>] [--verbose]
  line-botty logs [--function <name>] [--stage <stage>] [--region <region>] [--tail] [--startTime <time>] [--filter <pattern>] [--interval <milliseconds>]
  line-botty metrics [--function <function>] [--stage <stage>] [--region <region>] [--startTime <time>] [--endTime <time>]
  line-botty remove [--stage <stage>] [--region <region>] [--verbose]
  line-botty rollback [--timestamp <timestamp>] [--verbose]
  line-botty slstats (--enable | --disable)
  line-botty (--help | --version)

Options:
  -u, --url <url>
  -s, --stage <stage>
  -r, --region <region>
      --noDeploy
  -v, --verbose
     --type <type>
  -l, --log
  -d, --data
  -p, --path <path>
  -n, --name <name>
  -f, --function <name>
  -t, --tail
  -s, --startTime <time>
  -e, --endTime <time>
      --filter <filter>
  -i, --interval <milliseconds>
      --timestamp <timestamp>
      --enable
      --disable
  -h, --help
      --version
";

#[derive(RustcDecodable, Debug)]
pub struct Args {
    cmd_create: bool,
    cmd_build: bool,
    cmd_install: bool,
    cmd_deploy: bool,
    cmd_function: bool,
    cmd_invoke: bool,
    cmd_local: bool,
    cmd_info: bool,
    cmd_logs: bool,
    cmd_npm: bool,
    cmd_metrics: bool,
    cmd_add: bool,
    cmd_remove: bool,
    cmd_rollback: bool,

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
}

fn main() {
    let args: Args = Docopt::new(USAGE)
            .and_then(|d| d.decode())
            .unwrap_or_else(|e| e.exit());
    println!("{:?}", args);
}