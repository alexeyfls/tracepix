use clap::Parser;
use cli_arguments::TracepixCliArguments;
use std::process::ExitCode;
use tracepix::{CompareOptions, Comparison, TracepixResult, compare};

mod cli_arguments;

fn main() -> ExitCode {
    match run() {
        Ok(result) => {
            report(&result);
            ExitCode::from(exit_code(&result))
        }
        Err(err) => {
            eprintln!("error: {err:#}");
            ExitCode::from(1)
        }
    }
}

fn run() -> anyhow::Result<TracepixResult> {
    let args = TracepixCliArguments::parse();
    let Comparison { result, diff_image } = compare(
        &args.reference_img,
        &args.target_img,
        &CompareOptions {
            threshold: args.threshold,
            detect_antialiasing: args.antialiasing,
            emit_diff_image: args.diff_output.is_some(),
        },
    )?;

    if let (Some(path), Some(image)) = (args.diff_output, diff_image) {
        image.save(&path)?;
        eprintln!("Diff image saved to: {}", path.display());
    }

    Ok(result)
}

fn exit_code(result: &TracepixResult) -> u8 {
    match result {
        TracepixResult::Identical => 0,
        TracepixResult::Different { .. } => 21,
        TracepixResult::LayoutMismatch { .. } => 22,
    }
}

fn report(result: &TracepixResult) {
    match result {
        TracepixResult::Identical => println!("Images are identical"),
        TracepixResult::Different {
            diff_count,
            percentage,
        } => println!("Images differ: {diff_count} pixels ({percentage:.2}%)"),
        TracepixResult::LayoutMismatch { reference, target } => println!(
            "Images have different dimensions: {}x{} vs {}x{}",
            reference.0, reference.1, target.0, target.1
        ),
    }
}
