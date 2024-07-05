use std::error::Error;
use std::path::PathBuf;
use std::io::{BufReader, Write};
use std::fs;

use serde::de::DeserializeOwned;
use serde::Serialize;
use uuid::Uuid;

use super::main::{Repository, UniqueEntity};

pub struct FileRepository {
    path: PathBuf
}

impl FileRepository {
    fn new(path: PathBuf) -> Self {
        Self {
            path,
        }
    }

    pub fn build(path: PathBuf) -> Result<Self, Box<dyn Error>> {
        let repo = Self::new(path);
        if !repo.path.as_path().try_exists()? {
            fs::create_dir_all(repo.path.as_path())?;
        }
        Ok(repo)
    }
}

impl<T: Serialize + DeserializeOwned + UniqueEntity> Repository<T> for FileRepository {
    fn create(&self, item: &T) -> Result<(), Box<dyn Error>> {
        let mut full_path = self.path.clone();
        full_path.push(format!("{}.save", item.uuid().to_string()));
        let str = serde_json::to_string(&item)?;
        let mut file = fs::File::create(&full_path)?;
        file.write(str.as_bytes())?;
        Ok(())
    }

    fn get_by_uuid(&self, uuid: &Uuid) -> Result<T, Box<dyn Error>> {
        let mut full_path = self.path.clone();
        full_path.push(format!("{}.save", uuid.to_string()));
        dbg!(&full_path);
        let file = fs::File::open(full_path)?;
        let buf = BufReader::new(file);
        let item: T = serde_json::from_reader(buf)?;
        Ok(item)
    }
}

#[cfg(test)]
mod tests {
    use uuid::Uuid;
    use serde::Deserialize;

    use super::*;

    #[derive(Serialize, Deserialize)]
    struct TestFileRepositoryItem {
        uuid: Uuid,
    }

    impl UniqueEntity for TestFileRepositoryItem {
        fn uuid<'a>(&'a self) -> &'a Uuid {
            &self.uuid
        }
    }

    #[test]
    fn build_create_dir_if_no_exists() -> Result<(), Box<dyn Error>> {
        let path = PathBuf::from("./tests");
        if path.as_path().try_exists()? {
            fs::remove_dir(&path)?;
        }

        let repo = FileRepository::build(path)?;
        if !repo.path.as_path().try_exists()? {
            Err("File not found".into())
        } else {
            fs::remove_dir(&repo.path)?;
            Ok(())
        }
    }

    #[test]
    fn create_err_if_file_already_exists() -> Result<(), Box<dyn Error>> {
        let dir_path = PathBuf::from("./tests");
        let repo = FileRepository::build(dir_path)?;
        fs::File::create("./tests/file")?;

        let item = TestFileRepositoryItem { uuid: Uuid::new_v4() };
        repo.create(&item)
    }
}
