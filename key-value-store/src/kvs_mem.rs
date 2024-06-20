use libkeyvaluestore::ActionKV;

#[cfg(target_os = "windows")]
const USAGE: &str = "
Usage:
    kvs_mem.exe FILE get KEY
    kvs_mem.exe FILE delete KEY
    kvs_mem.exe FILE insert KEY
    kvs_mem.exe FILE update KEY
";

#[cfg(not(target_os = "windows"))]
const USAGE: &str = "
Usage:
    kvs_mem FILE get KEY
    kvs_mem FILE delete KEY
    kvs_mem FILE insert KEY
    kvs_mem FILE update KEY
";

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let fname = args.get(1).expect(&USAGE);
    let action = args.get(2).expect(&USAGE).as_ref();
    let key = args.get(3).expect(&USAGE).as_ref();
    let maybe_value = args.get(4);

    let path = std::path::Path::new(&fname);
    let mut a = ActionKV::open(path).expect("unable to open file");
    a.load().expect("unable to load data");

    match action {
        "get" => {
            match a.get(key).unwrap() {
                None => eprintln!("{:?} not found", key),
                Some(value) => println!("{:?}", value),
            }
        }
        "delete" => a.delete(key).unwrap(),
        "insert" => {
            let value = maybe_value.expect(&USAGE).as_ref();
            a.insert(key, value).unwrap()
        }
        "update" => {
            let value = maybe_value.expect(&USAGE).as_ref();
            a.update(key, value).unwrap()
        }
        _ => eprintln!("{}", &USAGE),
    }
}