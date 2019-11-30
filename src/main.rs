/******************************************************************************
 * Filter of city analyse without Orm/Sql/NoSql
 *
 * Initalliy i have done a script for filter a json file 20 mo in django
 * without using a database and the computing of information is very slowwwwww
 *
 * So... this is my hello world with rust !!!
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
 * it under the terms of the GNU General Public License version 2
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
use serde::Deserialize;
use std::fs::File;
use std::io::Read;

#[derive(Deserialize, Debug)]
struct City {
    country: String,
    name: String,
    lat: String,
    lng: String,
}

fn main() {
    /*
    let mut s = String::new();
    const PATH: &str = "/home/stephane/Code/Rust/filter-city-rust/assets/citys.json";
    File::open(PATH).unwrap().read_to_string(&mut s).unwrap();
    let _deserialized: Vec<City> = serde_json::from_str(&s).unwrap();
    for x in &_deserialized {
        println!("{}", x.name);
    }*/
    let s;
    const GENEVE: &str = "Genève";
    s = filter_city(GENEVE);
    println!("{}", s);
}

#[allow(unused_variables)]
fn filter_city(filter: &str) -> std::string::String {
    let mut json = String::new();
    let mut s = String::new();
    const PATH: &str = "/home/stephane/Code/Rust/filter-city-rust/assets/citys.json";
    File::open(PATH).unwrap().read_to_string(&mut s).unwrap();
    let _deserialized: Vec<City> = serde_json::from_str(&s).unwrap();
    for x in &_deserialized {
        // it's wanted to compute the for loop in its way
        // it's just for test with another rust program
        json = format!("{}", x.name);
        //json = [json, x.name.clone()].concat();
        // println!("{}", x.name);
    }
    json
}
