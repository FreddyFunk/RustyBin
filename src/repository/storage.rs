use std::{path::Path, io::ErrorKind};

use uuid::Uuid;

pub async fn load_file(path: String, file_id: Uuid) -> std::io::Result<(String, Vec<u8>)> {
    let entries = std::fs::read_dir(path.clone() + file_id.to_string().as_str())?; 
    let files =
    entries.filter_map(|entry| {
    entry.ok().and_then(|e|
        e.path().file_name()
        .and_then(|n| n.to_str().map(|s| String::from(s)))
    )
    }).collect::<Vec<String>>();

    if files.len() != 1 {
        // TODO: Do a proper logging here
        return Err(ErrorKind::Other.into());
    }

    let file_name = files[0].clone();
    let path_list = vec![path, file_id.to_string(), file_name.clone()];
    let file_path = path_list.join("/");
    let file_content = std::fs::read(file_path)?;

    Ok((file_name, file_content))
}

pub async fn store_file(path: String, temp_file_path: &Path, file_name: String) -> std::io::Result<String> {
    let file_id = Uuid::new_v4().to_string();
    let file_path = path.clone() + "/" + file_id.as_str();
    
    match std::fs::create_dir(file_path.clone()) {
        Ok(_) => {},
        Err(e) => {
            std::fs::remove_dir(temp_file_path)?;
            return Err(e);
        }
    }; 

    match std::fs::rename(temp_file_path, file_path.clone() + "/" + file_name.as_str()) {
        Ok(_) => return Ok(file_id),
        Err(e) => {
            std::fs::remove_dir(file_path)?;
            std::fs::remove_dir(temp_file_path)?;
            return Err(e);
        }
    };    
}

pub async fn overwrite_file(path: String, temp_file_path: &Path, file_name: String, file_id: Uuid) -> std::io::Result<()> {
    let file_path = path.clone() + "/" + file_id.to_string().as_str();

    for entry in std::fs::read_dir(file_path.clone())? {
        std::fs::remove_file(entry?.path())?;
    }
    
    match std::fs::rename(temp_file_path, file_path.clone() + "/" + file_name.as_str()) {
        Ok(_) => return Ok(()),
        Err(e) => {
            std::fs::remove_dir(temp_file_path)?;
            return Err(e);
        }
    };    
}

pub async fn delete_file(path: String, file_id: Uuid) -> std::io::Result<()> {
    let file_path = path.clone() + "/" + file_id.to_string().as_str();
    return std::fs::remove_dir_all(file_path);  
}
