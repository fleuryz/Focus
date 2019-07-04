extern crate time;
extern crate nix;
extern crate rand;
extern crate find_folder;
extern crate os_type;
#[macro_use]
extern crate conrod;

pub mod cenario;
pub mod dados;
pub mod par;
pub mod respostaSN;
pub mod sessao;
pub mod teste;
pub mod variavel;
pub mod gui;
pub mod support;

/*
extern crate time;
extern crate rand;
extern crate ordered_float;
extern crate nix;

use std::io;
use std::process::Command;
use std::str::FromStr;
use std::string::ParseError;
use std::thread;
use std::sync::mpsc;
use std::fs::File;

use std::io::BufReader;
use rand::Rng;
use std::cmp::Ordering;
use nix::sys::signal::Signal;
use nix::unistd::Pid;
use std::fs;

*/