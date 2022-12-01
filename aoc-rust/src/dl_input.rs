use std::env;

use cookie::Cookie;
use cookie_store::CookieStore;
use url::Url;

pub fn download(year_no: usize, day_no: usize) {
    println!("Downloading input...");
    // create dir
    let dirname = format!("input/year{:04}", year_no);
    std::fs::create_dir_all(dirname).unwrap();

    let dl_url = format!("https://adventofcode.com/{}/day/{}/input", year_no, day_no);

    let session_cookie = Cookie::new("session", env::var("AOC_SESSION").unwrap());
    let mut store = CookieStore::default();
    store
        .insert_raw(
            &session_cookie,
            &Url::parse("https://adventofcode.com/").unwrap(),
        )
        .unwrap();

    let agent = ureq::builder().cookie_store(store).build();
    let body = agent
        .get(&dl_url)
        .set("User-Agent", &env::var("USER_AGENT").unwrap())
        .call()
        .unwrap()
        .into_string()
        .unwrap();

    let filename = format!("input/year{:04}/day{:02}.input", year_no, day_no);
    std::fs::write(filename, body).unwrap();
}
