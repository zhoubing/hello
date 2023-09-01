use std::collections::hash_map::HashMap;
use std::fs::File;
use std::fs::OpenOptions;
use std::io;
use std::io::BufReader;
use std::io::BufWriter;
use std::io::Cursor;
use std::io::Read;
use std::io::Result;
use std::io::Seek;
use std::io::SeekFrom;
use std::io::Write;
use std::path::Path;
use std::path::PathBuf;
use std::collections::BTreeMap;

use byteorder::LittleEndian;
use byteorder::ReadBytesExt;
use byteorder::WriteBytesExt;
use crc::crc32;
use serde_derive::Deserialize;
use serde_derive::Serialize;
use serde_json::json;

const USAGE: &str = "
    Usage:
        1
        2
        3
";

const INDEX_KEY :&ByteStr = b"+index";

pub fn run_akv_disk() {
    let args: Vec<String> = std::env::args().collect();
    let fname = args.get(2).expect(USAGE);
    println!("fname: {}", fname);

    let action: &str = args.get(3).expect(USAGE).as_ref();
    let key: &ByteStr = args.get(4).expect(USAGE).as_ref();
    let maybe = args.get(5);
    println!("run_akv_disk action: {}", action);
    
    let mut a = ActionKV::open(Path::new(fname)).expect("open file error!");
    a.load().expect("load data error!");
    match action {
        "get" => {
            let index_as_bytes = a.get(INDEX_KEY).unwrap().unwrap();
            let index: HashMap<ByteString, u64> = bincode::deserialize(&index_as_bytes).unwrap();
            match index.get(key) {
                Some(i) => {
                    let kv = a.get_at(*i).unwrap();
                    println!("run_akv_disk value is {:?}", kv.value);
                },
                None => {

                }
            }
        }
        "insert" => {
            let value = maybe.expect(USAGE).as_ref();
            a.insert(key, value).unwrap();
            store_index_on_disk(&mut a, INDEX_KEY);
        }
        "delete" => {
            a.delete(key).unwrap();
        }
        "update" => {
            let value = maybe.expect(USAGE).as_ref();
            a.update(key, value).unwrap();
            store_index_on_disk(&mut a, INDEX_KEY);
        }
        _ => {
            eprintln!("{}", USAGE);
        }
    }

}

pub fn run_akv_mem() {
    let args: Vec<String> = std::env::args().collect();
    let fname = args.get(2).expect(USAGE);
    println!("fname: {}", fname);

    let action: &str = args.get(3).expect(USAGE).as_ref();
    let key: &ByteStr = args.get(4).expect(USAGE).as_ref();
    let maybe = args.get(5);
    println!("run_akv_disk action: {}", action);
    
    let mut a = ActionKV::open(Path::new(fname)).expect("open file error!");
    a.load().expect("load data error!");
    match action {
        "get" => {
            match a.get(key).unwrap() {
                Some(i) => {
                    println!("run_akv_mem value is {:?}", i);
                },
                None => {

                }
            }
        }
        "insert" => {
            let value = maybe.expect(USAGE).as_ref();
            a.insert(key, value).unwrap();
        }
        "delete" => {
            a.delete(key).unwrap();
        }
        "update" => {
            let value = maybe.expect(USAGE).as_ref();
            a.update(key, value).unwrap();
        }
        _ => {
            eprintln!("{}", USAGE);
        }
    }
}

pub fn run() {
    let args: Vec<String> = std::env::args().collect();
    println!("{:?}", args);

    let nth = "1/2/3".split("/").nth(1);
    println!("{}", nth.unwrap());
    println!("{}", nth.expect("not found"));

    let mut pb = PathBuf::from("/tmp/1.txt");
    pb.pop();
    println!("{}", pb.display());

    let fname = args.get(2).expect(USAGE);
    println!("fname: {}", fname);

    let action = args.get(3).expect(USAGE).as_ref();
    let key = args.get(4).expect(USAGE);
    let maybe = args.get(5);

    println!("action: {}", action);
    println!("key: {}", key);

    match maybe {
        Some(i) => {
            println!("maybe: {}", i);
        }
        None => {}
    }

    match action {
        "get" => {}

        _ => {}
    }

    let mut w: Vec<u8> = vec![];
    let (one, two, three) = write_numbers_to_file(&mut w);
    let (one_, two_, three_) = read_numbers_from_file(&w);

    assert_eq!(one, one_);
    assert_eq!(two, two_);
    assert_eq!(three, three_);

    println!("{:08x}", parity_bit(b"abc"));
    println!("{:08x}", parity_bit(b"abcd"));

    println!("bufreader test begin=================>");
    let f: File = File::open("111.txt").unwrap();
    let mut br: BufReader<File> = BufReader::new(f);
    let r = br.seek(SeekFrom::Current(2));
    match r {
        Ok(r) => println!("seek is {}", br.read_u8().unwrap()),
        Err(e) => {
            match e.kind() {
                io::ErrorKind::UnexpectedEof => {
                    println!("seek is {}", br.read_u8().unwrap());
                }
                _ => panic!("seek is out of range")
            }
        }
    }
    
    let mut cs = "h".chars();
    let c = cs.next().unwrap_or('!');
    println!("hashmap letter is {}", c);
    println!("hashmap letter is {}", u32::from(c));
    let c = cs.next().unwrap_or('!');
    println!("hashmap letter is {}", c);

    let mut hm = HashMap::new();
    hm.insert("k", "v");
    println!("hashmap {:?}", hm);
    println!("hashmap {}", hm["k"]);

    let cap = json!({
        "name" : "zhoubing",
        "age" : 42,
        "addr" : "sheyang"
    });
    println!("json {:?}", cap);
    println!("json {}", cap["name"]);

    let mut bm = BTreeMap::new();
    bm.insert("k", "v");
    println!("btreemap {:?}", bm);

}

type ByteString = Vec<u8>;
type ByteStr = [u8];

pub struct ActionKV {
    f: File,
    pub index: HashMap<ByteString, u64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct KeyValuePair {
    pub key: ByteString,
    pub value: ByteString,
}

fn store_index_on_disk(a: &mut ActionKV, index_key: &ByteStr) {
    a.index.remove(index_key);
    let index_as_bytes = bincode::serialize(&a.index).unwrap();
    a.index = HashMap::new();
    a.insert(index_key, &index_as_bytes).unwrap();
}

impl ActionKV {
    pub fn open(path: &Path) -> Result<Self> {
        let f = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .append(true)
            .open(path)?;
        let index = HashMap::new();
        Ok(ActionKV { f, index })
    }

    pub fn load(&mut self) -> Result<()> {
        let mut f = BufReader::new(&self.f);
        loop {
            let position = f.seek(SeekFrom::Current(0))?;
            let maybe_kv = ActionKV::process_record(&mut f);
            let kv = match maybe_kv {
                Ok(kv) => kv,
                Err(err) => match err.kind() {
                    io::ErrorKind::UnexpectedEof => {
                        break;
                    }
                    _ => return Err(err),
                },
            };
            self.index.insert(kv.key, position);
        }
        Ok(())
    }

    pub fn process_record<R: Read>(f: &mut R) -> Result<KeyValuePair> {
        let saved_checksum = f.read_u32::<LittleEndian>()?;
        let key_len = f.read_u32::<LittleEndian>()?;
        let value_len = f.read_u32::<LittleEndian>()?;
        let data_len = key_len + value_len;
        let mut data = ByteString::with_capacity(data_len.try_into().unwrap()); //或者用as强转????

        f.take(data_len.into()).read_to_end(&mut data);

        debug_assert_eq!(data_len as usize, data.len());
        let checksum = crc32::checksum_ieee(&data);
        if saved_checksum != checksum {
            panic!("data corruption!!!")
        }
        let value = data.split_off(key_len as usize);
        let key = data;

        Ok(KeyValuePair { key, value })
    }

    pub fn insert(&mut self, key: &ByteStr, value: &ByteStr) -> io::Result<()> {
        let position = self.insert_but_ignore_index(key, value)?;
        self.index.insert(key.to_vec(), position);
        Ok(())
    }

    pub fn insert_but_ignore_index(&mut self, key: &ByteStr, value: &ByteStr) -> io::Result<u64> {
        let mut bw = BufWriter::new(&self.f);
        let key_len = key.len();
        let val_len = value.len();

        let mut tmp = ByteString::with_capacity(key_len + val_len);
        for k in key {
            tmp.push(*k);
        }
        for v in value {
            tmp.push(*v);
        }
        let check_sum = crc32::checksum_ieee(&tmp);

        let next_byte = SeekFrom::End(0);
        let current_position = bw.seek(SeekFrom::Current(0))?;
        bw.seek(next_byte);

        bw.write_u32::<LittleEndian>(check_sum);
        bw.write_u32::<LittleEndian>(key_len as u32);
        bw.write_u32::<LittleEndian>(val_len as u32);
        bw.write_all(&tmp);
        Ok(current_position)
    }

    pub fn get(&mut self, key: &ByteStr) -> io::Result<Option<ByteString>> {
        let position = match self.index.get(key) {
            Some(position) => *position,
            None => return Ok(None),
        };
        let kv = self.get_at(position)?;
        Ok(Some(kv.value))
    }

    pub fn get_at(&mut self, position: u64) -> io::Result<KeyValuePair> {
        let mut f = BufReader::new(&self.f);
        f.seek(SeekFrom::Start(position))?;
        ActionKV::process_record(&mut f)
    }

    pub fn find(&mut self, target: &ByteStr) -> io::Result<Option<(u64, ByteString)>> {
        let mut f = BufReader::new(&mut self.f);
        let mut found: Option<(u64, ByteString)> = None;

        loop {
            let position = f.seek(SeekFrom::Current(0))?;
            let maybe = ActionKV::process_record(&mut f);
            let kv = match maybe {
                Ok(kv) => kv,
                Err(err) => {
                    match err.kind() {
                        io::ErrorKind::UnexpectedEof => {
                            break;
                        }
                        _ => return Err(err)
                    }
                }
            };
            if kv.key == target {
                found = Some((position, kv.value));
            }
        }
        Ok(found)
    }

    pub fn update(&mut self, key: &ByteStr, value: &ByteStr) -> io::Result<()> {
        self.insert(key, value)
    }

    pub fn delete(&mut self, key: &ByteStr) -> io::Result<()> {
        self.insert(key, b"")
    }

}

pub fn write_numbers_to_file(w: &mut Vec<u8>) -> (u32, i8, f64) {
    let one: u32 = 1;
    let two: i8 = 2;
    let three: f64 = 3.0;
    w.write_u32::<LittleEndian>(one);
    w.write_i8(two);
    w.write_f64::<LittleEndian>(three);
    (one, two, three)
}

pub fn read_numbers_from_file(w: &Vec<u8>) -> (u32, i8, f64) {
    let mut r = Cursor::new(w);

    let one = r.read_u32::<LittleEndian>().unwrap();
    let two = r.read_i8().unwrap();
    let three = r.read_f64::<LittleEndian>().unwrap();
    (one, two, three)
}

fn parity_bit(bytes: &[u8]) -> u8 {
    let mut n_ones: u32 = 0;
    for b in bytes {
        let ones = b.count_ones();
        n_ones += ones;
    }
    (n_ones % 2 == 0) as u8
}
