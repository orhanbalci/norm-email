use norm_email::Normalizer;

fn main() {
    let normalizer = Normalizer::new();
    let result = normalizer.normalize("orhan.balci@gmail.com").unwrap();
    println!("{:#?}", result.mx_records);
    println!("{}", result.mailbox_provider.unwrap());
    println!("{}", result.normalized_address);
}
