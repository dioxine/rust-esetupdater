use super::structs::Credentials;
use log::{error, info};
use reqwest::Client;
use reqwest::Error;
use std::fs::{File, create_dir_all};
use std::io::Write;
use std::path::Path;

use super::helpers::{local_path_fixer, remote_path_fixer};
use super::structs::Nups;

pub fn download_update_ver_file(
    local_path: &str,
    update_ver_file_with_path: &str,
    creds: &Credentials,
) {
    let _result = match download_file(local_path, update_ver_file_with_path, creds) {
        Ok(val) => {
            info!("File {val} downloaded successfully");
        }
        Err(err) => {
            error!("Error while downloading file: {}", err);
            std::process::exit(1);
        }
    };
}

pub fn download_nup_files(nups_paths: Vec<Nups>, root_dir: &str, creds: &Credentials) {
    let mut counter: u32 = 0;
    for nup_path in nups_paths {
        info!(
            "-------------------------------------------------------------------------------------------------------------"
        );
        nup_path.description.split("\n").for_each(|line| info!("{}", line));
        info!("nup_path: {}", remote_path_fixer(&creds, &nup_path.path));
        let _result = match download_file(&root_dir, &nup_path.path, &creds) {
            Ok(val) => {
                counter += 1;
                info!("File {val} downloaded successfully");
            }
            Err(err) => {
                error!("Error while downloading file: {}", err);
                std::process::exit(1);
            }
        };
    }
    info!("Total count {} of NUP-files downloaded.", counter);
}

pub fn download_file(
    local_path: &str,
    filename_with_path: &str,
    creds: &Credentials,
) -> Result<String, Error> {
    let url = remote_path_fixer(creds, filename_with_path);

    #[tokio::main(flavor = "current_thread")] //async downloading function using tokio (single thead) and reqwest
    async fn download_async(
        url: &str,
        local_path: &str,
        filename_with_path: &str,
        user_agent: &str,
        user: &str,
        password: &str,
    ) -> Result<String, Error> {
        let client = Client::builder().user_agent(user_agent).build()?;

        let response = client
            .get(url)
            .basic_auth(user, Some(password))
            .send()
            .await?;

        if response.status().is_success() {
            let path = local_path_fixer(local_path, filename_with_path);

            //coverting path to Path to get parent
            let path = Path::new(&path);
            let parent = path.parent().unwrap();

            let _ = create_dir_all(parent);
            let mut file = File::create(path).unwrap();
            let content = response.bytes().await?;
            match file.write_all(&content) {
                Ok(val) => val,
                Err(err) => panic!("Error while writing file: {}", err),
            }
            Ok(path.to_str().unwrap().to_string())
        } else {
            Err(response.error_for_status().unwrap_err())
        }
    }

    download_async(
        &url,
        &local_path,
        &filename_with_path,
        &creds.user_agent,
        &creds.user,
        &creds.password,
    )
}
