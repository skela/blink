use std::collections::HashSet;
use std::fs::{self, File};
use std::io::{self, BufRead};
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

pub(crate) fn load_ignores(path: &PathBuf) -> HashSet<PathBuf>
{
	let mut files: HashSet<PathBuf> = HashSet::new();
	if let Some(blinkignore_path) = find_blinkignore(path)
	{
		if let Ok(file) = File::open(&blinkignore_path)
		{
			let reader = io::BufReader::new(file);
			let blinkignore_dir = blinkignore_path.parent().unwrap_or(Path::new(""));

			for res in reader.lines()
			{
				if let Ok(line) = res
				{
					let ignored_path = blinkignore_dir.join(line.trim_end());
					if ignored_path.is_dir()
					{
						for entry in WalkDir::new(ignored_path)
						{
							let entry = entry.unwrap();
							if entry.file_type().is_file()
							{
								files.insert(entry.path().to_path_buf());
							}
						}
					}
					else
					{
						files.insert(ignored_path);
					}
				}
			}
		}
	}
	return files;
}

fn find_blinkignore(starting_path: &Path) -> Option<PathBuf>
{
	if let Ok(start) = fs::canonicalize(starting_path)
	{
		let mut current_path = start.to_path_buf();

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
				return None;
			}

			match current_path.parent()
			{
				Some(parent) => current_path = parent.to_path_buf(),
				None => return None,
			}
		}
	}

	None
}
