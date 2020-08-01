/******************************************************************************
 * This program allows to search a city in ASCII all over the world in a json
 * file.
 *
 * Initalliy I have done a script with Python but thas was very slow.
 *
 * By Stéphane Bressani
 *  ____  _             _
 * / ___|| |_ ___ _ __ | |__   __ _ _ __   ___
 * \___ \| __/ _ \ '_ \| '_ \ / _` | '_ \ / _ \
 *  ___) | ||  __/ |_) | | | | (_| | | | |  __/
 * |____/ \__\___| .__/|_| |_|\__,_|_| |_|\___|
 *               | |stephane-bressani.ch
 *               |_|github.com/stephaneworkspace
 *
 *****************************************************************************/
extern crate serde;
extern crate serde_derive;
extern crate serde_json;
extern crate unidecode;

use serde::Deserialize;
use serde::Serialize;
use std::fs::File;
use std::io::Read;
use unidecode::unidecode;

use std::io::{self, Write};
use std::process;
use std::str::FromStr;
use std::sync::mpsc::{channel, Sender};
use std::thread;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct City {
    pub country: String,
    pub name: String,
    pub lat: String,
    pub lng: String,
}

#[derive(Debug, Clone)]
pub struct Citys {
    pub citys: Vec<City>,
}

pub trait Fd {
    fn filter(&self, name: String) -> Vec<City>;
    fn filter_multithread(&self, name: String) -> Vec<City>;
}

impl Citys {
    pub fn new(path: &str) -> Citys {
        let mut s = String::new();
        let mut file_path: std::path::PathBuf = std::path::PathBuf::new();
        file_path.push(std::env::current_dir().unwrap().as_path());
        file_path.push(path);
        // println!("{:?}", file_path.as_path());
        File::open(file_path.as_path())
            .unwrap()
            .read_to_string(&mut s)
            .unwrap();
        Citys {
            citys: serde_json::from_str(&s).unwrap(),
        }
    }
}

impl Fd for Citys {
    fn filter(&self, name: String) -> Vec<City> {
        let filter_upper_decode = unidecode(name.as_str()).to_ascii_uppercase();
        let mut city: Vec<City> = Vec::new();
        for x in self.citys.clone() {
            if name.len() > 0 {
                let compare_string =
                    unidecode(x.name.as_str()).to_ascii_uppercase();
                if compare_string.contains(filter_upper_decode.as_str()) {
                    city.push(City {
                        country: x.country.clone(),
                        name: x.name.clone(),
                        lat: x.lat.clone(),
                        lng: x.lng.clone(),
                    });
                }
            }
        }
        city
    }

    fn filter_multithread(&self, name: String) -> Vec<City> {
        let mut citys: Vec<City> = vec![];
        if name.len() > 1 {
            let num_threads = 4;
            let (tx, rx) = channel();
            for i in 0..num_threads {
                let tx = tx.clone();
                let citys_static: Vec<City> = self.citys.clone();
                let filter_upper_decode =
                    unidecode(&name.as_str()).to_ascii_uppercase();
                let jj: usize = i * ((citys_static.len()) / num_threads);
                let max_jj: usize =
                    (i + 1) * (citys_static.len() / num_threads);
                thread::spawn(move || loop {
                    let mut j: usize = jj;
                    let x = citys_static[j].clone();
                    let compare_string =
                        unidecode(x.name.as_str()).to_ascii_uppercase();
                    if compare_string.contains(filter_upper_decode.as_str()) {
                        // print!(".");
                        // io::stdout().flush().unwrap();
                        /*println!(
                            "name: {} {}",
                            x.name.clone(),
                            x.country.clone()
                        );*/
                        tx.send(City {
                            country: x.country.clone(),
                            name: x.name.clone(),
                            lat: x.lat.clone(),
                            lng: x.lng.clone(),
                        })
                        .unwrap();
                    }
                    j += 1;
                    if j >= citys_static.len() || j >= max_jj {
                        break;
                    }
                });
            }
            drop(tx);
            for p in rx {
                citys.push(p.clone());
            }
        }
        citys
    }
}

pub fn filter_city(name: String) -> Vec<City> {
    const PATH: &str = "assets/citys.json";
    let fd = Citys::new(PATH);
    //fd.filter(name)
    fd.filter_multithread(name)
}
