mod dir_growth;

// DirGrowth - Directory Growth Rate
// Outputs an ASCII graph of how much a particular folder has grown over time based on
// file size / date.
use clap::Parser;

#[derive(Parser)]
#[clap(author,
       version,
       about = "Directory Growth Rate",
       long_about = "Reports the growth rate of a provided directory")]
struct Args {
    #[clap(short, long, help = "Path of the folder to calculate the growth rate of")]
    path : String,
}

fn main() {
    let args = Args::parse();

    let mut growth_reporter = dir_growth::DirGrowthReporter::new();
    growth_reporter.load_file_info(&args.path);
    growth_reporter.console_output();


}