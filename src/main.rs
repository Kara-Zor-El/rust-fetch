// crates
/*
use std::path::Path;
use walkdir::WalkDir;
use my_public_ip::resolve;
use std::path::Path;
*/ // unused crates
use local_ip_address::local_ip;
use std::str;
use std::process::Command;
use std::fs::File;
use std::fs;
use std::io::Read;
use titlecase::titlecase;
use uname::uname;
use std::env;
use libmacchina::GeneralReadout;
use sys_info::{mem_info, cpu_num};
use directories::ProjectDirs;
use serde::Deserialize;
use colored::{Colorize, Color};
use std::str::FromStr;
use whoami::*;
use rustix::process::Uname;
use std::time::SystemTime;
//use std::io::Write;


#[derive(Deserialize)]
struct Style {
    title: String,
    text: String,
}

pub struct ColorCodeIter<I: Iterator<Item = char>> {
    iter: I,
    color: Color,
    remainder: Option<char>,
    buffer: String,
}

impl<I: Iterator<Item = char>> ColorCodeIter<I> {
    pub fn new(iter: I) -> Self
    {
        Self { iter, color: Color::Blue /* or something like that */, remainder: None, buffer: String::with_capacity(4) }
    }
}

impl<I: Iterator<Item = char>> Iterator for ColorCodeIter<I> {
    type Item = (char, Color);

    fn next(&mut self) -> Option<(char, Color)> {
        if let Some(r) = self.remainder {
            self.remainder = None;
            return Some((r, self.color));
        }

        match self.iter.next() {
            Some('$') => match self.iter.next() {
                Some('{') => loop {
                    match self.iter.next() {
                        Some('}') => {
                            if let Ok(color) = Color::from_str(self.buffer.as_str()) {
                                self.color = color;
                            }
                            self.buffer.clear();

                            break Some((self.iter.next()?, self.color));
                        },
                        Some(v) => self.buffer.push(v),
                        None => (),
                    }
                },
                Some(v) => {
                    self.remainder = Some(v);
                    Some(('$', self.color))
                },
                None => None,
            },
            Some('\\') => Some((self.iter.next()?, self.color)),
            v => Some((v?, self.color)),
        }
    }
}

fn main() {
    let username = get_user().unwrap();
    let os = get_os().unwrap();
    let host = get_host().unwrap();
    let kernel = get_kernel().unwrap();
    let model = get_model().unwrap();
    println!("{}: {}", os.title, os.text);
    println!("{}: {}", username.title, username.text);
    println!("{}: {}", host.title, host.text);
    println!("{}: {}", kernel.title, kernel.text);
    println!("{}: {}", model.title, model.text);
}

fn get_os() -> Option<Style> {
    let os = whoami::distro();
    let style = Style {
        title: "OS".to_string(),
        text: os,
    };
    Some(style)
}

fn get_user() -> Option<Style> {
    let user = username();
    let style = Style {
        title: "User".to_string(),
        text: titlecase(&user),
    };
    Some(style)
}

fn get_host() -> Option<Style> {
    let host = hostname();
    let style = Style {
        title: "Host".to_string(),
        text: titlecase(&host),
    };
    Some(style)
}

fn get_model()-> Option<Style> {
    let machine_kind = if cfg!(unix) {
        "unix"
    } else if cfg!(windows) {
        "windows"
    } else if cfg!(macos) {
        "macos"
    } else {
        "unknown"
    };
    let model: String = match machine_kind {
        "unix" => {
            let product_name = fs::read_to_string("/sys/devices/virtual/dmi/id/product_name").unwrap().trim().to_string();
            let product_version = fs::read_to_string("/sys/devices/virtual/dmi/id/product_version").unwrap().trim().to_string();
            let model = format!("{} {}", product_name, product_version);
            model
        },
        "windows" => {
            let model = Command::new("wmic")
                .args(&["computersystem", "get", "manufacturer,model"])
                .output()
                .expect("failed to execute process")
                .stdout;
            let model = String::from_utf8_lossy(&model);
            let model = model.trim().to_string();
            model
        },
        "macos" => {
            let model = Command::new("sysctl")
                .arg("-n hw.model")
                .output()
                .expect("failed to execute process")
                .stdout;
            let model = str::from_utf8(&model).unwrap().to_string();
            model
        },
        _ => {
            let model = "Unknown".to_string();
            model
        },
    };
    let style = Style {
        title: "Model".to_string(),
        text: model,
    };
    Some(style)
}

fn get_kernel() -> Option<Style> {
    let kernel = uname().unwrap().release;
    let style = Style {
        title: "Kernel".to_string(),
        text: kernel,
    };
    Some(style)
}

pub fn uptime_time() -> String {
    let mut output = String::new();
    let mut uptime_f = File::open("/proc/uptime")
        .expect("Unable to open the file");
    let mut uptime = String::new();
    uptime_f.read_to_string(&mut uptime)
            .expect("Unable to open the file");
    let uptime: f32 = uptime.split(' ').collect::<Vec<&str>>()[0].parse().unwrap();

    let hour = uptime.round() as u32 / 3600;
    let rem = uptime as u32 - hour * 3600;
    let minutes = rem / 60;
    let day = hour as u32 / 24;
    let hour = &hour - day * 24;
    if day > 0 {
        output += &day.to_string();
        output += " days, ";
        output += &hour.to_string();
        output += " hours, ";
        output += &minutes.to_string();
        output += " min";
    } else if day <= 0 && hour > 0 {
        output += &hour.to_string();
        output += " hours, ";
        output += &minutes.to_string();
        output += " mins";
    } else {
        output += &minutes.to_string();
        output += " min";
    }
    output
}

pub fn gpu_find() -> String {
    let mut gpus = Command::new("sh");
    gpus.arg("-c");
    gpus.arg("lspci | grep -i 'vga\\|3d\\|2d' | cut -d ':' -f3 | cut -d '[' -f2 | cut -d ']' -f1");
    let gpu_out  = gpus.output()
        .expect("failed to execute process")
        .stdout;
    let gpu_out = match str::from_utf8(&gpu_out) {
        Ok(v) => v,
        Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
    };
    let gpu_out = &gpu_out.replace("\n", ", ");
    let gpu_out = &gpu_out[0..&gpu_out.len() - 2];
    // if gpu_out.contains(", ") {
    //     println!("GPUs: {}", gpu_out);
    // } else {
    //     println!("GPU: {}", gpu_out);
    // }

    gpu_out.to_string()
}

pub fn gtk_theme_find() -> String {
    let gtk_cmd = "cat $HOME/.config/gtk-3.0/settings.ini | grep gtk-theme-name | cut -d '=' -f2";
    let mut gtk_theme = Command::new("sh");
    gtk_theme.arg("-c");
    gtk_theme.arg(gtk_cmd);
    let gtk = gtk_theme.output()
                       .expect("failed to execute process")
                       .stdout;
    let gtk = match str::from_utf8(&gtk) {
        Ok(v) => v,
        Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
    };
    let gtk = &gtk.replace("\n", "");
    gtk.to_string()
}

pub fn gtk_icon_find() -> String {
    let gtk_cmd = "cat $HOME/.config/gtk-3.0/settings.ini | grep gtk-icon-theme-name | cut -d '=' -f2";
    let mut gtk_icon_theme = Command::new("sh");
    gtk_icon_theme.arg("-c");
    gtk_icon_theme.arg(gtk_cmd);
    let gtk_icon = gtk_icon_theme.output()
                                 .expect("failed to execute process")
                                 .stdout;
    let gtk_icon = match str::from_utf8(&gtk_icon) {
        Ok(v) => v,
        Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
    };
    let gtk_icon = &gtk_icon.replace("\n", "");
    gtk_icon.to_string()
}

pub fn cpu_usage_info() -> f32 {
    let cores = cpu_num().unwrap();

    let cpu_use_out = Command::new("sh")
        .arg("-c")
        .arg("ps aux | awk 'BEGIN {sum=0} {sum+=$3}; END {print sum}'")
        .output()
        .expect("failed to execute process")
        .stdout;

    let cpu_use = str::from_utf8(&cpu_use_out)
        .expect("cpu usage not utf-8")
        .trim()
        .parse::<f32>()
        .expect("cpu usage not a number");
    // let cpu_use = &cpu_use.replace("\n", "");
    let cpu_avg = (cpu_use / cores as f32).round();


    cpu_avg
}
pub fn battery_percentage() -> Option<(String, String)> {
    let battery_out = Command::new("sh")
        .arg("-c")
        .arg("upower -i `upower -e | grep 'BAT'` | grep 'percentage:' | tail -c 5")
        .output()
        .expect("failed to execute process")
        .stdout;
    let battery_per = str::from_utf8(&battery_out)
        .expect("battery output not utf-8")
        .trim()
        //.replace("%", "")
        //.parse::<i8>()
        .to_string();
        //.expect("battery output not a string");

    let state = Command::new("sh")
        .arg("-c")
        .arg("upower -i `upower -e | grep 'BAT'` | grep 'state:' | awk 'NF>1{print $NF}'")
        .output()
        .expect("failed to execute process")
        .stdout;

    let battery_state = str::from_utf8(&state)
        .expect("battery status not utf-8")
        .trim()
        .to_string();


    return Some((battery_per, battery_state));
}

pub fn user_list() -> Option<String> {
    let users = Command::new("sh")
        .arg("-c")
        .arg("awk -F':' '{ if($3 >= 1000 && $3 <= 6000) {print $1}}' /etc/passwd")
        .output()
        .expect("failed to execute process")
        .stdout;
    let user_out = str::from_utf8(&users)
        .expect("users output not utf-8")
        .trim()
        .replace("\n", ", ");
    return Some(user_out);
}

fn device_model() -> Option<String> {
    let mut product_name = File::open("/sys/devices/virtual/dmi/id/product_name")
        .expect("Unable to open the file");

    let mut product_version = File::open("/sys/devices/virtual/dmi/id/product_version")
        .expect("Unable to open the file");

    let mut model = String::new();
    product_name.read_to_string(&mut model)
        .expect("Unable to read the file"); // gets product name
    let _ = product_version.read_to_string(&mut model)
        .expect("Unable to read the file"); // get number revision
    let model = model.replace("\n", " ");
    return Some(model);
}
fn packages(which: &str) -> Option<String> {
    let mut how_many = String::new();
    if which == "package-managers" {
    // Do packages fully later
    // Package managers
    /* Todo List: // append all that exist to string and print string
    [X] pacman
    [X] apt
    [X] pip
    [X] cargo
    [X] flatpak
    [ ] appimages
     */
        // check if pacman exists
        let pac_e = std::path::Path::new("/bin/pacman").exists() | std::path::Path::new("/usr/bin/pacman").exists();
        // check if apt exists
        let apt_e = std::path::Path::new("/bin/apt").exists() | std::path::Path::new("/usr/bin/apt").exists();
        // check if pip exists
        // let pip_e = std::path::Path::new("/bin/pip").exists() | std::path::Path::new("/usr/bin/pip").exists();
        // check if cargo exists
        let cargo_e = std::path::Path::new("/bin/cargo").exists() | std::path::Path::new("/usr/bin/cargo").exists();
        // check if flatpak exists
        let flatpak_e = std::path::Path::new("/bin/flatpak").exists() | std::path::Path::new("/usr/bin/flatpak").exists();

        // checks how many files cargo has
        if cargo_e == true {
            let cargo_dir: String = "/home/".to_owned() + &whoami::username() + "/.cargo/bin";
            let cargo = fs::read_dir(cargo_dir).unwrap().count();
            how_many += "Cargo (";
            how_many += cargo.to_string().as_str();
            how_many += ")";
        }
        if pac_e == true {
            let pacman = Command::new("sh")
                .arg("-c")
                .arg("pacman -Q | wc -l")
                .output()
                .expect("failed to execute process")
                .stdout;
            let pacman_out = str::from_utf8(&pacman)
                .expect("pacman output not utf-8")
                .trim();
            if pacman_out != ""  {
                how_many += ", Pacman (";
                how_many += pacman_out;
                how_many += ")";
            }
        }
        if apt_e == true {
            let apt = Command::new("sh")
                .arg("-c")
                .arg("dpkg -l | wc -l")
                .output()
                .expect("failed to execute process")
                .stdout;
            let apt_out = str::from_utf8(&apt)
                .expect("apt output not uft-8")
                .trim();
            if apt_out != "" {
                how_many += ", APT (";
                how_many += apt_out;
                how_many += ")";
            }
        }
        /* if pip_e == true {
            let pip = Command::new("sh")
                .arg("-c")
                .arg("pip list | wc -l")
                .output()
                .expect("failed  to execute process")
                .stdout;
            let mut pip_out = str::from_utf8(&pip)
                .expect("pip status not utf-8")
                .trim()
                .parse::<i8>()
                .expect("pip output not uft-8");
            pip_out -= 2;
            if pip_out != 0 {
                how_many += ", pip (";
                how_many += pip_out.to_string().as_str();
                how_many += ")";
            }
        }*/

        if flatpak_e == true { // I think i used the wrong flatpak directories. should investagate when on wifi
            let flatpak_dir_system: String = "/var/lib/flatpak/app".to_string();
            let flatpak = fs::read_dir(flatpak_dir_system).unwrap().count();
            if flatpak.to_string() != "" {
                how_many += ", Flatpak (";
                how_many += flatpak.to_string().as_str();
                how_many += ")";
            }
        }

    } else if which == "path" {

        // Checks packages in all directories in $PATH
        let path = env::var("PATH").expect("$PATH is not set");
        let data = path.split(':');
        how_many = "".to_string();
        let home_dir = "/home/".to_owned() + &whoami::username();
        for s in data {
            // println!("PATH: {}", s);
            if s != "" && std::path::Path::new(s).exists() == true {
                if fs::read_dir(s).unwrap().count().to_string().as_str() != "0" {
                    if s.contains(&home_dir) {
                        how_many.push_str(s.replace(&home_dir, "~").to_string().as_str());
                    } else {
                        how_many.push_str(s);
                    }
                    how_many.push_str(" (");
                    how_many.push_str(fs::read_dir(s).unwrap().count().to_string().as_str());
                    how_many.push_str("), ");
                    // println!("Packages: {:?}", how_many);
                }
            }
        }
        how_many = (&how_many[0..how_many.len() - 2]).to_string();
    }
    Some(how_many.to_string())
}

pub fn wm_de() -> String {
    use libmacchina::traits::GeneralReadout as _;
    let general_readout = GeneralReadout::new();
    let resolution = general_readout.desktop_environment().expect("Failed to get desktop environment");
    resolution
}

pub fn ip() -> String {
   let my_local_ip = local_ip().unwrap().to_string();
   //let my_public_ip = my_public_ip::resolve().unwrap().to_string();

   return my_local_ip;
}

/* Todo:
[ X ] OS
[ X ] Host
[ X ] Model
[ X ] kernel
[ X ] Uptime
[ / ] Packages (add appimages)
[ X ] PATH Binaries
[ X ] Shell
[ X ] Resolution
[ X ] DE
[ X ] WM
[ X ] GTK Theme
[ X ] GTK Icons
[ X ] Terminal
[ N ] Terminal Font (as far as i can tell not possible unless testing in every terminal)
[ X ] CPU
[ X ] GPU
[ X ] Memory
Others:
[ X ] CPU Usage
[   ] Disk (KDE partition manager, my results and neofetches results do not line up with any of each other so will do more research later)
[ X ] Battery
[   ] Song
[ X ] Local IP
[ X ] Public IP (these 2 doubled runtime so disabled by default)
[ X ] Users
*/
/* Non-feature Specific Todos:
[ X ] Check for days with uptime
 */

/* ⚠ISSUES:⚠
- [ ] No wayland suppport
- [ ] inaccurate memory usage
*/
