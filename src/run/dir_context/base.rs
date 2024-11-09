use crate::run::DevaiDir;
use crate::support::files::current_dir;
use crate::Result;
use simple_fs::SPath;

#[derive(Debug, Clone)]
pub struct DirContext {
	/// Absolute path of the current_dir (pwd)
	/// (except for test, which can be mocked to another dir)
	current_dir: SPath,

	devai_dir: DevaiDir,

	// Absolute path of the devai
	devai_parent_dir: SPath,
}

/// Constructor/Loader
impl DirContext {
	pub fn new(devai_dir: DevaiDir) -> Result<Self> {
		let current_dir = current_dir()?;
		Self::from_devai_dir_and_current_dir(devai_dir, current_dir)
	}

	/// Private to create a new DirContext
	/// Note: Only the test function will provide a mock current_dir
	fn from_devai_dir_and_current_dir(devai_dir: DevaiDir, current_dir: SPath) -> Result<Self> {
		let devai_parent_dir = devai_dir.parent_dir().canonicalize()?;
		let current_dir = current_dir.canonicalize()?;
		Ok(Self {
			current_dir,
			devai_dir,
			devai_parent_dir,
		})
	}

	/// Here is a test function that create a new DirContext with a Mock current dir
	#[cfg(test)]
	pub fn from_parent_dir_and_current_dir_for_test(
		parent_dir: impl AsRef<std::path::Path>,
		mock_current_dir: SPath,
	) -> Result<Self> {
		Self::from_devai_dir_and_current_dir(DevaiDir::from_parent_dir(parent_dir)?, mock_current_dir)
	}
}

/// Property Getters
impl DirContext {
	pub fn current_dir(&self) -> &SPath {
		&self.current_dir
	}

	pub fn devai_dir(&self) -> &DevaiDir {
		&self.devai_dir
	}

	pub fn devai_parent_dir(&self) -> &SPath {
		&self.devai_parent_dir
	}
}
