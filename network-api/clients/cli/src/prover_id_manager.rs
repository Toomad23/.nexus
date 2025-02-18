use rand::RngCore;
use random_word::Lang;
use std::{fs, path::Path, path::PathBuf};

/// Gets an existing prover ID from the filesystem or generates a new one
pub fn get_or_generate_prover_id() -> String {
    let default_prover_id = generate_default_id();

    let home_path = match get_home_directory() {
        Ok(path) => path,
        Err(_) => return default_prover_id,
    };

    let nexus_dir = home_path.join(".nexus");
    let prover_id_path = nexus_dir.join("prover-id");

    if !nexus_dir.exists() {
        return handle_first_time_setup(&nexus_dir, &prover_id_path, &default_prover_id)
            .unwrap_or(default_prover_id);
    }

    match read_existing_prover_id(&prover_id_path) {
        Ok(id) => id,
        Err(e) => {
            eprintln!("Warning: Could not read prover-id file: {}", e);
            handle_read_error(e, &prover_id_path, &default_prover_id)
                .unwrap_or(default_prover_id)
        }
    }
}

fn generate_default_id() -> String {
    format!(
        "{}-{}-{}",
        random_word::gen(Lang::En),
        random_word::gen(Lang::En),
        rand::thread_rng().next_u32() % 100,
    )
}

fn get_home_directory() -> Result<PathBuf, std::io::Error> {
    home::home_dir().ok_or_else(|| {
        std::io::Error::new(
            std::io::ErrorKind::NotFound,
            "Home directory not found",
        )
    })
}

fn handle_first_time_setup(
    nexus_dir: &Path,
    prover_id_path: &Path,
    default_prover_id: &str,
) -> Result<String, std::io::Error> {
    fs::create_dir(nexus_dir)?;
    save_prover_id(prover_id_path, default_prover_id)?;
    Ok(default_prover_id.to_string())
}

fn read_existing_prover_id(prover_id_path: &Path) -> Result<String, std::io::Error> {
    let id = fs::read_to_string(prover_id_path)?;
    if id.trim().is_empty() {
        return Err(std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            "Prover ID file is empty",
        ));
    }
    Ok(id.trim().to_string())
}

fn save_prover_id(path: &Path, id: &str) -> Result<(), std::io::Error> {
    fs::write(path, id)?;
    println!("Successfully saved new prover-id to file: {}", id);
    Ok(())
}

fn handle_read_error(e: std::io::Error, path: &Path, default_id: &str) -> Result<String, std::io::Error> {
    match e.kind() {
        std::io::ErrorKind::NotFound => {
            save_prover_id(path, default_id)?;
            Ok(default_id.to_string())
        }
        _ => Err(e),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serial_test::serial;
    use std::env;
    use tempfile::TempDir;

    #[test]
    #[serial]
    fn test_new_user() {
        let temp_dir = TempDir::new().unwrap();
        env::set_var("HOME", temp_dir.path());

        let nexus_dir = temp_dir.path().join(".nexus");
        assert!(!nexus_dir.exists());

        let id1 = get_or_generate_prover_id();
        let id_path = nexus_dir.join("prover-id");
        assert!(id_path.exists());

        let saved_id = fs::read_to_string(&id_path).unwrap();
        assert_eq!(saved_id, id1);

        let id2 = get_or_generate_prover_id();
        assert_eq!(id2, id1);
    }
}
