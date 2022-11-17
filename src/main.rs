// SLOVNIK_LINES=5 slovnik-fc merde # 5 lines with frech-to-czech translations
fn main() {
    let name: &str = &get_programs_rsuffix();
    let dict = match name {
        "ce" => "encz.en",
        "ec" => "encz.cz",
        "cf" => "frcz.fr",
        "fc" => "frcz.cz",
        "ci" => "itcz.it",
        "ic" => "itcz.cz",
        "cr" => "rucz.ru",
        "rc" => "rucz.cz",
        "cg" => "gecz.ge",
        "gc" => "gecz.cz",
        "cs" => "spcz.sp",
        "sc" => "spcz.cz",
        _ => "encz.en",
    };

    let phrase = get_phrase();
    let url = format!("https://www.slovnik.cz/bin/mld.fpl?vcb={}&dictdir={}&lines={}&js=0", &phrase, &dict, get_n_lines());

    if let Some(html) = get_html(&url) {
        use scraper::*;
        let document = Html::parse_document(&html);
        let l_sel = Selector::parse("div.pair>span.l").unwrap();
        let r_sel = Selector::parse("div.pair>span.r").unwrap();
        let sel = Selector::parse("a").unwrap();

        for (l, r) in document.select(&l_sel).zip(document.select(&r_sel)) {
            println!("\t{} --> {}", get_joined_words(&l, &sel), get_joined_words(&r, &sel));
        }
    } else {
        eprintln!("Couldn't connect to \"{}\"!", &url);
        std::process::exit(1);
    }
}

fn get_programs_rsuffix() -> String {
    std::env::args().next().unwrap().chars().rev().take(2).collect::<String>()
}

fn get_n_lines() -> u32 {
    const DEFAULT: u32 = 50;
    std::env::var("SLOVNIK_LINES")
        .map_or(DEFAULT, |x| x.parse::<u32>().map_or(DEFAULT, |y| std::cmp::min(std::cmp::max(y, 5), DEFAULT)))
}

fn get_phrase() -> String {
    std::env::args().skip(1).collect::<Vec<_>>().join("+")
}

fn get_html(url: &str) -> Option<String> {
    let rsp = reqwest::blocking::get(url).ok()?;
    rsp.text().ok()
}

fn get_joined_words(e: &scraper::ElementRef, sel: &scraper::Selector) -> String {
    e.select(&sel).map(|x| x.inner_html()).collect::<Vec<_>>().join(" ")
}
