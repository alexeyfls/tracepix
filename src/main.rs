use std::process::ExitCode;

use tracepix::TracepixResult;

fn main() -> ExitCode {
    match tracepix::run() {
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

fn exit_code(result: &TracepixResult) -> u8 {
    match result {
        TracepixResult::Identical => 0,
        TracepixResult::Different { .. } => 21,
        TracepixResult::LayoutMismatch { .. } => 22,
    }
}

fn report(result: &TracepixResult) {
    match result {
        TracepixResult::Identical => {}
        TracepixResult::Different {
            diff_count,
            percentage,
        } => eprintln!("Images differ: {diff_count} pixels ({percentage:.2}%)"),
        TracepixResult::LayoutMismatch { reference, target } => eprintln!(
            "Images have different dimensions: {}x{} vs {}x{}",
            reference.0, reference.1, target.0, target.1
        ),
    }
}
