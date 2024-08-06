use libc::c_char;
use once_cell::sync::Lazy;
use once_cell::sync::OnceCell;
use serde::Deserialize;
use std::env;
/// .dll file loader
use std::ffi::CStr;
use std::ffi::CString;
use std::fs;
use std::io::Read;
//width->i32
//height->i32
//angle->f64
//altitude ->i32
#[derive(Deserialize)]
struct Param {
    width:i32,
    height:i32,
    title:String,
    option:String,
}
static WIDTH: OnceCell<i32> = OnceCell::new();
static HEIGHT: OnceCell<i32> = OnceCell::new();
static TITLE: OnceCell<String> = OnceCell::new();
static OPTION:OnceCell<String>=OnceCell::new();
#[no_mangle]
pub unsafe extern "C" fn values() {
    let mut directory = env::current_dir()
        .unwrap()
        .as_os_str()
        .to_str()
        .unwrap()
        .to_string();
    directory.push_str("\\Settings.toml");
    let read_content = fs::read_to_string(directory).unwrap();
    let toml_content: Param = toml::from_str(read_content.trim()).unwrap();
    WIDTH.set(toml_content.width).unwrap();
    HEIGHT.set(toml_content.height).unwrap();
    TITLE.set(toml_content.title).unwrap();
    OPTION.set(toml_content.option).unwrap();

}
#[no_mangle]
pub unsafe extern "C" fn width() -> i32 {
    WIDTH.get().unwrap().to_owned()
}
#[no_mangle]
pub unsafe extern "C" fn height() -> i32 {
    HEIGHT.get().unwrap().to_owned()
}
//CString
#[no_mangle]
pub unsafe extern "C" fn title() -> *mut c_char {
    let title = CString::new(TITLE.get().unwrap().trim()).unwrap();
    let pointer = title.into_raw();
    pointer
}
#[no_mangle]
pub unsafe extern "C" fn option()->*mut c_char{
let option=CString::new(OPTION.get().unwrap().trim()).unwrap();
let pointer=option.into_raw();
    pointer
}
