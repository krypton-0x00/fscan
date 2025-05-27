use std::path::PathBuf;
use std::fs;
use std::process::Command;
use std::env;

fn get_cache_path() -> PathBuf {
    let home = std::env::var("HOME").expect("HOME env var not set");
    let mut path = PathBuf::from(home);
    path.push(".cache/fscan");
    path.push("nmap_installed");
    path
}

fn is_nmap_installed_cached() -> bool {
    let cache_file = get_cache_path();
    cache_file.exists()
}

fn write_nmap_cache() -> std::io::Result<()> {
    let cache_file = get_cache_path();

    if let Some(dir) = cache_file.parent() {
        fs::create_dir_all(dir)?;
    }

    fs::write(cache_file, b"installed")
}

fn check_and_install_nmap() -> Result<(), Box<dyn std::error::Error>> {
    if is_nmap_installed_cached() {
        println!("[+] Cached nmap installation found. Skipping install.");
        return Ok(());
    }

    if Command::new("which").arg("nmap").status()?.success() {
        println!("[+] nmap is already installed.");
        write_nmap_cache()?;  
        return Ok(());
    }

    println!("[!] nmap not found. Installing...");
    if std::path::Path::new("/etc/arch-release").exists() {
        println!("[+] Arch-based system detected.");
        Command::new("sudo")
            .args(["pacman", "-Sy", "--noconfirm", "nmap"])
            .status()?;
    } else if std::path::Path::new("/etc/debian_version").exists() {
        println!("[+] Debian-based system detected.");
        Command::new("sudo").args(["apt", "update"]).status()?;
        Command::new("sudo").args(["apt", "install", "-y", "nmap"]).status()?;
    } else {
        eprintln!("[-] Unsupported OS. Please install nmap manually.");
        return Err("Unsupported OS".into());
    }

    write_nmap_cache()?;
    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    check_and_install_nmap()?;

    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <ip>", args[0]);
        std::process::exit(1);
    }
    let ip = &args[1];

    println!("[+] Running rustscan on {}", ip);
    let output = Command::new("rustscan")
        .args(["-g", "-a", ip])
        .output()
        .expect("Failed to run rustscan");

    let stdout = String::from_utf8_lossy(&output.stdout);
    let ports_raw = stdout
        .lines()
        .find(|line| line.contains("-> ["))
        .and_then(|line| line.split("[").nth(1))
        .and_then(|rest| rest.strip_suffix("]"))
        .unwrap_or("")
        .trim();

    if ports_raw.is_empty() {
        eprintln!("[-] No open ports found.");
        return Ok(());
    }

    let ports: Vec<&str> = ports_raw.split(',').map(|p| p.trim()).collect();
    println!("[+] {} ports are open: {:?}", ports.len(), ports);


    let port_str = ports.join(",");
    println!("[+] Running nmap -p{} on {}", port_str, ip);
    Command::new("nmap")
        .args(["-sC", "-sV", "-T5", "-Pn", "-p", &port_str, ip])
        .status()
        .expect("Failed to run nmap");

    Ok(())
}

