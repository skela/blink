use std::fs;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::{collections::HashSet, path::PathBuf};

pub(crate) fn load_ignores(path: &PathBuf) -> HashSet<PathBuf>
{
	let mut files: HashSet<PathBuf> = HashSet::new();
	if let Some(blinkignore) = find_blinkignore(path)
	{
		if let Ok(file) = File::open(&blinkignore)
		{
			let reader = io::BufReader::new(file);

			for res in reader.lines()
			{
				if let Ok(line) = res
				{
					files.insert(PathBuf::from(line));
				}
			}
		}
	}
	return files;
}

fn find_blinkignore(starting_path: &Path) -> Option<PathBuf>
{
	let mut current_path = starting_path.to_path_buf();

	while current_path.as_os_str().len() > 0
	{
		let blinkignore_path = current_path.join(".blinkignore");
		if fs::metadata(&blinkignore_path).is_ok()
		{
			return Some(blinkignore_path);
		}

		let git_path = current_path.join(".git");
		if git_path.is_dir()
		{
			println!("Found .git directory at: {:?}", git_path);
			return None;
		}

		match current_path.parent()
		{
			Some(parent) => current_path = parent.to_path_buf(),
			None => return None, // If no parent, we're at the root
		}
	}

	None
}
