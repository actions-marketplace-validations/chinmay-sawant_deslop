use std::path::PathBuf;

use anyhow::Result;
use clap::{Parser, Subcommand};

use goslop::{benchmark_repository, scan_repository, BenchmarkOptions, ScanOptions};

#[derive(Debug, Parser)]
#[command(author, version, about = "Scan Go repositories for likely AI slop patterns")]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Debug, Subcommand)]
enum Command {
    Scan {
        path: PathBuf,
        #[arg(long)]
        json: bool,
        #[arg(long)]
        no_ignore: bool,
    },
    Bench {
        path: PathBuf,
        #[arg(long, default_value_t = 5)]
        repeats: usize,
        #[arg(long, default_value_t = 1)]
        warmups: usize,
        #[arg(long)]
        json: bool,
        #[arg(long)]
        no_ignore: bool,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Command::Scan {
            path,
            json,
            no_ignore,
        } => {
            let report = scan_repository(&ScanOptions {
                root: path,
                respect_ignore: !no_ignore,
            })?;

            if json {
                println!("{}", serde_json::to_string_pretty(&report)?);
            } else {
                print_scan_report(&report);
            }
        }
        Command::Bench {
            path,
            repeats,
            warmups,
            json,
            no_ignore,
        } => {
            let report = benchmark_repository(&BenchmarkOptions {
                root: path,
                repeats,
                warmups,
                respect_ignore: !no_ignore,
            })?;

            if json {
                println!("{}", serde_json::to_string_pretty(&report)?);
            } else {
                print_benchmark_report(&report);
            }
        }
    }

    Ok(())
}

fn print_scan_report(report: &goslop::ScanReport) {
    println!("goslop scan root: {}", report.root.display());
    println!("Go files discovered: {}", report.files_discovered);
    println!("Go files analyzed: {}", report.files_analyzed);
    println!("Functions fingerprinted: {}", report.functions_found);
    println!("Findings: {}", report.findings.len());
    println!(
        "Index summary: packages={} symbols={} imports={}",
        report.index_summary.package_count,
        report.index_summary.symbol_count,
        report.index_summary.import_count
    );
    println!("Parse failures: {}", report.parse_failures.len());
    println!(
        "Timings: discover={}ms parse={}ms index={}ms heuristics={}ms total={}ms",
        report.timings.discover_ms,
        report.timings.parse_ms,
        report.timings.index_ms,
        report.timings.heuristics_ms,
        report.timings.total_ms
    );

    for file in &report.files {
        println!();
        println!("{}", file.path.display());
        println!(
            "  package={} syntax_error={} functions={}",
            file.package_name.as_deref().unwrap_or("<unknown>"),
            file.syntax_error,
            file.functions.len()
        );

        for function in &file.functions {
            println!(
                "  - {} [{}:{}] complexity={} comment_ratio={:.2} symmetry={:.2} any={} iface={} calls={}",
                function.name,
                function.start_line,
                function.end_line,
                function.complexity_score,
                function.comment_to_code_ratio,
                function.symmetry_score,
                function.contains_any_type,
                function.contains_empty_interface,
                function.call_count
            );
        }
    }

    if !report.findings.is_empty() {
        println!();
        println!("Findings:");
        for finding in &report.findings {
            println!(
                "  - {}:{} {} [{}]",
                finding.path.display(),
                finding.start_line,
                finding.message,
                finding.rule_id
            );
        }
    }

    if !report.parse_failures.is_empty() {
        println!();
        println!("Parse failures:");
        for failure in &report.parse_failures {
            println!("  - {}: {}", failure.path.display(), failure.message);
        }
    }
}

fn print_benchmark_report(report: &goslop::BenchmarkReport) {
    println!("goslop bench root: {}", report.root.display());
    println!(
        "Warmups={} Repeats={} Files={} Functions={} Findings={}",
        report.warmups,
        report.repeats,
        report.files_analyzed,
        report.functions_found,
        report.findings_found
    );
    println!(
        "Total ms: min={} max={} mean={:.2} median={:.2}",
        report.total.min_ms,
        report.total.max_ms,
        report.total.mean_ms,
        report.total.median_ms
    );
    println!(
        "Parse ms: min={} max={} mean={:.2} median={:.2}",
        report.parse.min_ms,
        report.parse.max_ms,
        report.parse.mean_ms,
        report.parse.median_ms
    );
    println!(
        "Index ms: min={} max={} mean={:.2} median={:.2}",
        report.index.min_ms,
        report.index.max_ms,
        report.index.mean_ms,
        report.index.median_ms
    );
    println!(
        "Heuristics ms: min={} max={} mean={:.2} median={:.2}",
        report.heuristics.min_ms,
        report.heuristics.max_ms,
        report.heuristics.mean_ms,
        report.heuristics.median_ms
    );
}
