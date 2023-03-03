use reqwest::blocking;
use scraper::{Html, Selector};
use polars::prelude::*;
use std::collections::HashMap;

//figure out how to pass a Vec or recursively pass vals from a Vec

fn main() ->  Result<DataFrame, PolarsError> {
    let mut url = String::new();
    println!("Name of Event: ");
    let b2 = std::io::stdin().read_line(&mut url).unwrap();
    let mut month = String::new();
    println!("Beginning Month of Search (01, 02...) :");
    let b1 = std::io::stdin().read_line(&mut month).unwrap();

    let mut year = String::new();    
    println!("Beginning Year to Search (2022, 2023...): ");
    let b3 = std::io::stdin().read_line(&mut year).unwrap();
    let mut end_year = String::new();
    println!("End Year: ");
    let b4 = std::io::stdin().read_line(&mut end_year).unwrap();

    let url1 = url.replace(" ", "%20");
    let url2 = format!("https://www.pdga.com/tour/search?OfficialName={url1}&date_filter[min][date]={year}-{month}-01&date_filter[max][date]={end_year}-12-31");
    let response = reqwest::blocking::get(url2).expect("Could not load url.");
    let body = response.text().unwrap();
    //print!("{}", body);

    let document = Html::parse_document(&body);
    let fragment = Html::parse_fragment(&body);
    let tbody_selector = Selector::parse("tbody").unwrap();
    let link_selector = Selector::parse("td a").unwrap();
    let mut listy = Vec::new();
    let mut link_title_list = Vec::new();

    for element in document.select(&tbody_selector) {
        let link_element = element.select(&link_selector).next().expect("Could not find book link element.");
        let link = link_element.value().attr("href").expect("Could not find href attribute");
        let title_element = link_element.inner_html().to_string();
        //println!("{:?}", link);
        link_title_list.push(title_element);
        listy.push(link);
    }

    let mut listy2 = Vec::new();
    for x in listy {
        let y = format!("https://www.pdga.com{x}");
        listy2.push(y);
    }

    let mut hm = HashMap::new();
    for i in 0..(listy2.len() + 1) {
        hm.insert(&link_title_list[i], &listy2[i]);
    }

    for x in listy2 {
        let scores_response = reqwest::blocking::get(x).expect("Could not load url.");
        let scores_body = scores_response.text().unwrap();
        let scores_document = Html::parse_document(&scores_body);
        let scores_fragment = Html::parse_fragment(&scores_body);
        let tbody_selector = Selector::parse("tbody").unwrap();
        let row_elements_selector = Selector::parse("tbody>tr").unwrap();
        let row_element_data_selector = Selector::parse("td, th").unwrap();
        let all_tables = scores_fragment.select(&tbody_selector);
        let row_elements = table.select(&row_elements_selector);
        let mut rows:Vec<Vec<String>> = Vec::new();
        for table in all_tables{
            for row_element in row_elements{
                let mut row = Vec::new();
                for td_element in row_element.select(&row_element_data_selector){
                    let mut element = td_element.text().collect::<Vec<_>>().join(" ");
                    element = element.trim().replace("\n", " ");
                    row.push(element);
                    }
                rows.push(row)        
                }
            pub fn from_vec(
                name: &str,
                v: Vec<<T as PolarsNumericType>::Native, Global>
            ) -> ChunkedArray<T>

        let df: PolarsResult<DataFrame> = DataFrame::new(vec![rows.from_vec()])
        }
}
df
}
    