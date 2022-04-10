use indexmap::IndexMap;
use itertools::Itertools;
use rusqlite::{params, Connection};
use rustc_hash::FxHasher;
use std::error::Error;
use std::fs::File;
use std::hash::BuildHasherDefault;
use std::io::{self, BufRead, BufReader};
use std::path::Path;
use std::{env, process};
use tempfile::NamedTempFile;
use unicode_segmentation::UnicodeSegmentation;

type FnvIndexMap<K, V> = IndexMap<K, V, BuildHasherDefault<FxHasher>>;

fn find_unigram_frequencies<P: AsRef<Path>>(path: P) -> io::Result<FnvIndexMap<String, usize>> {
    let reader = BufReader::new(File::open(path)?);
    let mut map = FnvIndexMap::default();

    for line in reader.lines() {
        for word in line?.unicode_words() {
            match map.get_mut(word) {
                Some(count) => {
                    *count += 1;
                }
                None => {
                    map.insert(word.to_owned(), 1usize);
                }
            }
        }
    }

    Ok(map)
}

fn find_bigram_frequencies<P: AsRef<Path>>(path: P) -> io::Result<FnvIndexMap<String, usize>> {
    let reader = BufReader::new(File::open(path)?);
    let mut map = FnvIndexMap::default();

    for line in reader.lines() {
        for (word1, word2) in line?.unicode_words().tuple_windows() {
            let bigram = format!("{} {}", word1, word2);
            *map.entry(bigram).or_insert(0usize) += 1usize;
        }
    }

    Ok(map)
}

fn find_trigram_frequencies<P: AsRef<Path>>(path: P) -> io::Result<FnvIndexMap<String, usize>> {
    let reader = BufReader::new(File::open(path)?);
    let mut map = FnvIndexMap::default();

    for line in reader.lines() {
        for (word1, word2, word3) in line?.unicode_words().tuple_windows() {
            let trigram = format!("{} {} {}", word1, word2, word3);
            *map.entry(trigram).or_insert(0usize) += 1usize;
        }
    }
    Ok(map)
}

fn _find_trigram_frequencies_sqlite<P: AsRef<Path>>(
    path: P,
) -> Result<FnvIndexMap<String, usize>, Box<dyn Error>> {
    let reader = BufReader::new(File::open(path)?);
    let dbpath = NamedTempFile::new()?;
    let mut map = FnvIndexMap::default();
    let mut db = Connection::open(dbpath.path())?;
    println!("dbfile path: {}", dbpath.path().display());

    db.execute(
        "CREATE TABLE trigrams (tg TEXT PRIMARY KEY, count INTEGER)",
        [],
    )?;
    db.pragma_update(None, "synchronous", "OFF")?;
    db.pragma_update(None, "journal_mode", "OFF")?;
    db.pragma_update(None, "page_size", "65536")?;
    {
        for (i, line) in reader.lines().enumerate() {
            for (word1, word2, word3) in line?.unicode_words().tuple_windows() {
                let trigram = format!("{} {} {}", word1, word2, word3);
                *map.entry(trigram).or_insert(0usize) += 1usize;
                //db.execute(
                //    "INSERT INTO trigrams(tg, count) VALUES (?, ?)
                //    ON CONFLICT(tg) DO UPDATE SET count = count + 1;",
                //)?;
            }
            if map.len() >= 1_000_000 {
                //if i % 1000000 == 0 {
                let tx = db.transaction()?;
                println!("processed {i} lines till now");
                {
                    let mut stmt = tx.prepare(
                        "INSERT INTO trigrams(tg, count) VALUES (?, ?)
                ON CONFLICT(tg) DO UPDATE SET count = count + ?;",
                    )?;
                    for (trigram, count) in map.drain(..) {
                        stmt.execute(params![trigram, count, count])?;
                    }
                }
                tx.commit()?;
                println!("done inserts");
            }
        }
    }
    let tx = db.transaction()?;
    println!("final commit");
    {
        let mut stmt = tx.prepare(
            "INSERT INTO trigrams(tg, count) VALUES (?, ?)
                ON CONFLICT(tg) DO UPDATE SET count = count + ?;",
        )?;
        for (trigram, count) in map.drain(..) {
            stmt.execute(params![trigram, count, count])?;
        }
    }
    tx.commit()?;
    println!("done inserts");
    let mut stmt = db.prepare("SELECT tg, count FROM trigrams ORDER BY count DESC LIMIT 100")?;
    let map = stmt
        .query_map([], |row| Ok((row.get(0)?, row.get(1)?)))?
        .collect::<Result<_, _>>()?;
    Ok(map)
}

fn usage(msg: Option<&str>) -> ! {
    if let Some(msg) = msg {
        eprintln!("{}", msg);
    }
    eprintln!("usage: {} [FILE] [1|2|3]", env!("CARGO_BIN_NAME"));
    process::exit(1);
}

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<_> = env::args().collect();
    if args.len() != 3 {
        usage(None);
    }
    let mut map = match args[2].as_str() {
        "1" => find_unigram_frequencies(&args[1]),
        "2" => find_bigram_frequencies(&args[1]),
        "3" => find_trigram_frequencies(&args[1]),
        _ => usage(Some("invalid value of n for n-gram")),
    }?;
    //let mut map = find_trigram_frequencies_sqlite(file)?;
    map.sort_unstable_by(|_, v1, _, v2| v2.cmp(v1));
    for (k, v) in map.iter().take(100) {
        println!("{k}|{v}");
    }
    Ok(())
}
