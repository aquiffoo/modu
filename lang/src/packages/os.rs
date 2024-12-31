use std::collections::HashMap;
use std::process::Command;
use crate::ast::AST;

pub fn exec(args: Vec<AST>, _context: &mut HashMap<String, AST>) -> Result<AST, String> {
	if args.len() != 1 {
		return Err("os.exec requires exactly one argument".to_string());
	}

	let command = match &args[0] {
		AST::String(value) => value,
		_ => return Err("os.exec argument must be a string".to_string()),
	};

	#[cfg(windows)]
	let output = Command::new("cmd")
		.args(["/C", command])
		.output();

	#[cfg(not(windows))]
	let output = Command::new("sh")
		.args(["-c", command])
		.output();

	match output {
		Ok(output) => {
			let stdout = String::from_utf8_lossy(&output.stdout).to_string();
			let stderr = String::from_utf8_lossy(&output.stderr).to_string();
			let result = if stderr.is_empty() { stdout } else { stderr };
			
			Ok(AST::String(result))
		},
		Err(e) => Err(format!("Command execution failed: {}", e))
	}
}

pub fn get_object() -> HashMap<String, AST> {
	let mut object = HashMap::new();

	object.insert(
		"exec".to_string(),
		AST::InternalFunction {
			name: "exec".to_string(),
			args: vec!["command".to_string()],
			call_fn: exec,
		}
	);

	object
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_exec_echo() {
		let args = vec![AST::String("echo hello".to_string())];
		let result = exec(args, &mut HashMap::new()).unwrap();
		match result {
			AST::String(value) => {
				assert!(value.contains("hello"));
			},
			_ => panic!("Expected string output")
		}
	}
}