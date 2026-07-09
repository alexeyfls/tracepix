use std::process::ExitCode;

// Dev-only harness for running tracepix locally.
fn main() -> anyhow::Result<ExitCode> {
    tracepix::run()
}
