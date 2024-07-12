use std::error::Error;
use std::marker::PhantomData;
use std::path::{Path, PathBuf};
use std::io::{self, BufReader, Write};
use std::fs;

use serde::de::DeserializeOwned;
use serde::Serialize;
use uuid::Uuid;

use super::main::{Repository, RepositoryError, UniqueEntity};

pub struct FileRepository<T> {
    path: PathBuf,
    phantom: PhantomData<T>
}

impl<T> FileRepository<T> {
    fn new(path: PathBuf) -> Self {
        Self {
            path,
            phantom: PhantomData
        }
    }

    pub fn build(path: PathBuf) -> Result<Self, RepositoryError> {
        let repo = Self::new(path);
        if !repo.path.as_path().try_exists()? {
            fs::create_dir_all(repo.path.as_path())?;
        }
        Ok(repo)
    }

    pub fn full_path<P: AsRef<Path>>(&self, path: P) -> PathBuf {
        self.path.join(path)
    }
}

impl<T: Serialize + DeserializeOwned + UniqueEntity> Repository<T> for FileRepository<T> {
    fn list(&self) -> Result<Vec<Uuid>, RepositoryError> {
        let list = fs::read_dir(&self.path)?;
        let mut uuids: Vec<Uuid> = vec![];
        for file in list {
            let name = file?.file_name().into_string().unwrap();
            let splitted_name: Vec<&str> = name.split('.').collect();
            let uuid = Uuid::parse_str(splitted_name[0])?;
            uuids.push(uuid)
            
        }
        Ok(uuids)
    }

    fn create(&self, item: &T) -> Result<(), RepositoryError> {
        let path = self.full_path(format!("{}.save", item.uuid().to_string()));
        let str = serde_json::to_string(&item)?;
        let mut file = fs::File::create(&path)?;
        file.write(str.as_bytes())?;
        Ok(())
    }

    fn get_by_uuid(&self, uuid: &Uuid) -> Result<T, RepositoryError> {
        let path = self.full_path(format!("{}.save", uuid.to_string()));
        let file = fs::File::open(path)?;
        let buf = BufReader::new(file);
        let item: T = serde_json::from_reader(buf)?;
        Ok(item)
    }

    fn update(&self, uuid: &Uuid, item: &T) -> Result<(), RepositoryError> {
        let path = self.full_path(format!("{}.save", uuid.to_string()));
        let str = serde_json::to_string(&item)?;
        fs::write(path, str)?;
        Ok(())
    }

    fn delete(&self, uuid: &Uuid) -> Result<(), RepositoryError> {
        let path = self.full_path(format!("{}.save", uuid.to_string()));
        fs::remove_file(path)?;
        Ok(())
    }
}

impl From<serde_json::Error> for RepositoryError {
    fn from(value: serde_json::Error) -> Self {
        Self::new(format!("FileRepository serde_json::Error:\n{value}"))
    }
}

impl From<io::Error> for RepositoryError {
    fn from(value: io::Error) -> Self {
        Self::new(format!("FileRepository io::Error:\n{value}"))
    }
}

impl From<uuid::Error> for RepositoryError {
    fn from(value: uuid::Error) -> Self {
        Self::new(format!("FileRepository std::Error:\n{value}"))
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

        let repo: FileRepository<TestFileRepositoryItem> = FileRepository::build(path)?;
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
        repo.create(&item)?;
        Ok(())
    }
}
