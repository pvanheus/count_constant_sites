/*
Copyright (c) 2019 Peter van Heusden <pvh@sanbi.ac.za>

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
*/

extern crate clap;
extern crate seq_io;
extern crate flate2;

use clap::{App, Arg};
use seq_io::fasta;
use std::collections::HashMap;
use flate2::read::GzDecoder;
use std::io::{Read,BufReader};
use std::fs::File;

fn main() {
    let matches = App::new("compute constant sites in an alignment")
        .version("0.1")
        .author("Peter van Heusden <pvh@sanbi.ac.za>")
        .arg(Arg::with_name("INPUT")
            .help("Multiple sequence alignment input file (FASTA format)")
            .required(true)
            .index(1)
        ).get_matches();

    let filename = matches.value_of("INPUT").unwrap();
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
//    for result in reader.records() {
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

    println!("{},{},{},{}",
        constant_sites.get(&'a').unwrap(),
        constant_sites.get(&'c').unwrap(),
        constant_sites.get(&'g').unwrap(),
        constant_sites.get(&'t').unwrap());
}
