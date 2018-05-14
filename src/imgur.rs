use reqwest::{
    Client,
    header::{
        Headers,
        Authorization
    }
};
use failure::Error;
use rand::Rng;

use conf::Config;

#[derive(Debug, Clone)]
pub struct Imgur {
    client: Client
}

#[derive(Deserialize, Debug, Clone, PartialEq)]
struct Search {
    pub success: bool,
    pub status: usize,
    pub data: Vec<Album>
}

#[derive(Deserialize, Debug, Clone, PartialEq)]
struct Album {
    pub images: Option<Vec<Image>>,
}

#[derive(Deserialize, Debug, Clone, PartialEq)]
struct Image {
    pub link: String
}

impl Imgur {
    pub fn new(conf: &Config) -> Result<Imgur, Error> {
        let client = {
            let mut headers = Headers::new();
            headers.set(Authorization({
                let pre = "Client-ID ";
                let mut s = String::with_capacity(pre.len() + &conf.imgur_id.len());
                s += pre;
                s += &conf.imgur_id;
                s
            }));
            Client::builder().default_headers(headers).build().map_err(Error::from)
        }?;
        Ok(Imgur {client})
    }

    pub fn get_rand<T, S>(&self, query: T, rand: &mut S) -> Result<String, Error> where T: AsRef<str>, S: Rng {
        let mut search: Search = {
            let mut tmp = self.client.get("https://api.imgur.com/3/gallery/search")
                .query(&[("q", query.as_ref())])
                .send()
                .map_err(Error::from)?;
            tmp.json()?
        };
        let album_count = search.data.len();
        loop {
            let album = &search.data[rand.gen_range(0, album_count)];
            if let Some(ref imgs) = album.images {
                let img_count = imgs.len();
                break Ok(imgs[rand.gen_range(0, img_count)].link.clone());
            }
        }
    }
}