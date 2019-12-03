use rand::prelude::random;

const LOGIN_MAX_SIZE: u8 = 29;
const ALPHABETH_SIZE: u8 = 25;

const LOGIN_XOR: u16 = 0x5678;
const SERIAL_XOR: u16 = 0x1234;

fn generate_login() -> String {
    let size = random::<u8>() % LOGIN_MAX_SIZE + 1;
    let login: String = (0..size)
        .map(|_| (random::<u8>() % ALPHABETH_SIZE + 65) as char) // plus 'A'
        .collect();

    login
}

fn get_login_hash(login: String) -> u16 {
    login.chars().fold(0, |sum, x| sum + (x as u16)) ^ LOGIN_XOR
}

fn main() {
    println!("Crackme v1.0 Keygen by errmac-v");
    println!("------------------------------");

    let login = generate_login();
    println!("Login: {}", login);

    let password = get_login_hash(login) ^ SERIAL_XOR;
    println!("Password: {}", password);
    println!("------------------------------");
}
