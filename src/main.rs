use clap::{crate_description, crate_name, crate_version, App, Arg};
use solana_sdk::signer::{keypair::Keypair, Signer};
use std::{
    sync::{
        atomic::{AtomicBool, AtomicUsize, Ordering},
        Arc,
    },
    thread::{sleep, spawn},
    time::{Duration, Instant},
};

fn main() {
    let num_cpus_string = num_cpus::get().to_string();
    let matches = App::new(crate_name!())
        .about(crate_description!())
        .version(crate_version!())
        .arg({
            Arg::with_name("threads")
                .help("Number of threads for lookup")
                .short("t")
                .long("threads")
                .takes_value(true)
                .default_value(&num_cpus_string)
        })
        .arg({
            Arg::with_name("ignore_case")
                .help("Ignore case distinctions")
                .short("i")
                .long("ignore-case")
        })
        .arg({
            Arg::with_name("exit")
                .help("Exit on first match")
                .short("e")
                .long("exit")
        })
        .arg({
            Arg::with_name("stat")
                .help("Print genrate stats every X seconds")
                .short("s")
                .long("stat")
                .takes_value(true)
        })
        .arg(
            Arg::with_name("word")
                .help("Filter by starting from word")
                .index(1)
                .required(true),
        )
        .get_matches();

    let threads = matches
        .value_of("threads")
        .unwrap()
        .parse::<usize>()
        .unwrap();
    let ignore_case = matches.is_present("ignore_case");
    let exit = matches.is_present("exit");
    let stat = matches
        .value_of("stat")
        .map(|stat| stat.parse::<u64>().unwrap());
    let mut word = matches.value_of("word").unwrap().to_string();
    if ignore_case {
        word = word.to_ascii_lowercase();
    }

    let exit_flag = Arc::new(AtomicBool::new(false));

    let perf_count = Arc::new(AtomicUsize::new(0));
    let mut perf_ts = Instant::now();

    let mut threads = (0..threads)
        .map(|_| {
            let word = word.clone();
            let exit_flag = Arc::clone(&exit_flag);
            let perf_count = Arc::clone(&perf_count);
            spawn(move || {
                while !exit_flag.load(Ordering::Relaxed) {
                    let chunk = 10;
                    for _ in 0..chunk {
                        if generate(&word, ignore_case) && exit {
                            exit_flag.store(true, Ordering::Relaxed);
                        }
                    }

                    perf_count.fetch_add(chunk, Ordering::AcqRel);
                }
            })
        })
        .collect::<Vec<_>>();
    if let Some(sleep_time) = stat {
        let sleep_time = Duration::from_secs(sleep_time);
        threads.push(spawn(move || loop {
            let sts = Instant::now();
            while sts.elapsed() < sleep_time {
                sleep(Duration::from_millis(50));
                if exit_flag.load(Ordering::Relaxed) {
                    return;
                }
            }

            let elapsed = perf_ts.elapsed().as_micros() as f64;
            let perf_total = perf_count.swap(0, Ordering::AcqRel) as f64;
            perf_ts = Instant::now();

            eprintln!("Genrate: {:.2?} op/s", perf_total * 1_000_000.0 / elapsed);
        }));
    }
    for thread in threads {
        thread.join().unwrap();
    }
}

fn generate(word: &str, ignore_case: bool) -> bool {
    let kp = Keypair::new();
    let pubkey = kp.pubkey().to_string();

    let matched = match ignore_case {
        true => pubkey.to_ascii_lowercase(),
        false => pubkey,
    }
    .starts_with(word);

    if matched {
        println!("{} {}", kp.pubkey().to_string(), kp.to_base58_string());
        true
    } else {
        false
    }
}
