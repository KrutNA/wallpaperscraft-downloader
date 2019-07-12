extern crate clap;
extern crate reqwest;
extern crate select;
extern crate regex;

use clap::{Arg, App};
use select::predicate::Class;
use select::document::Document;
use std::collections::HashMap;
use regex::Regex;

fn main() {
    let mut map: HashMap<&str, String> = HashMap::new();
    parsing_arguments(&mut map);
    spawn_treads(&map);
}

fn download(vec: Vec<String>, st_num: i16, end: i16, cnt: i16) {
    let mut num = st_num; 
    loop {
        if num > end && end != 0 {
            break;
        }
        let req = match reqwest::get(
            &format!("{}{}", vec[0] , num)) {
            Err(_) => break,
            Ok(mut v) => v.text().unwrap(),
        };
        let doc = Document::from(&req[..]);
        println!(" Downloading page {}", num);
        for link in doc.find(Class("wallpapers__link")) {
            let tmp_arr: Vec<&str> = link.attr("href").unwrap().splitn(4, "/").collect();
            let tmp = [tmp_arr[2], tmp_arr[3]].join("_");
            std::fs::create_dir_all(
                &format!("files/{}/{}", vec[1], num)).unwrap();
            std::fs::write(
                &format!("files/{}/{}/{:width$}.jpg", vec[1], num, tmp,
                         width = match tmp.len() {
                             0...100 => tmp.len(),
                             _ => 100,
                         }),
                reqwest::get(
                    &format!("{}{}", vec[0], num)).unwrap()
                    .text().unwrap())
                .unwrap();
        }
        println!("  Finished downloading page {}", num);
        num += cnt;
    }
}

fn parse(val: &String) -> i16 {
    match val.parse::<i16>() {
        Ok(v) => v,
        Err(e) => panic!("{}", e),
    }
}

fn spawn_treads(map: &HashMap<&str, String>) {
    let num = parse(map.get("begin").unwrap());
    let end = parse(map.get("end").unwrap());
    let cnt = parse(map.get("threads").unwrap());
    std::fs::create_dir_all(
        &format!("files/{}", map.get("tag").unwrap())).unwrap();
    println!("Tag: {}", map.get("tag").unwrap());
    let handlers: Vec<_> = (0..cnt)
        .map(|i| {
            let vec = [
                map.get("url").cloned().unwrap(),
                map.get("tag").cloned().unwrap()
            ].to_vec();
            std::thread::spawn(move || download(vec, num+i, end, cnt))
        })
        .collect();
    for h in handlers {
        h.join().unwrap();
    }
}

fn parsing_arguments(map: &mut HashMap<&str, String>) {    
    let matches = App::new("Wallpaperscraft downloader")
        .version("0.1.0")
        .author("KrutNA <krutko_n_a@mail.ru>")
        .about("Downloads images from wallpaperscraft.ru")
        .arg(Arg::with_name("tag")
             .short("t").long("tag")
             .takes_value(true)
             .help("base tag for downloading")
             .validator(|x| match Regex::new(r"[[:alpha:]]+").unwrap().is_match(&x[..]) {
                 true => Ok(()),
                 _ => Err(String::from("Error tag"))
             })
             .default_value("games"))
        .arg(Arg::with_name("resolution")
             .short("r").long("s")
             .takes_value(true)
             .help("image resolution")
             .validator(|x| match Regex::new(r"^\d+x\d+$").unwrap().is_match(&x[..]) {
                 true => Ok(()),
                 _ => Err(String::from("Error resolution"))
             })
             .default_value("1366x768"))
        .arg(Arg::with_name("begin")
             .short("b").long("begin")
             .takes_value(true)
             .help("Begin page number")
             .validator(|x| match Regex::new(r"\d+").unwrap().is_match(&x[..]) {
                 true => Ok(()),
                 _ => Err(String::from("Error begin page"))
             })
             .default_value("1"))
        .arg(Arg::with_name("end")
             .short("e").long("end")
             .takes_value(true)
             .help("End page (\"0\" if to end)")
             .validator(|x| match Regex::new(r"\d+").unwrap().is_match(&x[..]) {
                 true => Ok(()),
                 _ => Err(String::from("Error end page"))
             })
             .default_value("0"))
        .arg(Arg::with_name("threads")
             .short("c").long("count")
             .takes_value(true)
             .help("Threads count")
             .validator(|x| match Regex::new(r"\d+").unwrap().is_match(&x[..]) {
                 true => Ok(()),
                 _ => Err(String::from("Error count of threads"))
             })
             .default_value("5"))
        .get_matches();
    map.insert("baseUrl", String::from("https://wallpaperscraft.ru/catalog"));
    map.insert("baseDownload", String::from("https://images.wallpaperscraft.ru/image/"));
    map.insert("tag", matches.value_of("tag").unwrap().to_owned());
    map.insert("resolution", matches.value_of("resolution").unwrap().to_owned());
    map.insert("begin", matches.value_of("begin").unwrap().to_owned());
    map.insert("end", matches.value_of("end").unwrap().to_owned());
    map.insert("threads", matches.value_of("threads").unwrap().to_owned());
    map.insert("url",
               [map.get("baseUrl").cloned().unwrap(), map.get("tag").cloned().unwrap(),
                map.get("resolution").cloned().unwrap(), String::from("page")]
               .join("/"));
}
