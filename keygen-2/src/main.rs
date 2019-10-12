use rand::prelude::random;

const LOGIN_MASK: u32 = 0xEDB88320;
const STEPS: u8 = 8;

const LOGIN_MAX_SIZE: u8 = 29;
const ALPHABETH_SIZE: u8 = 25;

fn generate_login() -> String {
    let size = random::<u8>() % LOGIN_MAX_SIZE + 1;
    let login: String = (0..size)
        .map(|_| (random::<u8>() % ALPHABETH_SIZE + 65) as char) // plus 'A'
        .collect();

    login
}

fn hash_login(login: String) -> u32 {
    let mut login_hash = 0xFFFFFFFF;

    for x in login.chars() {
        login_hash ^= x as u32;

        for _ in 0..STEPS {
            let negative = (login_hash & 1).overflowing_neg().0;
            let temp = negative & LOGIN_MASK;
            login_hash = login_hash.overflowing_shr(1).0;
            login_hash ^= temp;
        }
    }

    !login_hash & 0xFF
}

fn hash_password(password: String) -> u32 {
    let acc = password.chars().into_iter().fold(0, |acc, x| {
        let character = x as u32;
        acc + (character ^ 0x99)
    });

    acc & 0xFF
}

fn compute_hash_password(hash: u32) -> String {
    let alphabeth = get_alphabeth();
    let mut chars = vec![];

    for x in &alphabeth {
        let mut password_hash = x ^ 0x99;
        chars.clear();
        chars.push(x);

        for y in &alphabeth {
            password_hash += y ^ 0x99;
            chars.push(y);

            if hash == password_hash & 0xFF {
                return chars.into_iter().map(|i| *i as u8 as char).collect();
            }
        }
    }

    chars.into_iter().map(|i| *i as u8 as char).collect()
}

fn get_alphabeth() -> Vec<u32> {
    let mut alphabeth: Vec<u32> = vec![];

    alphabeth.append(&mut (0x30..0x39).collect()); // [0, 9]
    alphabeth.append(&mut (0x41..0x5A).collect()); // [A, Z]
    alphabeth.append(&mut (0x61..0x7A).collect()); // [a, z]

    alphabeth
}

fn main() {
    println!("Crackme OTUS-1 Keygen by tmbrlkV");
    println!("------------------------------");

    let login = generate_login();
    let login_hash = hash_login(login.to_owned());
    println!("Login {}", login);

    let password = compute_hash_password(login_hash);
    println!("Password: {}", format!("{}", password));
    let password_hash = hash_password(password);
    println!("Valid hashes: {}", password_hash == login_hash);

    println!("------------------------------");
}
