use crate::conf::Config;
use lazy_static::lazy_static;
use rand::seq::IteratorRandom;
use rand::Rng;
use reqwest::blocking::Client;
use reqwest::header::{HeaderMap, HeaderValue, InvalidHeaderValue, AUTHORIZATION};
use reqwest::Url;
use serde::Deserialize;
use thiserror::Error;

macro_rules! build_string {
    () => {String::new()};
    ($($x:expr),+ $(,)?) => {
        {
            let mut len = 0;
            $(len += $x.len();)+
            let mut s = String::with_capacity(len);
            $(s += $x;)+
            s
        }
    }
}

lazy_static! {
    static ref SEARCH_URL: Url = Url::parse("https://api.imgur.com/3/gallery/search/").unwrap();
}

#[derive(Error, Debug)]
pub enum Error {
    #[error("failed initialise client: {0}")]
    ClientInit(#[source] reqwest::Error),

    #[error("bad imgur id: {0}")]
    BadImgurId(#[from] InvalidHeaderValue),

    #[error("failed to send request: {0}")]
    Send(#[source] reqwest::Error),

    #[error("bad response: {0}")]
    BadResponse(#[source] reqwest::Error),

    #[error("no images matched query")]
    NoneFound,
}

pub struct Imgur {
    client: Client,
}

#[derive(Deserialize, Debug)]
struct Search {
    pub success: bool,
    pub status: u16,
    pub data: Vec<Album>,
}

#[derive(Deserialize, Debug)]
struct Album {
    pub images: Option<Vec<Image>>,
}

#[derive(Deserialize, Debug)]
struct Image {
    pub link: String,
}

impl Imgur {
    pub fn new(conf: &Config) -> Result<Self, Error> {
        let client = {
            let mut headers = HeaderMap::with_capacity(1);
            let val = HeaderValue::from_str(&build_string!("Client-ID ", &conf.imgur_id))?;
            headers.insert(AUTHORIZATION, val);
            Client::builder()
                .default_headers(headers)
                .build()
                .map_err(Error::ClientInit)?
        };

        Ok(Imgur { client })
    }

    pub fn get_rand<T, S>(&self, query: T, rand: &mut S) -> Result<String, Error>
    where
        T: AsRef<str>,
        S: Rng,
    {
        let search: Search = self
            .client
            .get(SEARCH_URL.clone())
            .query(&[("q", query.as_ref())])
            .send()
            .map_err(Error::Send)?
            .json()
            .map_err(Error::BadResponse)?;

        search
            .data
            .into_iter()
            .filter_map(|x| x.images)
            .flatten()
            .choose(rand)
            .ok_or_else(|| Error::NoneFound)
            .map(|x| x.link)
    }
}
