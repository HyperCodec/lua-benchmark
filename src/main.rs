use std::{fs::OpenOptions, io::Write, path::{Path, PathBuf}};

use clap::Parser;
use indicatif::ProgressBar;
use mlua::{Function, Lua};

#[derive(Parser)]
#[command(
    name = "benchmark",
    about = "Benchmarks things",
    author = "HyperCodec"
)]
struct Cli {
    #[arg(help = "the path to the lua file")]
    path: PathBuf,

    #[arg(short, long, help = "The name of the function to benchmark", default_value = "bench")]
    bench_func: String,

    #[arg(short, long, help = "The name of the function that sets up stuff for the benchmark. Values returned by the setup function will be provided as args for the benchmark function. Setup time is not included in the bench time.")]
    setup_func: Option<String>,

    #[arg(long, help = "The minimum N", default_value = "1")]
    min: u64,

    #[arg(long, help = "The maxmimum N", default_value = "10000")]
    max: u64,

    #[arg(short, long, help = "The number of times to repeat each N. A higher number results in more stable/accurate results.", default_value = "50")]
    repeats: usize,

    #[arg(short, long, help = "The path to the output .csv file")]
    output: PathBuf,
}

const BENCH: &str = include_str!("bench.lua");

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Cli::parse();

    println!("loading lua files");
    let lua = Lua::new();
    lua.load(BENCH).exec()?;

    let bench_avg_time: Function = lua.globals().get("benchAvgTime")?;

    let bench_test_text = std::fs::read_to_string(args.path)?;
    lua.load(bench_test_text).exec()?;

    let bench_test: Function = lua.globals().get(args.bench_func)?;

    let setup: Option<Function> = if let Some(name) = args.setup_func {
        Some(lua.globals().get(name)?)
    } else {
        None
    };

    let mut timings = Vec::with_capacity((args.max-args.min+1) as usize);
    
    println!("running bench test");

    let pb = ProgressBar::new(args.max-args.min+1);
    
    for n in args.min..=args.max {
        let output: f32 = bench_avg_time.call((bench_test.clone(), setup.clone(), n, args.repeats))?;
        timings.push(output);
        pb.inc(1);
    }

    pb.finish_and_clear();

    println!("dumping output to csv");

    write_csv(args.output, timings, args.min)?;

    println!("done");

    Ok(())
}

fn write_csv(path: impl AsRef<Path>, timings: Vec<f32>, min: u64) -> std::io::Result<()> {
    let mut file = OpenOptions::new()
        .create_new(true)
        .append(true)
        .open(path)?;

    file.write_all(b"N,time\n")?;

    for (i, t) in timings.into_iter().enumerate() {
        let row = format!("{},{t}\n", i as u64 + min);
        file.write_all(row.as_bytes())?;
    }

    Ok(())
}
