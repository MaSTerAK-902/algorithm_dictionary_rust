use algorithm_dictionary::cryptosystem;

mod algorithm_dictionary;
fn main() {
    let plane_message = "Hello,World!";
    let secret_key = 123;
    let un_key = 12;

    //暗号化
    let encrypted = cryptosystem(plane_message, secret_key);
    println!("Encrypted: {:?}", encrypted);

    // 復号化
    let decrypted = cryptosystem(&String::from_utf8_lossy(&encrypted), un_key);
    let decrypted_message = String::from_utf8(decrypted).unwrap();
    println!("Decrypted: {:?}", decrypted_message);
}