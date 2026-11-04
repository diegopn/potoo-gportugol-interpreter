use std::{
    fs,
    io::{self, BufRead, Write},
    process::ExitCode,
};

use potoo_gportugol_interpreter::RuntimeIo;

struct ConsoleIo;

impl RuntimeIo for ConsoleIo {
    fn write(&mut self, text: &str) {
        let _ = io::stdout().write_all(text.as_bytes());
        let _ = io::stdout().flush();
    }

    fn read_line(
        &mut self,
        prompt: &potoo_gportugol_interpreter::InputPrompt,
    ) -> Result<String, String> {
        eprint!("input ({}:{}): ", prompt.source.line, prompt.source.column);
        read_console_line()
    }
}

fn read_console_line() -> Result<String, String> {
    let mut input = String::new();
    if io::stdin()
        .lock()
        .read_line(&mut input)
        .map_err(|error| error.to_string())?
        == 0
    {
        return Err("standard input is closed".to_owned());
    }
    Ok(input.trim_end_matches(['\r', '\n']).to_owned())
}

fn main() -> ExitCode {
    let mut arguments = std::env::args_os();
    let program = arguments.next().unwrap_or_default();
    let Some(source_path) = arguments.next() else {
        eprintln!(
            "Usage: {} <source-file>",
            std::path::Path::new(&program).display()
        );
        return ExitCode::FAILURE;
    };
    if arguments.next().is_some() {
        eprintln!(
            "Usage: {} <source-file>",
            std::path::Path::new(&program).display()
        );
        return ExitCode::FAILURE;
    }

    let source = match fs::read_to_string(&source_path) {
        Ok(source) => source,
        Err(error) => {
            eprintln!(
                "cannot read {}: {error}",
                std::path::Path::new(&source_path).display()
            );
            return ExitCode::FAILURE;
        }
    };

    match potoo_gportugol_interpreter::execute(&source, &mut ConsoleIo) {
        Ok(_) => ExitCode::SUCCESS,
        Err(diagnostics) => {
            for diagnostic in diagnostics {
                eprintln!("{diagnostic}");
            }
            ExitCode::FAILURE
        }
    }
}
