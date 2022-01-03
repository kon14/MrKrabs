extern crate reqwest;
extern crate scraper;
extern crate selectors;

use scraper::{Html, Selector};
use selectors::attr::CaseSensitivity;
use crate::constants::DOMAIN_NAME;
use crate::structs::Product;

pub struct Filter {
  // TODO: Validate >= 0.0
  pub min_price: f64,
  pub max_price: f64,
}

pub async fn get_first_page(orig_url: &str) -> &str {
  // TODO: Find page #1
  orig_url
}

pub async fn scrape(start_url: &str, filter: &Filter) -> Vec<Product> {
  let client = reqwest::Client::new();
  let initial_page = get_page_body(&client, &start_url).await;
  let mut found_products: Vec<Product> = Vec::new();

  println!("Filtering products...",);
  let mut page_html = initial_page;
  loop {
      found_products.append(&mut process_page_products(&page_html, filter));

      // Set page_hmtl to the next page
      let next = get_next_page_url(&page_html);
      if next.is_some() {
        page_html = get_page_body(&client, &next.unwrap()).await;
      } else {
          break;
      }
  }

  found_products
}

async fn get_page_body(client: &reqwest::Client, url: &str) -> Html {
  let res = client.get(url).send().await.unwrap();
  assert!(res.status().is_success());
  Html::parse_document(&res.text().await.unwrap())
}

fn process_page_products(page_html: &Html, filter: &Filter) -> Vec<Product> {
  let product_li_div_sel = Selector::parse("ol#sku-list > li.cf.card > div.card-content").unwrap();
  let mut found_products: Vec<Product> = Vec::new();

  // Find products matching user's criteria
  for product_div in page_html.select(&product_li_div_sel) {
      let mut product: Product = Product::default();
      for child in product_div.children() {
          let child_elem = child.value().as_element().unwrap();
          if child_elem.name() == "h2" {
              product.name = child.first_child().unwrap().value().as_element().unwrap().attr("title").unwrap().to_string();
              product.url = String::from(DOMAIN_NAME) + &child.first_child().unwrap().value().as_element().unwrap().attr("href").unwrap().to_string();
          } else if child_elem.name() == "div" && child_elem.has_class("price", CaseSensitivity::AsciiCaseInsensitive) {
              for inner_price_div_child_div in child.first_child().unwrap().children() { // children: <strike>, <a>, <span>
                  if inner_price_div_child_div.value().as_element().is_some() &&
                     inner_price_div_child_div.value().as_element().unwrap().name() == "a" {
                      // Found the <a> tag, containing the price. Possible formats: <a>500 €</a> || <a><span>από</span>500 €</a>
                      product.price = inner_price_div_child_div.last_child().unwrap().value().as_text().unwrap().to_string().split_whitespace().next().unwrap()
                        .trim().replace('.', "").replace(',', ".").parse().unwrap();
                  }
              }
          }
      }
      // TODO: Handle processing in a filter function
      if product.price >= filter.min_price && product.price <= filter.max_price {
        found_products.push(product);
      }
  }

  found_products
}

fn get_next_page_url(page_html: &Html) -> Option<String> {
  let paginator_li_sel = Selector::parse(".list-controls > ol.react-component.paginator.cf > li").unwrap();
  let mut curr_page_url = String::from(DOMAIN_NAME);
  let mut next_page_url = String::from(DOMAIN_NAME);
  let mut found_curr_page_url = false;
  let mut ol_lis: Vec<String> = Vec::new();
  for page_li in page_html.select(&paginator_li_sel) {
      // Find current page
      if !found_curr_page_url && page_li.value().has_class("current_page", CaseSensitivity::AsciiCaseInsensitive) {
          curr_page_url += page_li.first_child().unwrap().value().as_element().unwrap().attr("href").unwrap();
          found_curr_page_url = true;
      }
      // Find next page
      let page_li_href = page_li.first_child().unwrap().value().as_element().unwrap().attr("href");
      if page_li_href.is_some() { // child could be <span>
          ol_lis.push(page_li_href.unwrap().to_string());
      }

  }
  if ol_lis.last().is_some() {
    next_page_url += ol_lis.last().unwrap();
  }
  if next_page_url != curr_page_url {
      return Some(next_page_url);
  } else {
      return None;
  }
}
