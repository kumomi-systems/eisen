// Eisen Operating System
// Copyright (C) 2025  Kumomi Systems
// 
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
// 
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
// 
// You should have received a copy of the GNU General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

extern crate alloc;

use serde::Deserialize;
use spin::once::Once;
use uefi::{CStr16, cstr16};
use uefi::proto::media::file::{File, FileInfo, FileMode};

const CONFIG_FILE_PATH: &CStr16 = cstr16!("eisen\\bootconfig.toml");

pub static CONFIG: Once<Config> = Once::new();

#[derive(Deserialize)]
pub struct Config {
  pub cmdline:  alloc::string::String,
  pub loglevel: usize
}

const DEFAULT_CONFIG: Config = Config {
  cmdline:  alloc::string::String::new(),
  loglevel: crate::logger::LOGLEVEL_NORMAL
};

pub fn load_config<'a>() -> &'a Config {
  let mut config_file;
  match crate::fs::read_file(CONFIG_FILE_PATH, FileMode::CreateReadWrite) {
    Some(some)  => { config_file = some; }
    None        => { panic!("Failed to open/create config file") }
  }

  let config_file_info: alloc::boxed::Box<FileInfo>;
  match config_file.get_boxed_info() {
    Ok(ok)      => { config_file_info = ok; }
    Err(err)    => { panic!("Failed to determine config file size: {}", err) }
  }

  let config_file_size = config_file_info.file_size() as usize;
  let mut config_file_buffer = alloc::vec![0; config_file_size];
  match config_file.read(&mut config_file_buffer) {
    Ok(ok)      => {
      if ok != config_file_size {
        panic!("Failed to read config file")
      }
    }
    Err(err)    => { panic!("Failed to read config file: {}", err) }
  }

  CONFIG.call_once(|| {
    toml::from_slice(&config_file_buffer).unwrap_or(DEFAULT_CONFIG)
  })
}

