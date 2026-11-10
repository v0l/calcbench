use std::process::ExitCode;

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if args.is_empty() {
        return match gui() {
            Ok(()) => ExitCode::SUCCESS,
            Err(e) => {
                eprintln!("calcbench: {e}");
                ExitCode::FAILURE
            }
        };
    }
    match calcbench::cli::run(&args) {
        Ok(out) => {
            println!("{out}");
            ExitCode::SUCCESS
        }
        Err(e) => {
            eprintln!("error: {e}");
            ExitCode::FAILURE
        }
    }
}

#[cfg(not(target_os = "android"))]
fn gui() -> Result<(), String> {
    calcbench::app::run().map_err(|e| e.to_string())
}

#[cfg(target_os = "android")]
fn gui() -> Result<(), String> {
    Err("not available".into())
}
