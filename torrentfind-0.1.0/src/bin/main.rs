extern crate atty;
extern crate clap;
extern crate colorful;

use clap::{App, Arg};
use atty::Stream;
use colorful::Colorful;

use torrentfind::{
    Result,
    query,
    querymagnet,
};

fn main() {
    if let Err(e) = run() {
        println!("{}", e.to_string().red());
        std::process::exit(1);
    } else {
        std::process::exit(0);
    }
}

fn run() -> Result<()> {
    // Argument parsing
    let matches = App::new("torrentfind")
        .version("1.0")
        .author("Sem Lindhout")
        .about("A cli tool that finds torrents on 1337x.to")
        .arg(Arg::with_name("search")
             .value_name("query")
             .required(true)
             .takes_value(true)
             .help("The term to search for"))
        .arg(Arg::with_name("page")
             .value_name("page")
             .short("p")
             .takes_value(true)
             .help("The page on 1337x to look, default is first."))
        .arg(Arg::with_name("ugly")
             .short("u")
             .help("Print output ugly this will automatically be set when scripting."))
        .arg(Arg::with_name("number")
             .value_name("number")
             .short("n")
             .takes_value(true)
             .help("Amount of results to return, defaults to 10."))
        .arg(Arg::with_name("magnet")
             .short("m")
             .help("Gets the magnet link of the query"))
        .get_matches();

    let search = matches.value_of("search").unwrap();
    let mut page = None;
    if let Some(p) = matches.value_of("page") {
        page = Some(p.trim().parse::<u32>()?);
    }

    // Decide if were going to print pretty or ugly
    let mut ugly = false;
    if !atty::is(Stream::Stdout) | matches.is_present("ugly") {
        ugly = true
    }

    let mut number = 10;
    if let Some(n) = matches.value_of("number") {
        number = n.trim().parse::<usize>()?;
    }

    if matches.is_present("magnet") {
        println!("{}", querymagnet(search)?);
    } else {
        // Quering
        let result = query(search, page, number)?;
        if ugly {
            println!("{}", result);
        } else {
            // TODO pretty printing
            result.pretty();
        }
    }

    Ok(())
}
