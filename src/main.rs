use clap::Parser;
use figment::{providers::Env, Figment};
use serde::Deserialize;
use std::{
	error::Error,
	path::{Path, PathBuf}, fs,
};

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
	#[clap(short, long, value_parser)]
	name: String,

	#[clap(value_parser, value_name = "FILE")]
	file: PathBuf,
}

#[derive(Debug, Deserialize)]
struct Settings {
	student_id: usize,
	first: String,
	last: String,
}

fn main() {
	let args = Args::parse();
	let settings = get_settings().unwrap(); // get the settings

	rename_file(settings, args).unwrap();
}

fn get_settings() -> Result<Settings, Box<dyn Error>> {
	let figment = Figment::from(Env::prefixed("NAME_RS_")); // get all env vars starting with
																												// NAME_RS_
	Ok(figment.extract()?) // Return the object
}

fn get_course(file: &Path) -> Result<String, Box<dyn Error>> {
	let course = file
		.canonicalize()?.to_str().unwrap().split('/').rev().nth(1).unwrap().split(' ').next().unwrap().to_string();
	Ok(course)
}

fn get_file_extension(file: &Path) -> Result<String, Box<dyn Error>> {
	let extension = file.extension().unwrap().to_str().unwrap().to_string();
	Ok(extension)
}

fn rename_file(settings: Settings, args: Args) -> Result<(), Box<dyn Error>> {
	let course = get_course(&args.file)?;
	let file_extension = get_file_extension(&args.file)?;
	let new_name = format!(
		"{}_{}_{}_{}_{}.{}",
		settings.student_id, settings.last, settings.first, course, args.name.replace(" ", "_"), file_extension
	);
	rename(args.file.to_str().unwrap(), new_name.as_str());
	Ok(())
}

fn rename(start : &str, end : &str) {
	println!("Renaming {} to {}", start, end);
	if fs::rename(start, end).is_err() {
		rename(start, end);
	} else {
	println!("Done Rename");
	
}}
