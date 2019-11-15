use std::sync::{Mutex, MutexGuard};
use std::fs::File;
use std::ops::Deref;

pub struct ConfigFileGenerator(Mutex<()>);

impl ConfigFileGenerator {
	pub fn get_file_handle_read<'a>(&self) -> std::io::Result<ConfigFile> {
		let lock = self.0.lock().unwrap();
		let file = std::fs::File::open("restic_gui_backup.config")?;

		Ok(ConfigFile(file, lock))
	}

	pub fn get_file_handle_write<'a>(&self) -> std::io::Result<ConfigFile> {
		let lock = self.0.lock().unwrap();

		let file = std::fs::OpenOptions::new()
			.create(true)
			.write(true)
			.truncate(true)
			.open("restic_gui_backup.config")?;

		Ok(ConfigFile(file, lock))
	}

	pub fn new() -> ConfigFileGenerator {
		ConfigFileGenerator(Mutex::new(()))
	}
}

pub struct ConfigFile<'a>(File, MutexGuard<'a,()>);

impl<'a> Deref for ConfigFile<'a> {
	type Target = File;

	fn deref(&self) -> &File {
		&self.0
	}
}



