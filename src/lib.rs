extern crate seq_io;
extern crate flate2;

use seq_io::fasta;
use std::collections::HashMap;
use flate2::read::GzDecoder;
use std::io::{Read,BufReader};
use std::fs::File;


pub fn count_constant_sites(filename: &str) -> HashMap<char, u64> {
    let infile = File::open(filename).unwrap();
    let buf_reader = BufReader::new(infile);

    let mut reader =  fasta::Reader::new(if filename.ends_with(".gz") {
        Box::new(GzDecoder::new(buf_reader)) as Box<dyn Read>
    } else {
        Box::new(buf_reader) as Box<dyn Read>
    });
    let mut first_sequence = true;
    let mut sites: Vec<char> = Vec::new();
    while let Some(result) = reader.next() {
        let record = result.expect("Error reading record");
        if first_sequence {
            for line in record.seq_lines() {
                for base in line {
                    sites.push((*base as char).to_ascii_lowercase());
                }
            }
            first_sequence = false;
        } else {
            let mut i = 0;
            for line in record.seq_lines() {
                for base in line {
                    if sites[i] != (*base as char).to_ascii_lowercase() {
                        sites[i] = '-';
                    }
                    i += 1;
                }
            }
        }
    }

    let mut constant_sites: HashMap<char, u64> = HashMap::new();
    constant_sites.insert('a', 0);
    constant_sites.insert('c', 0);
    constant_sites.insert('g', 0);
    constant_sites.insert('t', 0);

    for base in sites {
        if constant_sites.contains_key(&base) {
            constant_sites.insert(base, constant_sites.get(&base).unwrap() + 1);
        }
    }

    return constant_sites;
}

#[cfg(test)]
mod tests {
    use super::count_constant_sites;

    #[test]
    fn test_count_constant_sites() {
        let constant_sites = count_constant_sites("data/input_fasta1.fasta");
        assert_eq!(*constant_sites.get(&'a').unwrap(), 1);
        assert_eq!(*constant_sites.get(&'c').unwrap(), 0);
        assert_eq!(*constant_sites.get(&'g').unwrap(), 0);
        assert_eq!(*constant_sites.get(&'t').unwrap(), 1);
    }
}