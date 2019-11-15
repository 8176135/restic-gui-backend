mod config_store;
mod automated_backups;

use serde::{Deserialize, Serialize};

use std::path::{Path, PathBuf};
use config_store::ConfigFileGenerator;
use lazy_static::lazy_static;
use restic_interfacer::{ResticConfig, ResticStorageConfig, BackupTarget, ForgetRate};
use std::collections::HashMap;
use chrono::TimeZone;
use std::time::Duration;

lazy_static! {
    static ref CONFIG_FILE_GENERATOR: ConfigFileGenerator = ConfigFileGenerator::new();
}

#[derive(Debug, Deserialize)]
#[serde(tag = "type")]
enum GuiCommand {
	PathLookup { path: String, target: BackupTarget },
	RetrieveAllConfigs,
	GetNewTemplate,
	SaveAllConfigs { config: Vec<FlatterConfig> },
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
pub struct ResticBackupConfiguration {
	config: ResticConfig,
	targets: BackupTarget,
	forget_rate: ForgetRate,
	backup_interval: std::time::Duration,
}

//impl ResticBackupConfiguration {
//    pub fn new() -> ResticBackupConfiguration {
//        ResticBackupConfiguration {
//
//        }
//    }
//}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FlatterConfig {
	backup_interval: u64,
	repo_password: String,
	repo_path: String,
	forget_rate: ForgetRate,
	targets: BackupTarget,
	last_backup_time: String,
	name: String,
}

impl Default for FlatterConfig {
	fn default() -> FlatterConfig {
		FlatterConfig {
			repo_path: serde_json::to_string(&ResticStorageConfig::Local(PathBuf::new().join("./").canonicalize().unwrap())).unwrap(),
			name: chrono::Local::now().timestamp().to_string(),
			targets: BackupTarget::default(),
			last_backup_time: "Never".to_string(),
			backup_interval: 22,
			repo_password: String::new(),
			forget_rate: ForgetRate::default(),
		}
	}
}

pub fn load_stored_config() -> HashMap<String, ResticBackupConfiguration> {
	let file = CONFIG_FILE_GENERATOR.get_file_handle_read();

	match file {
		Ok(file) => serde_json::from_reader(&*file).unwrap(),
		Err(ref err) if err.kind() == std::io::ErrorKind::NotFound => HashMap::new(),
		Err(err) => panic!(err)
	}
}

fn save_configs(to_save: &HashMap<String, ResticBackupConfiguration>) {
	let file = CONFIG_FILE_GENERATOR.get_file_handle_write()
		.expect("Failed to open/create config file");

	serde_json::to_writer_pretty(&*file, to_save).expect("Failed to write")
}

fn to_flatter_config(og: HashMap<String, ResticBackupConfiguration>) -> Vec<FlatterConfig> {
	og.into_iter().map(|(name, data)| {
		let last_bk_time = automated_backups::get_last_time(&name);
		FlatterConfig {
			backup_interval: data.backup_interval.as_secs() / 3600,
			repo_password: data.config.repo_password,
			repo_path: serde_json::to_string(&data.config.repo_path).unwrap(),
			forget_rate: data.forget_rate,
			targets: data.targets,
			last_backup_time: if last_bk_time == 0 { "Never".to_owned() } else { chrono::Local.timestamp(last_bk_time as i64, 0).format("%Y/%m/%d - %H:%M:%S").to_string() },
			name,
		}
	}).collect()
}

fn from_flatter_config(og: Vec<FlatterConfig>) -> HashMap<String, ResticBackupConfiguration> {
	og.into_iter().map(|c| {
		(c.name, ResticBackupConfiguration {
			backup_interval: Duration::from_secs(c.backup_interval * 60 * 60),
			config: ResticConfig::new(c.repo_password, serde_json::from_str(&c.repo_path).expect("Not valid path json")), // TODO: Propergate
			forget_rate: c.forget_rate,
			targets: c.targets,
		})
	}).collect()
}

fn main() {

	color_backtrace::install();

	let mut loaded_configs: HashMap<String, ResticBackupConfiguration> = load_stored_config();

	loaded_configs.insert("Config 99".to_owned(), ResticBackupConfiguration {
		config: ResticConfig::new("1234".to_owned(), ResticStorageConfig::Local("/mnt/d/FDrive/Documents/RustProjects/restic-interfacer/sample_repo".into())),
		targets: BackupTarget::new_from_string(&["./src", "./frontend"], vec!["node_modules".to_owned()], Vec::new()).unwrap(),
		forget_rate: ForgetRate {
			keep_weekly: 5,
			keep_daily: 7,
			..Default::default()
		},
		backup_interval: std::time::Duration::from_secs(60 * 60 * 20),
	});

	save_configs(&loaded_configs);

//	std::thread::spawn(automated_backups::backup_scheduler);

	ws::listen("127.0.0.1:3012", |out| {
		move |msg: ws::Message| {
			let txt = msg.into_text().unwrap();
			let backup_target = restic_interfacer::BackupTarget::new_from_string(
				&vec![".".to_owned()],
				vec!["static".to_owned()],
				Vec::new(),
			).unwrap();

			let cmd: GuiCommand = serde_json::from_str(&txt).unwrap();

			match cmd {
				GuiCommand::PathLookup { path, target } => {
					let dir: Vec<DirEntryStripped> = get_dir_list(path, &target);
					out.send(ws::Message::Text(serde_json::to_string(&dir).unwrap()))
				}
				GuiCommand::RetrieveAllConfigs => {
					out.send(ws::Message::Text(serde_json::to_string(&to_flatter_config(load_stored_config())).unwrap()))
				}
				GuiCommand::SaveAllConfigs { config} => {
					save_configs(&from_flatter_config(config));
					Ok(())
				}
				GuiCommand::GetNewTemplate => {
					out.send(ws::Message::Text(serde_json::to_string(&FlatterConfig::default()).unwrap()))
				}
			}
		}
	})
		.unwrap();
}
