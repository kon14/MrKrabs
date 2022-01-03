mod scraper;
mod quotes;
mod cli;
mod constants;
mod structs;

extern crate libnotify;

use std::{thread, time};

#[tokio::main]
async fn main() {
    println!("\n");
    let args = cli::get_args();

    quotes::spit_facts();

    let filter = scraper::Filter{
        min_price: args.min_price,
        max_price: args.max_price,
    };

    let start_url = scraper::get_first_page(&args.url).await;
    let period = time::Duration::from_secs(u64::from(args.period)*60);

    #[cfg(feature = "notifications")]
    libnotify::init("Mr.Krabs").unwrap();

    loop {
        // Scrape the website
        let found_products = scraper::scrape(&start_url, &filter).await;

        // Handle found products
        for product in found_products.iter() {
            println!("Product: {}\nPrice: {}\nUrl: {}\n", product.name, product.price, product.url);
        }
        println!("Found {} products", found_products.len());

        // Send desktop notifications [Linux]
        if found_products.len() > 0 {
            #[cfg(feature = "notifications")]
            {
                let notification = libnotify::Notification::new(
                    "Mr.Krabs 💶",
                    Some(format!("Found {} products of interest", found_products.len()).as_str()),
                    None // TODO: Specify icon
                );
                notification.set_urgency(libnotify::Urgency::Critical);
                notification.set_timeout(0);
                notification.show().unwrap();
            }
        }

        // Exit loop or schedule next run
        if args.run_once { break; }
        println!("Next run starting in {} {}", args.period, if args.period == 1 { "minute" } else { "minute"});
        thread::sleep(period);
    }

    // Terminate
    #[cfg(feature = "notifications")]
    libnotify::uninit()
}
