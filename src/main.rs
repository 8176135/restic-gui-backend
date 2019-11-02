use serde::{Deserialize, Serialize};

use std::path::Path;
use std::sync::Mutex;
use std::io::{Write, Read};
use lazy_static::lazy_static;
use restic_interfacer::{ResticConfig, ResticStorageConfig, BackupTarget};
use std::collections::HashMap;

lazy_static! {
    static ref CONFIG_FILE: Mutex<std::fs::File> = Mutex::new(std::fs::OpenOptions::new()
            .create(true)
            .read(true)
            .write(true)
            .open("restic_gui_backup.config")
            .expect("Failed to open/create config file"));
}

#[derive(Debug, Deserialize)]
#[serde(tag = "type")]
enum GuiCommand {
	PathLookup { path: String },
	RetrieveAllConfigs,
}

#[derive(Serialize)]
struct DirEntryStripped {
	name: String,
	extension: String,
	is_dir: bool,
	included_in_backup: bool,
	has_backup_inside: bool,
}

impl From<std::fs::DirEntry> for DirEntryStripped {
	fn from(other: std::fs::DirEntry) -> Self {
		Self {
			name: other.file_name().to_string_lossy().to_string(),
			extension: other.path().extension().map(|c| c.to_string_lossy().to_string()).unwrap_or(String::new()),
			is_dir: other.file_type().unwrap().is_dir(),
			included_in_backup: true,
			has_backup_inside: true,
		}
	}
}

fn get_dir_list<P: AsRef<Path>>(
	path: P,
	backup_target: &restic_interfacer::BackupTarget,
) -> Vec<DirEntryStripped> {
	use restic_interfacer::BackupFileSelectionType;
	let target_type = backup_target.check_path_is_in_backup(&path);

	std::fs::read_dir(path.as_ref())
		.unwrap()
		.filter_map(Result::ok)
		.map(DirEntryStripped::from)
		.map(|mut c| {
			if target_type == BackupFileSelectionType::Excluded {
				c.included_in_backup = false;
				c.has_backup_inside = false;
			} else {
				match backup_target.check_path_is_in_backup(&path.as_ref().join(&c.name)) {
					BackupFileSelectionType::Excluded | BackupFileSelectionType::Irreverent => {
						c.included_in_backup = false;
						c.has_backup_inside = false;
					}
					BackupFileSelectionType::Included => {
						c.included_in_backup = true;
						c.has_backup_inside = false;
					}
					BackupFileSelectionType::Contains => {
						c.included_in_backup = false;
						c.has_backup_inside = true;
					}
				}
			}
			c
		})
		.collect()
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct ResticBackupConfiguration {
	config: restic_interfacer::ResticConfig,
	targets: Vec<restic_interfacer::BackupTarget>,
}

//impl ResticBackupConfiguration {
//    pub fn new() -> ResticBackupConfiguration {
//        ResticBackupConfiguration {
//
//        }
//    }
//}

fn load_stored_config() -> HashMap<String, ResticBackupConfiguration> {
	let file = std::fs::File::open("restic_gui_backup.config");

	match file {
		Ok(file) => serde_json::from_reader(file).unwrap(),
		Err(ref err) if err.kind() == std::io::ErrorKind::NotFound => HashMap::new(),
		Err(err) => panic!(err)
	}
}

fn save_configs(to_save: &HashMap<String, ResticBackupConfiguration>) {
	let file = std::fs::OpenOptions::new()
		.create(true)
		.write(true)
		.open("restic_gui_backup.config")
		.expect("Failed to open/create config file");

	serde_json::to_writer(file, to_save).expect("Failed to write")
}

fn main() {
	let mut loaded_configs: HashMap<String, ResticBackupConfiguration> = load_stored_config();

	ws::listen("127.0.0.1:3012", |out| {
		move |msg: ws::Message| {
			let txt = msg.into_text().unwrap();
			let backup_target = restic_interfacer::BackupTarget::new_from_string(
				&vec![".".to_owned()],
				vec!["static".to_owned()],
				Vec::new(),
			)
				.unwrap();

			let cmd: GuiCommand = serde_json::from_str(&txt).unwrap();

			match cmd {
				GuiCommand::PathLookup { path } => {
					let dir: Vec<DirEntryStripped> = get_dir_list(path, &backup_target);

					out.send(ws::Message::Text(serde_json::to_string(&dir).unwrap()))
				}
				GuiCommand::RetrieveAllConfigs => {
					let loaded_configs: HashMap<String, ResticBackupConfiguration> = load_stored_config();
					out.send(ws::Message::Text(serde_json::to_string(&loaded_configs).unwrap()))
				}
			}
		}
	})
		.unwrap();
}
