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
use std::io::Read;
use titlecase::titlecase;
use uname::uname;
use std::env;
use libmacchina::GeneralReadout;
extern crate sys_info;
use sys_info::{mem_info, cpu_num};

fn main() {
    //println!("rustFetch");

    /* let mut host_name_file = File::open("/etc/hostname").unwrap();
    let mut host_name = String::new();
    host_name_file.read_to_string(&mut host_name).unwrap();*/

    let user_name = titlecase(&whoami::username()); // username
    let host_name = whoami::devicename(); // hostname
    let title_length = host_name.chars().count() + user_name.chars().count() + 1; //length of hostname and username + @ symbol
    let os = whoami::distro(); // distro (and version if not rolling release)


    // devices model
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

    let kernel = uname().unwrap().release; // kernel


    // Do packages fully later
    // Package managers
    /* Todo List: // append all that exist to string and print string
    [/] pacman
    [/] apt
    [/] pip
    [X] cargo
    [/] flatpak
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
    let cargo_dir: String = "/home/".to_owned() + &whoami::username() + "/.cargo/bin";
    let cargo = fs::read_dir(cargo_dir).unwrap().count();

    // Checks packages in all directories in $PATH
    let mut path = env::var("PATH").expect("$PATH is not set");
    let data = path.split(':');
    let mut how_many = "".to_string();
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
    let how_many = &how_many[0..how_many.len() - 2];


    // User Shell
    let usr_shell = env::var("SHELL").expect("$SHELL is not set");

    // Checks users desktop Env
    let mut de = env::var("XDG_CURRENT_DESKTOP")
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
    println!("{}@{}", user_name, host_name);
    // println!("{}", title_length); // prints length of title
    println!("{:-<1$}", "", title_length);
    println!("OS: {}", os);
    println!("model: {}", model);
    println!("Kernel: {}", kernel);
    uptime_time();
    // files exists?
    /* println!(" - Pacman exists? {}", pac_e);
    println!(" - APT exists? {}", apt_e);
    println!(" - PIP exists? {}", pip_e);
    println!(" - Cargo exists? {}", cargo_e);
    println!(" - Flatpak exists? {}", flatpak_e);
    println!(" - Cargo ({})", cargo); */
    println!("Packages: {}", how_many);
    println!("Defualt Shell: {}", usr_shell);
    println!("Screen Resolution: {}", res_info);
    println!("DE/WM: {}", de);
    gtk_theme_find();
    gtk_icon_find();
    println!("Terminal: {}", terminal);
    println!("CPU: {} ({}%)", cpu_info, cpu_usage_info());
    gpu_find();
    println!("Memory: {}Mib / {}Mib ({:.2}%)", mem_used, mem.total/1024, mem_percent);
    let (per, state) = battery_percentage();
    println!("Battery: {}% [{}]", per, state);
    // let (local_ip, public_ip) = ip();
    // println!("IP: {} [Local], {} [Public]", local_ip, public_ip);
}

pub fn uptime_time(){
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
        println!("Uptime: {} days, {} hours, {} min", day, hour, minutes);
    } else if day <= 0 && hour > 0 {
        println!("Uptime: {} hours, {} min", hour, minutes);
    } else {
        println!("Uptime: {} min", minutes);
    }
}

pub fn gpu_find() {
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
    if gpu_out.contains(", ") {
        println!("GPUs: {}", gpu_out);
    } else {
        println!("GPU: {}", gpu_out);
    }
}

pub fn gtk_theme_find(){
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
    println!("GTK Theme: {}", gtk);
}

pub fn gtk_icon_find(){
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
    println!("GTK Icon Theme: {}", gtk_icon);
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
    return cpu_avg;
}
pub fn battery_percentage() -> (i8, String) {
    let battery_out = Command::new("sh")
        .arg("-c")
        .arg("upower -i `upower -e | grep 'BAT'` | grep 'percentage:' | tail -c 5")
        .output()
        .expect("failed to execute process")
        .stdout;
    let battery_per = str::from_utf8(&battery_out)
        .expect("battery output not utf-8")
        .trim()
        .replace("%", "")
        .parse::<i8>()
        .expect("battery output not a number");

    let state = Command::new("sh")
        .arg("-c")
        .arg("upower -i `upower -e | grep 'BAT'` | grep 'state:' | awk 'NF>1{print $NF}'")
        .output()
        .expect("failed ot execute process")
        .stdout;

    let battery_state = str::from_utf8(&state)
        .expect("battery status not utf-8")
        .trim()
        .to_string();
    return (battery_per, battery_state);
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
[ / ] Packages
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
[ X ] Public IP
[   ] Users
*/
/* Non-feature Specific Todos:
[ X ] Check for days with uptime
*/
