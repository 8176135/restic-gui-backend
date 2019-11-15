use std::time::Duration;

pub fn get_last_time(name: &str) -> u64 {
	let filename = base64::encode_config(name, base64::Config::new(base64::CharacterSet::UrlSafe, false));
	std::fs::read_to_string(filename)
		.or_else(|err| if err.kind() == std::io::ErrorKind::NotFound { Ok("0".to_owned()) } else { Err(err) })
		.unwrap()
		.parse()
		.expect("File contents where unexpected")
}

fn save_new_last_time(name: &str, timestamp: u64) {
	let filename = base64::encode_config(name, base64::Config::new(base64::CharacterSet::UrlSafe, false));
	std::fs::write(filename, timestamp.to_string()).unwrap();
}

pub fn backup_scheduler() {
//	std::thread::sleep(Duration::from_secs(10 * 60));
	loop {
		let data = crate::load_stored_config();
		dbg!(&data);
		let since_the_epoch = std::time::SystemTime::now()
			.duration_since(std::time::UNIX_EPOCH)
			.expect("Time went backwards")
			.as_secs();
		for (name, config) in data {
			let last_time = get_last_time(&name);
			dbg!(last_time);
			if last_time + config.backup_interval.as_secs() <= since_the_epoch {
				let _out = config.config.restic_backup(&config.targets).expect("Failed to run backup");
				save_new_last_time(&name, since_the_epoch)
			}
		}
		std::thread::sleep(Duration::from_secs(60 * 60)) // Running once an hour
	}
}