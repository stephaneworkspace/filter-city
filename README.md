# filter-city
```
   __ _ _ _                      _ _         
  / _(_) | |_ ___ _ __       ___(_) |_ _   _ 
 | |_| | | __/ _ \ '__|____ / __| | __| | | |
 |  _| | | ||  __/ | |_____| (__| | |_| |_| |
 |_| |_|_|\__\___|_|        \___|_|\__|\__, |
                                      |___/ 
 By Stéphane Bressani
  ____  _             _
 / ___|| |_ ___ _ __ | |__   __ _ _ __   ___
 \___ \| __/ _ \ '_ \| '_ \ / _` | '_ \ / _ \
  ___) | ||  __/ |_) | | | | (_| | | | |  __/
 |____/ \__\___| .__/|_| |_|\__,_|_| |_|\___|
               | |stephane-bressani.ch
               |_|github.com/stephaneworkspace

``` 
## Introduction
This program allows to search a city in ASCII all over the world in a json file.
Initalliy I have done a script with Python but thas was very slow.
## Use
```
extern crate filter_city;

let search Vec<filter_city::City> = filter_city::filter_city("Search...");
```
## Version

0.1.3
* This program allows to search a city in ASCII all over the world in a json
  file.
