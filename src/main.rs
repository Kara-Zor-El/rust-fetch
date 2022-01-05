// crates
/*
use std::path::Path;
use walkdir::WalkDir;
use local_ip_address::local_ip;
use my_public_ip::resolve;
*/ // unused crates
use std::str;
use std::process::Command;
use std::fs::File;
use std::fs;
use std::io::{Read, Error, BufRead, BufReader};
use titlecase::titlecase;
use uname::uname;
use std::env;
use libmacchina::GeneralReadout;
use sys_info::{mem_info, cpu_num};
use directories::ProjectDirs;
use serde::Deserialize;
use colored::Colorize;
use std::path::Path;

#[derive(Deserialize)]
struct Config {
    packages: String,
    info_color: String,
    logo_color: String,
    os: String,
}

fn main() {
    let user_name = titlecase(&whoami::username()); // username
    let host_name = whoami::devicename(); // hostname
    let title_length = host_name.chars().count() + user_name.chars().count() + 3; //length of hostname and username + @ symbol
    let os = whoami::distro(); // distro (and version if not rolling release)

    let path = Path::new("src/ascii_art/arch");
    let file = File::open(path).expect("File not found or cannot be opened");
    let content = BufReader::new(&file);
    let lines = content.lines();

    let kernel = uname().unwrap().release; // kernel

    // User Shell
    let usr_shell = env::var("SHELL").expect("$SHELL is not set");

    // Checks users desktop Env
    let de = env::var("XDG_CURRENT_DESKTOP")
        .expect("$XDG_CURRENT_DESKTOP is not set"); /* +
        " " +
        &env::var("DESKTOP_SESSION")
        .expect("$DESKTOP_SESSION is not set"); */
    //de = titlecase(&de);

    // Checks current terminal
    use libmacchina::traits::GeneralReadout as _;
    let mut terminal = titlecase(&GeneralReadout::new().terminal().unwrap());
    if terminal == "Kitty" {
        terminal = terminal + " 🐱";
    }

    // Checks CPU name and cores
    let cpu_info = GeneralReadout::new().cpu_model_name().unwrap();

    // Checks which GPUs you have
    //gpu_find();

    // Checks memory info
    let mem = mem_info().unwrap();
      let mem_used = mem.total/1024 - mem.avail/1024;
      let mem_percent: f32  = ((mem_used as f32)/((mem.total as f32)/1024.0)*100.0) as f32;

    // Checks Screen Resolution info
    let res_info = GeneralReadout::new().resolution().unwrap();

    // print outs
    // println!("{}", title_length); // prints length of title
    if let Some(proj_dirs) = ProjectDirs::from("dev", "Kara-Wilson", "rust-fetch") {
        let config_dir = proj_dirs.config_dir();

        let config_file = fs::read_to_string(
            config_dir.join("config.toml"),
        );

        let config: Config = match config_file {
            Ok(file) => toml::from_str(&file).unwrap(),
            Err(_) => Config {
                packages: "path".to_string(),
                info_color: "blue".to_string(),
                logo_color: "magenta".to_string(),
                os: "arch".to_string(),
            },
        };

        let modules:[String;18] = [
            format!("{} {} {}",
                    user_name.color(config.info_color.clone()),
                    "@".blue().bold(),
                    host_name.color(config.info_color.clone())),
            format!("{:—<1$}", "", title_length),



            format!("{} {}",
                     "OS:".color(config.info_color.clone()).bold(),
                     os.normal()),

            if let Some(model) = device_model() {
                if model != "" {
                    format!("{} {}",
                             "Model:".color(config.info_color.clone()).bold(),
                             model.normal())
                } else {
                    format!("{} {}",
                            "Model:".color(config.info_color.clone()).bold(),
                            "Model not found".normal())
                }
            } else {
                format!("{} {}",
                        "Model:".color(config.info_color.clone()).bold(),
                        "Model not found".normal())
            },

            format!("{} {}",
                     "Kernel:".color(config.info_color.clone()).bold(),
                     kernel.normal()),

            format!("{} {}",
                     "Uptime:".color(config.info_color.clone()).bold(),
                     uptime_time().normal()),
            if let Some(how_many) = packages(&config.packages) {

                if how_many != "" {
                    format!("{} {}",
                             "Packages:".color(config.info_color.clone()).bold(),
                             how_many.normal())
                } else {
                    format!("{} {}",
                            "Packages:".color(config.info_color.clone()).bold(),
                            "packages not found".normal())
                }
            } else {
                format!("{} {}",
                            "Packages:".color(config.info_color.clone()).bold(),
                            "packages not found".normal())
            },

            if usr_shell != "" {
                format!("{} {}",
                         "Defualt Shell:".color(config.info_color.clone()).bold(),
                         usr_shell.normal())
            } else {
                format!("{} {}",
                        "Default Shell:".color(config.info_color.clone()).bold(),
                        "defualt shell not found".normal())
            },
            format!("{} {}",
                     "Screen Resolution:".color(config.info_color.clone()).bold(),
                     res_info.normal()),
            format!("{} {}",
                     "DE/WM:".color(config.info_color.clone()).bold(),
                     de.normal()),
            format!("{} {}",
                     "GTK Theme:".color(config.info_color.clone()).bold(),
                     gtk_theme_find().normal()),
            format!("{} {}",
                     "GTK Icon Theme:".color(config.info_color.clone()).bold(),
                     gtk_icon_find().normal()),
            format!("{} {}",
                     "Terminal:".color(config.info_color.clone()).bold(),
                     terminal.normal()),
            format!("{} {} {}{}{}",
                     "CPU:".color(config.info_color.clone()).bold(),
                     cpu_info.normal(),
                     "(".normal(),
                     cpu_usage_info(),
                     "%)"),

            if gpu_find().contains(", ") {
                format!("{} {}",
                         "GPUs:".color(config.info_color.clone()).bold(),
                         gpu_find().normal())
            } else {
                format!("{} {}",
                         "GPU: {}".color(config.info_color.clone()).bold(),
                         gpu_find().normal())
            },

            format!("{} {}{}{}{}{:.2}{}",
                     "Memory:".color(config.info_color.clone()).bold(),
                     mem_used.to_string().normal(),
                     "Mib / ",
                     mem.total/1024,
                     "Mib (",
                     mem_percent,
                     "%)"),
            if let Some((per, state)) = battery_percentage() {
                if per != "" && state != "" {
                    format!("{} {} {}{}{}",
                             "Battery:".color(config.info_color.clone()).bold(),
                             per.normal(),
                             "[",
                             state,
                             "]")
                } else if per != "" {
                    format!("{} {}",
                             "Battery:".color(config.info_color.clone()).bold(),
                             per.normal())
                } else {
                    format!("{} {}",
                            "Battery:".color(config.info_color.clone()).bold(),
                            "battery info not found".normal())
                }
            } else {
               format!("{} {}",
                       "Battery:".color(config.info_color.clone()).bold(),
                        "battery info not found".normal())
            },

            if let Some(users) = user_list() {
                if users != "" {
                    format!("{} {}",
                             "Users:".color(config.info_color.clone()).bold(),
                             users.normal())
                } else {
                    format!("{} {}",
                            "Users:".color(config.info_color.clone()).bold(),
                            "users not found".normal())
                }
            } else {
                format!("{} {}",
                        "Users:".color(config.info_color.clone()).bold(),
                        "users not found".normal())
            }
        ];

        for (x, line) in modules.into_iter().zip(lines) {
            println!("{}{}",
                     line.expect("failed to fetch ASCII art").color(config.logo_color.clone()).bold(),
                     x);
        }
    }
    // let (local_ip, public_ip) = ip();
    // println!("IP: {} [Local], {} [Public]", local_ip, public_ip);
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
        let pip_e = std::path::Path::new("/bin/pip").exists() | std::path::Path::new("/usr/bin/pip").exists() | std::path::Path::new("/bin/pip3").exists() | std::path::Path::new("/usr/bin/pip3").exists();
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
        if pip_e == true {
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
        }

        if flatpak_e == true { // I think i used the wrong flatpak directories. should investagate when on wifi
            let flatpak_dir_system: String = "/var/lib/flatpak".to_string();
            let flatpak_dir_user: String = "/home/".to_string() + &whoami::username().to_string() + "/.local/share/flatpak";
            let flatpak = fs::read_dir(flatpak_dir_system).unwrap().count() + fs::read_dir(flatpak_dir_user).unwrap().count();
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

// pub fn ip() -> (String, String){
//    let my_local_ip = local_ip().unwrap().to_string();
//    let my_public_ip = my_public_ip::resolve().unwrap().to_string();
//
//    return (my_local_ip, my_public_ip);
//}

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
