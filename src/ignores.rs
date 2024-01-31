use std::{collections::HashSet, path::PathBuf};

// TODO: Need to check current location, and navigate upwards until you reach either .git or
// .blinkignore, whichever comes first
pub(crate) fn load_ignores(path: &PathBuf) -> HashSet<PathBuf>
{
	return HashSet::new();
}
