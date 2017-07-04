extern crate base64;
extern crate rand;
extern crate ring;
extern crate clap;
extern crate scoped_threadpool;
extern crate num_cpus;

use rand::Rng;
use ring::digest;
use std::str::FromStr;
use clap::{App, Arg};
use scoped_threadpool::Pool;

fn main() {
    let cpus: &str = &*num_cpus::get().to_string();
    let matches = App::new("vanityhash")
        .version("0.2")
        .author("Evan Schwartz <evan@ripple.com>")
        .about("Searches for hashes that match a certain prefix")
        .arg(
            Arg::with_name("include_both_cases")
                .help("Include upper and lowercase letters")
                .long("include_both_cases")
                .alias("cases")
        )
        .arg(
            Arg::with_name("include_symbols")
                .help("Include lookalike characters and numbers")
                .long("include_symbols")
                .alias("symbols")
        )
        .arg(
            Arg::with_name("preimage_prefix")
                .help("The prefix the hash preimage should start with")
                .long("preimage_prefix")
                .value_name("PREIMAGE_PREFIX")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("threads")
                .help("Number of threads to use")
                .long("threads")
                .short("t")
                .value_name("NUM_THREADS")
                .takes_value(true)
                .default_value(cpus),
        )
        .arg(
            Arg::with_name("prefix")
                .help("The prefix the vanity hash should start with")
                .required(true),
        )
        .get_matches();

    let include_both_cases = matches.is_present("include_both_cases");
    let include_symbols = matches.is_present("include_symbols");
    let threads: u32 = FromStr::from_str(matches.value_of("threads").unwrap_or("8")).unwrap();
    let preimage_prefix = matches.value_of("preimage_prefix").unwrap_or("");
    let prefix = matches.value_of("prefix").unwrap_or("");

    let mut pool = Pool::new(threads);
    pool.scoped(|scoped| {
        for _i in 0..threads {
            scoped.execute(move || {
                // TODO count the number of hashes we've done
                let mut rng = rand::thread_rng();
                let mut test_bytes =
                    base64::decode_config(preimage_prefix, base64::URL_SAFE_NO_PAD).unwrap();
                let preimage_prefix_byte_length = test_bytes.len();
                let rand_bytes: Vec<u8> =
                    rng.gen_iter().take(32 - preimage_prefix_byte_length).collect();
                test_bytes.extend(rand_bytes);

                loop {
                    increment(&mut test_bytes);
                    // TODO configurable hash function and encoding
                    let output = base64::encode_config(
                        &digest::digest(&digest::SHA256, &test_bytes),
                        base64::URL_SAFE_NO_PAD,
                    );
                    let matching =
                        matching_characters(&prefix, &output, include_both_cases, include_symbols);
                    if !matching {
                        continue;
                    }
                    println!(
                        "{} {}",
                        output,
                        base64::encode_config(&test_bytes, base64::URL_SAFE_NO_PAD)
                    );
                }
            });
        }
    });

    // Keep the main thread alive
    loop {}
}

fn increment (v: &mut Vec<u8>) {
    for i in (0..32).rev() {
        if v[i] < 255 {
            v[i] += 1;
            return
        } else {
            v[i] = 0;
            continue
        }
    }
}

fn matching_characters(a: &str, b: &str, include_both_cases: bool, include_symbols: bool) -> bool {
    let a = if include_both_cases {
        a.to_lowercase()
    } else {
        a.to_string()
    };
    let b = if include_both_cases {
        b.to_lowercase()
    } else {
        b.to_string()
    };
    let a_chars = a.chars();
    let mut b_chars = b.chars();
    for a_next in a_chars {
        if let Some(b_next) = b_chars.next() {
            if a_next == b_next || (include_symbols && matches_symbol(a_next, b_next)) {
                continue;
            } else {
                return false;
            }
        } else {
            return false;
        }
    }
    return true;
}

fn matches_symbol(a: char, b: char) -> bool {
    match a {
        'a' => b == '^' || b == '4',
        'c' => b == 'k',
        'e' => b == '3',
        'g' => b == '6',
        'i' => b == '1' || b == 'y' || b == 'l' || b == 'j',
        'o' => b == '0',
        's' => b == '5' || b == '2' || b == 'z',
        't' => b == '+' || b == '-' || b == '7',
        _ => false,
    }
}
