#![allow(non_snake_case)]
#![allow(unused_must_use)]

extern crate rustc_serialize;
extern crate docopt;
extern crate git2;
extern crate copperline;
extern crate json_flex;

use std::env;
use std::path::Path;
use std::process::Command;
use std::fs;
use std::fs::File;
use std::io::{Read, Write};
use docopt::Docopt;
use git2::Repository;
use copperline::Copperline;
use json_flex::JFObject;
use std::collections::HashMap;

static USAGE: &'static str = "
Usage:
  line-botty create [--path <path>] [--name <name>] [--token <token>]
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
  -t, --token <token>
  -u, --url <url>
  -v, --verbose
      --disable
      --enable
      --filter <filter>
      --noDeploy
      --startTime <time>
      --tail
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

    flag_token: Option<String>,
    flag_url: Option<String>,
    flag_stage: Option<String>,
    flag_region: Option<String>,
    flag_noDeploy: bool,
    flag_verbose: bool,
    flag_type: Option<String>,
    flag_log: bool,
    flag_data: bool,
    flag_tail: bool,
    flag_startTime: Option<String>,
    flag_endTime: Option<String>,
    flag_filter: Option<String>,
    flag_interval: Option<i64>,
    flag_path: Option<String>,
    flag_name: Option<String>,
    flag_function: Option<String>,
    flag_timestamp: Option<String>,
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

fn chdir_dest() {
    let root = Path::new("./dest");
    if env::set_current_dir(&root).is_err() {
        panic!("didnot change directory to ./dest");
    }
}

//   line-botty create [--path <path>] [--name <name>] [--token <token>]
fn create_commands(args: Args){
    let mut token: String = String::new();
    if let Some(tok) = args.flag_token {
        token.push_str(&tok);        
    }else{
        let cfg = copperline::Config {
            encoding: copperline::Encoding::Utf8,
            mode: copperline::EditMode::Emacs,
        };

        let mut cl = Copperline::new();
        if let Ok(line) = cl.read_line("input your line message API access token: ", &cfg) {
            //cl.add_history(line);
            token.push_str(&line);
        }
    }

    let mut p = String::from(".");
    if let Some(path) = args.flag_path {
        p = String::from(path);
    }

    // git clone
    let url = "https://github.com/JunSuzukiJapan/line-bot-ts-template.git";
    let _ = match Repository::clone(url, &p) {
        Ok(repo) => repo,
        Err(e) => panic!("failed to clone: {}", e),
    };    

    // create src/token.ts
    let mut f = File::create("./src/token.ts").unwrap();
    f.write_all(format!("export const accessToken = '{}';", token).as_bytes()).unwrap();

    // fix package.json
    let mut f = File::open("./src/package.json").unwrap();
    let mut s = String::new();
    f.read_to_string(&mut s);

    let mut jf = json_flex::decode(s);

    if let Some(name) = args.flag_name {
        let mut map: HashMap<std::string::String, json_flex::JFObject> = HashMap::new();

        if let Some(m) = jf.into_hashmap() {
            for (key, val) in m {
                if key == "name" {
                    map.insert(key.clone(), JFObject::String(name.clone()));
                }else{
                    map.insert(key.clone(), val.clone());
                }
            }
        }
        jf = Box::new(json_flex::JFObject::Dictionary(map));
    }

    //
    // write to src/package.json
    //
    let json_path = "./src/package.json";
    let _ = fs::remove_file(json_path);
    let mut file = File::create(json_path).unwrap();

    file.write(b"{\n");

    if let Some(m) = jf.into_hashmap() {
        if let Some(obj) = m.get("name") {
            let s = obj.into_string().unwrap();
            file.write_fmt(format_args!("  \"name\": \"{}\",\n", s));
        }
        if let Some(obj) = m.get("version") {
            let s = obj.into_string().unwrap();
            file.write_fmt(format_args!("  \"version\": \"{}\",\n", s));
        }
        if let Some(obj) = m.get("description") {
            let s = obj.into_string().unwrap();
            file.write_fmt(format_args!("  \"description\": \"{}\",\n", s));
        }
        if let Some(obj) = m.get("author") {
            let s = obj.into_string().unwrap();
            file.write_fmt(format_args!("  \"author\": \"{}\",\n", s));
        }
        if let Some(obj) = m.get("license") {
            let s = obj.into_string().unwrap();
            file.write_fmt(format_args!("  \"license\": \"{}\",\n", s));
        }
        if let Some(obj) = m.get("main") {
            let s = obj.into_string().unwrap();
            file.write_fmt(format_args!("  \"main\": \"{}\",\n", s));
        }

        // dependencies
        // scripts
        file.write(
br#"  "dependencies": {
    "superagent": "^3.3.1"
  },
  "scripts": {
    "test": "echo \"Error: no test specified\" && exit 1"
  }
"#);
    }

    file.write(b"}");
}

fn build_commands(){
    let mut cmd = Command::new("npm");
    cmd.arg("install")
       .stdin(std::process::Stdio::inherit())
       .stdout(std::process::Stdio::inherit())
       .stderr(std::process::Stdio::inherit())
       .output()
       .expect("failed to execute process");

    let mut cmd = Command::new("make");
    cmd.arg("build")
       .stdin(std::process::Stdio::inherit())
       .stdout(std::process::Stdio::inherit())
       .stderr(std::process::Stdio::inherit())
       .output()
       .expect("failed to execute process");
}

//   line-botty npm (install | uninstall) <args>...
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
    cmd.stdin(std::process::Stdio::inherit())
       .stdout(std::process::Stdio::inherit())
       .stderr(std::process::Stdio::inherit())
       .output()
       .expect("failed to execute process");
}

/*
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
*/
fn sls_commands(args: Args){
    build_commands();

    chdir_dest();
    let mut cmd = Command::new("serverless");

    if args.cmd_install {
        cmd.arg("install");
        if let Some(url) = args.flag_url {
            cmd.arg("--url")
               .arg(url);
        }

    } else if args.cmd_deploy {
        cmd.arg("deploy");
        if args.flag_noDeploy {
            cmd.arg("--noDeploy");
        }

    } else if args.cmd_invoke {
        cmd.arg("invoke");
        if args.cmd_local {
            cmd.arg("local");
        }
        if let Some(path) = args.flag_path {
            cmd.arg("--path")
               .arg(path);
        }
        if let Some(typ) = args.flag_type {
            cmd.arg("--type")
               .arg(typ);
        }
        if args.flag_log {
            cmd.arg("--log");
        }
        if args.flag_data {
            cmd.arg("--data");
        }

    } else if args.cmd_info {
        cmd.arg("info");

    } else if args.cmd_logs {
        cmd.arg("logs");
        if args.flag_tail {
            cmd.arg("--tail");
        }
        if let Some(filter) = args.flag_filter {
            cmd.arg("--filter")
               .arg(filter);
        }
        if let Some(ms) = args.flag_interval {
            cmd.arg("--interval")
               .arg(ms.to_string());
        }

    } else if args.cmd_metrics {
        cmd.arg("metrics");

    } else if args.cmd_remove {
        cmd.arg("remove");

    } else if args.cmd_rollback {
        cmd.arg("rollback");

        if let Some(timestamp) = args.flag_timestamp {
            cmd.arg("--timestamp")
               .arg(timestamp);
        }
    } else if args.cmd_slstats {
        cmd.arg("slstats");

        if args.flag_enable {
            cmd.arg("enable");
        }else {
            cmd.arg("disable");
        }
    }

    if let Some(function) = args.flag_function {
        cmd.arg("--function")
            .arg(function);
    }
    if let Some(stage) = args.flag_stage {
        cmd.arg("--stage")
            .arg(stage);
    }
    if let Some(region) = args.flag_region {
        cmd.arg("--region")
            .arg(region);
    }
    if let Some(time) = args.flag_startTime {
        cmd.arg("--startTime")
           .arg(time);
    }
    if let Some(time) = args.flag_endTime {
        cmd.arg("--endTime")
           .arg(time);
    }
    // option --verbose
    if args.flag_verbose {
        cmd.arg("--verbose");
    }

    cmd.stdin(std::process::Stdio::inherit())
       .stdout(std::process::Stdio::inherit())
       .stderr(std::process::Stdio::inherit())
       .output()
       .expect("failed to execute process");
}