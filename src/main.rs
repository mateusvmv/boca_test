use std::{fs::read_to_string, process::{Command, Stdio}, io::Write};
use console::Term;
use serde::Deserialize;

#[derive(Deserialize)]
struct Config {
	build_cmd: String,
	build_args: Option<Vec<String>>,
	exe_cmd: String,
	exe_args: Option<Vec<String>>,
	problem_dir: Option<String>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
	let mut args = std::env::args();
	let exe = args.next().unwrap();
	let cfg_path = std::env::args()
		.nth(1)
		.ok_or(format!("Usage: {exe} (CONFIG)"))?;
	let s = read_to_string(cfg_path)?;
	let cfg: Config = toml::from_str(&s)?;

	let mut term = Term::stdout();

	write!(term, "🕚 - Building...")?;
	let i = std::time::Instant::now();
	let output = Command::new(&cfg.build_cmd)
		.args(cfg.build_args.unwrap_or_default())
		.stderr(Stdio::piped())
		.spawn()?
		.wait_with_output()?;
	if !output.status.success() {
		let stderr = String::from_utf8(output.stderr)?;
		term.clear_line()?;
		writeln!(term, "❌ - Compilation error:\n{stderr}")?;
		return Ok(());
	}
	term.clear_line()?;
	writeln!(term, "✅ - Compiled in {:?}", i.elapsed())?;

	let problem_dir = cfg.problem_dir
		.unwrap_or_else(|| ".".into());
	let input_dir = format!("{problem_dir}/input");
	let output_dir = format!("{problem_dir}/output");

	let mut command = Command::new(&cfg.exe_cmd);
	command.stdin(Stdio::piped())
		.stdout(Stdio::piped())
		.stderr(Stdio::piped())
		.args(cfg.exe_args.unwrap_or_default());

	let mut dir: Vec<_> = std::fs::read_dir(input_dir)?.collect::<Result<Vec<_>, _>>()?;
	dir.sort_by_key(|d| d.file_name());
	for dir in dir {
		let input = read_to_string(dir.path())?;
		let file = dir.file_name();
		let test_name = file.to_str().unwrap();
		let correct = read_to_string(format!("{output_dir}/{}", test_name))?;
		
		write!(term, "🕚 - Running test {test_name}...")?;
		let i = std::time::Instant::now();
		let mut child = command.spawn()?;
		let stdin = child.stdin.as_mut().unwrap();
		stdin.write_all(input.as_bytes())?;

		let output = child.wait_with_output()?;
		let correct = correct.lines();
		let stderr = String::from_utf8(output.stderr)?;
		if !stderr.is_empty() {
			term.clear_line()?;
			writeln!(term, "🗒️ - STDERR:\n{stderr}")?;
		}
		let output = String::from_utf8(output.stdout)?;
		let mut output = output.lines();
		for c in correct {
			let o = output.next();
			match o {
				Some(o) => if o.trim() != c.trim() {
					term.clear_line()?;
					writeln!(term, "❌ - Test {test_name} - Expected '{c}', got '{o}'")?;
					return Ok(());
				},
				None => {
					term.clear_line()?;
					writeln!(term, "❌ - Test {test_name} - Output was too short")?;
					return Ok(());
				}
			}
		}
		term.clear_line()?;
		writeln!(term, "✅ - Test {test_name} - OK in {:?}", i.elapsed())?;
	}
	println!("Ok");
	Ok(())
}
