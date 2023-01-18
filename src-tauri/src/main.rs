#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]
#![allow(dead_code,unused_assignments)]
use chrono::FixedOffset;
use serde::Deserialize;
use std::cell::RefCell;

pub mod csv_in;
pub mod csv_out;

#[derive(Debug,Deserialize,Clone,PartialEq)]
pub struct Input{
    date_full: String,
    date: String,
    clock: String,
    pin: String,
    nip: String,
    name: String,
    occupation: Option<String>,
    departement: Option<String>,
    office : Option<String>,
    verivication : String,
    io :String,
    workcode : String,
    sn :String,
    machine: String
}

#[derive(Debug,PartialEq, Clone,Eq,PartialOrd, Ord)]
pub enum PrayTime {
    Duhur,
    Asyar,
    Maghrib,
    Isya,
    Subuh,
    Tahajud
}

#[derive(Debug)]
pub struct TimeLimit {
    duhur_s: String,
    duhur_f: String,
    asyar_s: String,
    asyar_f: String,
    maghrib_s: String,
    maghrib_f: String,
    isya_s: String,
    isya_f: String,
    subuh_s: String,
    subuh_f: String,
    tahajud_s: String,
    tahajud_f: String,
}

#[derive(Debug,PartialEq,Eq,PartialOrd,Ord,Clone)]
pub struct Hold {
    holder: Vec<Holder>
}
#[derive(Debug,PartialEq,Eq,PartialOrd,Ord,Clone)]
pub struct Holder {
    pin: String,
    name: String,
    pray: RefCell<Vec<PrayHold>>
}
#[derive(Debug,PartialEq,Eq,PartialOrd, Ord,Clone)]
pub struct PrayHold {
    date: (u32,u32,u32),
    pray:PrayTime,
    db_date:chrono::DateTime<FixedOffset>,
    machine:String
}
#[derive(Debug)]
struct CSVOUT {
    pin: String,
    name: String,
    date: (u32,u32,u32),
    pray: PrayTime,
    db_date: chrono::DateTime<FixedOffset>,
    machine: String
}
#[derive(Debug,serde::Serialize)]
struct TauriOut {
    machine: Vec<String>,
    start: String,
    finish:String,
    status:u16,
    data:u32,
    valid:u32,
    invalid:u32,
    double:u32
}

#[derive(Debug)]
pub struct CsvDebug {
    err_count: usize,
    valid_count:usize,
    invalid_count:usize,
    data_count:usize,
    double_count:usize
}
static mut CACHE:Hold=Hold{holder:Vec::new()};
// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn wtf(){
    println!("invoked");
}
#[tauri::command]
fn greet(path: String,all:String)-> String {
    println!("invoked");
    let mut hate = all.split(",");
    let tl = TimeLimit { 
        duhur_s:hate.next().unwrap().to_string(),
        duhur_f:hate.next().unwrap().to_string(),
        asyar_s:hate.next().unwrap().to_string(),
        asyar_f:hate.next().unwrap().to_string(),
        maghrib_s:hate.next().unwrap().to_string(),
        maghrib_f:hate.next().unwrap().to_string(),
        isya_s:hate.next().unwrap().to_string(),
        isya_f:hate.next().unwrap().to_string(),
        subuh_s:hate.next().unwrap().to_string(),
        subuh_f:hate.next().unwrap().to_string(),
        tahajud_s:hate.next().unwrap().to_string(),
        tahajud_f: hate.next().unwrap().to_string()};
    let holder = csv_in::csv2database(&path, &tl);
    if holder.is_ok(){
        let res = holder.unwrap();
        unsafe{
            CACHE = res.0.clone();
        }
        let out = TauriOut{
            status:200,
            start:res.0.get_range().0,
            finish:res.0.get_range().1,
            machine:res.0.get_machine(),
            data:res.1.data_count as u32,
            invalid:res.1.invalid_count as u32,
            valid:res.1.valid_count as u32,
            double:res.1.double_count as u32
        };
        return serde_json::to_string(&out).unwrap();
    }
    let out = TauriOut{status:400,start:String::new()
        ,finish:String::new(),machine:Vec::new(),data:0,invalid:0,valid:0,double:0};
    serde_json::to_string(&out).unwrap()
}
#[tauri::command]
fn get_result(machine:String,path:String)->bool{
    let mut hold = csv_in::new_hold();
    unsafe{
        hold = CACHE.clone();
    }
    let res = hold.direct_csv(&machine, &path);
    if res.is_ok(){
        return true;
    }
    false
}
fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![get_result,greet,wtf])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
