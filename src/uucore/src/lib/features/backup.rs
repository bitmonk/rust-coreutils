// This file is part of the uutils coreutils package.
//
// (c) Sylvestre Ledru <sylvestre@debian.org>
// (c) Justin Ryan <justizin@gmail.com>
//
// For the full copyright and license information, please view the LICENSE
// file that was distributed with this source code.

use std::env;
use std::path::{PathBuf};

use clap::{ArgMatches};

pub static OPT_BACKUP: &str = "backup";
pub static OPT_BACKUP_NO_ARG: &str = "b";
pub static OPT_SUFFIX: &str = "suffix";

#[derive(Clone, Copy, Eq, PartialEq)]
pub enum BackupMode {
    NoBackup,
    SimpleBackup,
    NumberedBackup,
    ExistingBackup,
}

pub fn determine_backup_mode(matches: &ArgMatches) -> BackupMode {
    if matches.is_present(OPT_BACKUP_NO_ARG) {
        BackupMode::SimpleBackup
    } else if matches.is_present(OPT_BACKUP) {
        match matches.value_of(OPT_BACKUP).map(String::from) {
            None => BackupMode::SimpleBackup,
            Some(mode) => match &mode[..] {
                "simple" | "never" => BackupMode::SimpleBackup,
                "numbered" | "t" => BackupMode::NumberedBackup,
                "existing" | "nil" => BackupMode::ExistingBackup,
                "none" | "off" => BackupMode::NoBackup,
                _ => panic!(), // cannot happen as it is managed by clap
            },
        }
    } else {
        BackupMode::NoBackup
    }
}

pub fn determine_backup_suffix(backup_mode: BackupMode, matches: &ArgMatches) -> String {
    if matches.is_present(OPT_SUFFIX) {
        matches.value_of(OPT_SUFFIX).map(String::from).unwrap()
    } else if let (Ok(s), BackupMode::SimpleBackup) =
        (env::var("SIMPLE_BACKUP_SUFFIX"), backup_mode)
    {
        s
    } else {
        "~".to_owned()
    }
}

pub fn simple_backup_path(path: &PathBuf, suffix: &str) -> PathBuf {
    let mut p = path.to_string_lossy().into_owned();
    p.push_str(suffix);
    PathBuf::from(p)
}

pub fn numbered_backup_path(path: &PathBuf) -> PathBuf {
    (1_u64..)
        .map(|i| path.with_extension(format!("~{}~", i)))
        .find(|p| !p.exists())
        .expect("cannot create backup")
}

pub fn existing_backup_path(path: &PathBuf, suffix: &str) -> PathBuf {
    let test_path = path.with_extension("~1~");
    if test_path.exists() {
        numbered_backup_path(path)
    } else {
        simple_backup_path(path, suffix)
    }
}