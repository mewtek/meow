use std::io::Write;
use std::fs::{File, self, read};
use std::path::Path;

use colored::Colorize;
use tar::Archive;

use crate::api;
use crate::config;
use crate::database;

/// Downloads a package from one of the mirrors in /etc/meow.d/mirrorlist
async fn download_pkg(pkg: &api::PackageDetails)
{
    let config = config::get_config();
    let arch = config::get_cpu_arch();
    let mirrors : Vec<String> = config::get_mirrors();
    let download_path = config.general.download_path;
    
    // TODO: Test mirror latency and determine the best one to download from
    let mirror : String = mirrors[1].to_owned();
    
    if !mirror.contains("$arch") && !mirror.contains("$repo")
    {
        println!("Mirror {} is invalid.\nMake sure all of the mirrors in /etc/meow.d/mirrorlist contain the keys {} and {}",
            mirror, "$arch".yellow().bold(), "$repo".yellow().bold());
        return;
    }


    if Path::new(&format!("{}{}", &download_path, pkg.filename)).exists()
    {
        fs::remove_file(format!("{}{}", &download_path, pkg.filename)).expect("Failed to remove file\nBad privileges?");
    }

    let download_url = format!("{}/{}", 
        mirror.replace("$arch", &arch).replace("$repo", &pkg.repo.to_string()),
        pkg.filename);

    // Download main archive
    let res = reqwest::get(&download_url).await.expect("WHOOPS!");
    let body = res.bytes().await.unwrap();
    let mut out = File::create(format!("{}{}", download_path, pkg.filename)).expect("Failed to create file!");
    out.write_all(&body).expect("Failed to write bytes!");

    // TODO: Download archive signature
    
}

/// Installs a pkg & its dependencies.
pub async fn install_pkg(pkg: &api::PackageDetails)
{
    let config = config::get_config();
    let pkg_path = format!("{}{}", config.general.download_path, &pkg.filename);
 
    for dependency in &pkg.depends
    {
        let x : api::PackageDetails = api::search_packages_exact(&dependency).await;

        if database::is_pkg_installed(&x).await {continue;}

        let depend_path = format!("{}{}", config.general.download_path, &x.filename);
        println!("{} Downloading {}..", "::".green().bold(), &dependency.to_string().blue());
        download_pkg(&x).await;
        install_files(&depend_path);
        database::add_pkg(&x).await;
    }
    
    println!("{} Downloading {}..", "::".green().bold(), &pkg.pkgname.to_string().blue());
    download_pkg(&pkg).await;
    install_files(&pkg_path);
    database::add_pkg(&pkg).await;
}

pub async fn upgrade_packages(pkgs: &Vec<api::PackageDetails>)
{
    let config = config::get_config();

    for pkg in pkgs 
    {
        let pkg_path = format!("{}{}", config.general.download_path, &pkg.filename);
        println!("{} Upgrading {} to version {}-{}..",
            "::".bold().green(), &pkg.pkgname.blue(), &pkg.pkgver, &pkg.pkgrel);

        database::remove_pkg(&pkg.pkgname).await;
        download_pkg(&pkg).await;
        install_files(&pkg_path);
        database::add_pkg(&pkg).await;
    }

    println!("{} {} {}", "Successfully upgraded".bold().green(), &pkgs.len().to_string().bold().green(), "packages!".bold().green());
}

fn install_files(path: &str)
{
    let path_compressed = &path;
    let path_decompressed = &path.replace(".tar.zst", ".tar");

    decompress_zstd(&path_compressed);
    expand_tar(path_decompressed);
}

/// Decompresses the .tar.zst file into a standard tar file for expansion into the main filesystem
fn decompress_zstd(path: &str)
{
    let decompressed_path = &path.replace(".tar.zst", ".tar");
    let mut compressed = read(&path).expect("Failed to read bytes of file.");
    let mut decompressed = File::create(decompressed_path.to_owned()).unwrap();
    let a = zstd::bulk::decompress(&mut compressed, 99999999 as usize).unwrap();
    let mut c : &[u8] = &a;

    println!("==> Decompressing {}", &path.red());
    decompressed.write_all(&mut c).expect("Failed to write bytes.");
}

fn expand_tar(path: &str)
{
    let tar = File::open(&path).unwrap();
    let mut archive = Archive::new(tar);
    println!("==> Extracting {}", path.red());
    archive.unpack("/").unwrap();
    fs::remove_file(&path).expect("Failed to remove old file\nBad previleges?");
}