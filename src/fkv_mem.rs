use libfrankv::FranKV;


#[cfg(target_os = "windows")]
const USAGE: &str = "
Usage:
    fkv_mem.exe FILE get KEY
    fkv_mem.exe FILE delete KEY
    fkv_mem.exe FILE insert KEY VALUE
    fkv_mem.exe FILE update KEY VALUE
";

#[cfg(not(target_os = "windows"))]
const USAGE: &str = "
Usage:
    fkv_mem.exe FILE get KEY
    fkv_mem.exe FILE delete KEY
    fkv_mem.exe FILE insert KEY VALUE
    fkv_mem.exe FILE update KEY VALUE
";

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let fname = args.get(1).expect(&USAGE);
    let action: &[u8] = args.get(2).expect(&USAGE).as_ref();
    let key: &[u8] = args.get(3).expect(&USAGE).as_ref();

    let path = std::path::Path::new(&fname);
    let mut store = FranKV::open(path).expect("unable to open file");
    store.load().expect("unable to load data");

    match action {
        "get" => match store.get(key).unwrap {
            None => eprintln!("{:?} not found", key),
            Some(value) => println!("{:?}", value),
        },

        "delete" => store.delete(key).unwrap(),

        "insert" => {
            let value = maybe_value.expect(&USAGE).as_ref();
            store.insert(key, value).unwrap()
        },

        "update" => {
            let value = maybe_value.expect(&USAGE).as_ref();
            store.update(key, value).unwrap()
        },

        _ => eprintln!("{}", &USAGE),
    }
}