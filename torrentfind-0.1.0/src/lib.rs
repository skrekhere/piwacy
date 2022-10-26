#![deny(missing_docs)]

//! torrentfind scrapes the 1337x.to website for torrents

extern crate reqwest;
extern crate thiserror;
extern crate colorful;
extern crate regex;

use regex::Regex;

pub mod models;
use models::{Results, Magnet};

use error::Error;

/// Result type of this crate
pub type Result<T> = std::result::Result<T, error::Error>;

/// Url of 1337x
pub const URL: &'static str = "https://1337x.to/";

/// Gets the magnet link of a torrent
pub fn querymagnet(name: &str) -> Result<Magnet> {
    let id = getid(&name)?;
    let name = name.replace(" ", "-");
    let url = format!("{}torrent/{}/{}/", URL, id, name);
    let magnetlink = getmagnet(&url)?;
    Ok(Magnet::new(magnetlink))
}

/// Function that queries the site and returns a struct with results
pub fn query(query: &str, page: Option<u32>, number: usize) -> Result<Results> {
    let url = makeurl(query, page);
    let res = makereq(&url)?;
    let mut results = res.into();
    if let Results::Results(_) = results {
        results.truncate(number).unwrap();
        Ok(results)
    } else {
        Err(Error::NoResults)
    }
}

// Get the id of a torrent
pub fn getid(name: &str) -> Result<String> {
    let url = makeurl(name, None);
    let res = makereq(&url)?;
    let re = Regex::new("<a href=\"/torrent/(?P<id>[0-9]*?)/.*?\"").unwrap();
    let id = &re.captures(&res).unwrap()["id"];
    Ok(String::from(id))
}

// Get the magnet link from a torrents webpage
fn getmagnet(url: &str) -> Result<String> {
    let res = makereq(url)?;
    let re = Regex::new("(?P<link>magnet:\\?.*?)\"").unwrap();
    let link = &re.captures(&res).unwrap()["link"];
    Ok(String::from(link))
}

// Make a request
fn makereq(url: &str) -> Result<String> {
    Ok(reqwest::blocking::get(url)?.text()?)
}

fn makeurl(query: &str, page: Option<u32>) -> String {
    if let Some(page) = page {
        format!("{}search/{}/{}/", URL, query, page)
    } else {
        format!("{}search/{}/1/", URL, query)
    }
}

mod error {
    use thiserror::Error;

    #[derive(Error, Debug)]
    pub enum Error {
        #[error("Couldn't query site\n {0}")]
        Internet(#[from] reqwest::Error),
        #[error("Couldn't query site after 5 tries")]
        Timeout,
        #[error("No Results found")]
        NoResults,
        #[error("Couldnt parse Integer")]
        ParseError(#[from] std::num::ParseIntError),
        #[error("Wrong variant")]
        WrongVariant,
    }

}

#[cfg(test)]
mod test {
    use super ::*;
    #[test]
    fn test_req() {
        let res = makereq("example.com/doesntexist");
        if let Err(e) = res {
            if let Error::Internet(_) = e {
            } else {
                panic!("Wrong error code expected Error::Internet got Error::{:?}", e)
            }
        } else {
            // impawsibble
            panic!("The request worked?????");
        }
    }


    fn test_query(search: &str) {
        match query(search, None, 10) {
            Ok(_) => {},
            Err(Error::NoResults) => panic!("no results"),
            Err(Error::Timeout) => panic!("timeout"),
            Err(Error::Internet(_)) => panic!("couldn't query site"),
            Err(Error::ParseError(_)) => unreachable![],
            Err(Error::WrongVariant) => unreachable![],
        }
    }

    #[test]
    fn querytest() {
        test_query("rick and morty");
        test_query("pirates of the");
    }

    #[test]
    fn getid_test() {
        assert_eq!("3036321", getid("Elementary.S06E06.HDTV.x264-LOL[eztv]").unwrap());
        assert_eq!("4112092", getid("Rick-and-Morty-S04E01-1080p-WEBRip-x264-TBS-TGx").unwrap());
        assert_eq!("394184", getid("Indiana-Jones-and-the-Kingdom-of-the-Crystal-Skull-2008-1080p-BrRip-x264-YIFY").unwrap());
    }
}
