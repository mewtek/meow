use toml;
use serde_derive::{Serialize, Deserialize};
use std::fs::{File, create_dir};
use std::io::Write;
use std::path::Path;

use crate::config;
use crate::api;

#[derive(Serialize, Deserialize, Debug)]
pub struct InstalledPackage
{
    pub desc : PackageDesc,
    pub files : Vec<String>
}


#[derive(Serialize, Deserialize, Debug)]
pub struct PackageDesc
{
    pub pkgname : String,
    pub pkgbase : String,
    pub pkgver : String,
    pub pkgdesc : String,
    pub url : String,
    pub build_date : String,
    pub packager: String,
    pub size : i64,
    pub arch : String,
    pub licenses : Vec<String>,
    pub dependencies : Vec<String>,
    pub dependencies_optional : Vec<String>
}

pub async fn add_pkg_to_database(pkg: &api::PackageDetails)
{
    let config = config::get_config();
    let file_list = api::get_package_files(&pkg).await;
    let dir_path : String = format!("{}{}-{}-{}", config.general.db_path, &pkg.pkgname, &pkg.pkgver, &pkg.pkgrel);

    if Path::exists(&Path::new(&dir_path))
    {
        // TODO:
        return;
    }

    let pkgdesc = PackageDesc {
        pkgname: pkg.pkgname.to_owned(),
        pkgbase: pkg.pkgbase.to_owned(),
        pkgver: format!("{}-{}", pkg.pkgver, pkg.pkgrel).into(),
        pkgdesc: pkg.pkgdesc.to_owned(),
        url: pkg.url.to_owned(),
        build_date: pkg.build_date.to_owned(),
        packager: pkg.packager.to_owned(),
        size: pkg.installed_size,
        arch: pkg.arch.to_owned(),
        licenses: pkg.licenses.to_owned(),
        dependencies: pkg.depends.to_owned(),
        dependencies_optional: pkg.optdepends.to_owned()
   };

   let installed_pkg = InstalledPackage {desc: pkgdesc, files: file_list};
   let toml = toml::to_string(&installed_pkg).unwrap();

   create_dir(&dir_path).unwrap();
   let mut file = File::create(format!("{}/{}", &dir_path, "PKGDESC")).expect("Failed to create PKGDESC file\nBad permissions?");
   file.write_all(&toml.as_bytes()).expect("Failed to write to database\nBad permissions?");
}