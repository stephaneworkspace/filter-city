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
 * This program is free software; you can redistribute it and/or modify
 * it under the terms of the GNU General Public License version 3
 * as published by the Free Software Foundation.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program; if not, see <http://www.gnu.org/licenses/>.
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

#[derive(Serialize, Deserialize, Debug)]
pub struct City {
    country: String,
    name: String,
    lat: String,
    lng: String,
}

pub fn filter_city(filter: &str) -> Vec<City> {
    let filter_upper_decode = unidecode(filter).to_ascii_uppercase();
    let mut compare_string;
    let mut s = String::new();
    const PATH: &str = "assets/citys.json";
    let mut file_path: std::path::PathBuf = std::path::PathBuf::new();
    file_path.push(std::env::current_dir().unwrap().as_path());
    file_path.push(PATH);
    println!("{:?}", file_path.as_path());
    File::open(file_path.as_path())
        .unwrap()
        .read_to_string(&mut s)
        .unwrap();
    let _deserialized: Vec<City> = serde_json::from_str(&s).unwrap();
    let mut city: Vec<City> = Vec::new();
    for x in &_deserialized {
        if filter.len() > 0 {
            compare_string =
                unidecode(x.name.clone().as_str()).to_ascii_uppercase();
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