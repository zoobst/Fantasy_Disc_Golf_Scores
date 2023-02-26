fn main() {

    let mut month = String::new();
    println!("Beginning Month of Search (01, 02...) :");
    let b1 = std::io::stdin().read_line(&mut month).unwrap();
    let mut url = String::new();
    println!("Name of Event: ");
    let b2 = std::io::stdin().read_line(&mut url).unwrap();

    let mut year = String::new();    
    println!("Beginning Year to Search (2022, 2023...): ");
    let b3 = std::io::stdin().read_line(&mut year).unwrap();
    let mut end_year = String::new();
    println!("End Year: ");
    let b4 = std::io::stdin().read_line(&mut end_year).unwrap();

    let url1 = url.replace(" ", "%20");
    let response = reqwest::blocking::get(
        format!("https://www.pdga.com/tour/search?OfficialName={url1}&date_filter[min][date]={year}-{month}-01&date_filter[max][date]={end_year}-12-31"),
    )
    .unwrap()
    .text()
    .unwrap();

    let document = scraper::Html::parse_document(&response);
    let fragment = Html::parse_fragment(html);
    let a_selector = Selector::parse("a").unwrap();
    let href_selector = Selector::parse("href").unwrap();
    let href = fragment.select(&href_selector).next().unwrap();
    let re = Regex::new("\/t\w*\/e\w*\/\d\d\d\d\d").unwrap();
    for element in href.select(&a_selector) {
        assert_eq!("href", element.value().name());
    }
    println!("{}", response)
}