use std::fmt;
use html_extractor::{html_extractor, HtmlExtractor};
use colorful::Colorful;
use term_table::{Table, TableStyle};
use term_table::table_cell::{Alignment, TableCell};
use term_table::row::Row;

use crate::Result;
use crate::error::Error;

type Html = String;

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum Results {
    Results(Vec<Torrent>),
    NoResults,
}

impl Results {
    pub fn truncate(&mut self, number: usize) -> Result<()> {
        if let Results::Results(r) = self {
            r.truncate(number);
            Ok(())
        } else {
            Err(Error::WrongVariant)
        }
    }

    pub fn pretty(&self) {
        if let Results::Results(results) = self {
            let mut table = Table::new();
            table.style = TableStyle::simple();
            table.add_row(Row::new(vec![
                TableCell::new_with_alignment(format!("name"), 1, Alignment::Center),
                TableCell::new_with_alignment(format!("{} {} ", "seeds".clone().green(), " ".white()), 1, Alignment::Center),
                TableCell::new_with_alignment(format!("{} {} ", "leeches".clone().red(), " ".white()), 1, Alignment::Center),
                TableCell::new_with_alignment(format!("size"), 2, Alignment::Center),
                TableCell::new_with_alignment(format!("time"), 1, Alignment::Center),
            ]));
            for torrent in results {
                table.add_row(Row::new(vec![
                    TableCell::new_with_alignment(format!("{}", torrent.name), 1, Alignment::Center),
                    TableCell::new_with_alignment(format!("{}{}", torrent.seeds.clone().green(), " ".white()), 1, Alignment::Center),
                    TableCell::new_with_alignment(format!("{}{}", torrent.leeches.clone().red(), " ".white()), 1, Alignment::Center),
                    TableCell::new_with_alignment(format!("{}", torrent.size.clone().white()), 2, Alignment::Center),
                    TableCell::new_with_alignment(format!("{}", torrent.time), 1, Alignment::Center),
                ]));
            }
            println!("{}", table.render());
        } else {
            println!("{}", "No results".red().bold());
        }
    }
}

impl From<Vec<Torrent>> for Results {
    fn from(res: Vec<Torrent>) -> Self {
        Results::Results(res)
    }
}

impl From<Html> for Results {
    fn from(html: Html) -> Self {
        let torrenthtml = TorrentHtml::extract_from_str(&html)
            .expect("Couldn't parse html");
        let mut torrents = Vec::new();

        // make sure each one has the correct lenght
        let mut length = torrenthtml.names.len();
        let seeds = torrenthtml.seeds.len();
        let leeches = torrenthtml.leeches.len();
        let size = torrenthtml.size.len();
        let time = torrenthtml.time.len();
//      eprintln!("names: {}", length);
//      eprintln!("seeds: {}", seeds);
//      eprintln!("leeches: {}", leeches);
//      eprintln!("size: {}", size);
//      eprintln!("time: {}", time);
        if length > seeds {
            length = seeds;
        }
        if length > leeches {
            length = leeches;
        }
        if length > size {
            length = size;
        }
        if length > time {
            length = time;
        }

        if length == 0 {
            return Results::NoResults
        }

        for i in 1..length-1 {
            // TODO fix this clone bullshit
            torrents.push(Torrent::new(
                    torrenthtml.names[i].clone(), torrenthtml.seeds[i].clone(),
                    torrenthtml.leeches[i].clone(), torrenthtml.size[i].clone(),
                    torrenthtml.time[i].clone()
                    ))
        }
//      eprintln!("finished converting");
        torrents.into()
    }
}

impl fmt::Display for Results {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Results::Results(res) = self {
            let mut out = String::new();
            for torrent in res {
                out.push_str(&torrent.to_string());
                out.push('\n')
            }
            write!(f, "{}", out)
        } else {
            write!(f, "No Results")
        }
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Torrent {
    pub name: String,
    pub seeds: String,
    pub leeches: String,
    pub size: String,
    pub time: String,
}

impl Torrent {
    fn new(name: String, seeds: String, leeches: String, size: String, time: String) -> Torrent {
        Torrent { name, seeds, leeches, size, time }
    }
}

impl fmt::Display for Torrent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}\t{}\t{}\t{}\t{}",
               self.name, self.size,
               self.seeds, self.leeches,
               self.time)
    }
}

html_extractor! {
    #[derive(Debug, Clone)]
    TorrentHtml {
        names: Vec<String> = (text of ".name", collect),
        seeds: Vec<String> = (text of ".seeds", collect),
        leeches: Vec<String> = (text of ".leeches", collect),
        size: Vec<String> = (text of ".size", collect),
        time: Vec<String> = (text of ".coll-date", collect),
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Magnet {
    pub link: String,
}

impl Magnet {
    pub fn new(link: String) -> Magnet {
        Magnet { link: link }
    }
}

impl fmt::Display for Magnet {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.link)
    }
}


#[cfg(test)]
mod test {
    use super ::*;

    fn testtorrent() -> Torrent {
        Torrent::new("hoi".to_string(), "hoi".to_string(), "hoi".to_string(), "hoi".to_string(), "hoi".to_string())
    }

    #[test]
    fn testfromvec() {
        let vec = vec![testtorrent(), testtorrent()];
        if let Results::Results(results) = vec.clone().into() {
            assert_eq!(results, vec);
        } else {
            panic!("No results");
        }
    }
}
