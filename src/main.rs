use serde::{Deserialize, Serialize};
use std::env;
use std::fs::File;

#[allow(dead_code)]
#[derive(Deserialize, Serialize)]
struct ProductsResponse {
    products: Vec<Product>,
}

#[derive(Deserialize, Serialize)]
struct Product {
    handle: String,
    title: String,
    vendor: String,
    product_type: String,
    variants: Vec<Variant>,
    images: Vec<Images>,
}
#[derive(Deserialize, Serialize)]
struct Variant {
    id: u64,
    title: String,
    price: String,
    sku: String,
    barcode: Option<String>,
    grams: Option<u32>,
}
#[derive(Deserialize, Serialize)]
struct Images {
    position: u32,
    src: String,
}

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: cargo run -- <https://example.com>");
        return;
    }

    let url_base = &args[1];
    let current_date = chrono::Utc::now();
    let date = current_date.date_naive();
    let file_name = format!("products_{}.csv", date);

    let mut page_number = 1;
    let mut total_products = 0;
    let mut is_first_page = true;

    loop {
        match fetch_page(url_base, page_number).await {
            Ok(response) => {
                if response.products.is_empty() {
                    println!("No more products found. Stopping at page {}", page_number);
                    break;
                }

                println!(
                    "Page {}: Found {} products",
                    page_number,
                    response.products.len()
                );
                total_products += response.products.len();

                // Write to CSV (append mode after first page)
                extract_product_data(response, &file_name, is_first_page)
                    .expect("Failed to write product data to CSV");
                is_first_page = false;
                page_number += 1;
            }
            Err(e) => {
                eprintln!("Error fetching page {}: {}", page_number, e);
                break;
            }
        }
    }

    println!("Extraction completed! Total products: {}", total_products);
}

async fn fetch_page(
    url_base: &str,
    page_number: i32,
) -> Result<ProductsResponse, Box<dyn std::error::Error>> {
    // Your existing client code, but return the response instead of processing it
    let client = reqwest::ClientBuilder::new()
        .user_agent("Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/138.0.0.0 Safari/537.36")
        .build()?;

    let resp = client
        .get(&format!("{}/products.json?limit=250&page={}", url_base, page_number))
        .header("Accept", "text/html,application/xhtml+xml,application/xml;q=0.9,image/avif,image/webp,image/apng,*/*;q=0.8")
        .header("Accept-Language", "en-GB,en;q=0.8")
        .header("Cache-Control", "max-age=0")
        .header("Sec-Ch-Ua", "'Not)A;Brand';v='8', 'Chromium';v='138', 'Brave';v='138'")
        .header("Sec-Ch-Ua-Mobile", "?0")
        .header("Sec-Ch-Ua-Platform", "Linux")
        .header("Sec-Fetch-Dest", "document")
        .header("Sec-Fetch-Mode", "navigate")
        .header("Sec-Fetch-Site", "none")
        .header("Sec-Fetch-User", "?1")
        .header("Sec-Gpc", "1")
        .send()
        .await?
        .json::<ProductsResponse>()
        .await?;

    Ok(resp)
}

fn extract_product_data(
    products_response: ProductsResponse,
    file_name: &str,
    write_headers: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    let file = if write_headers {
        File::create(file_name)?
    } else {
        std::fs::OpenOptions::new().append(true).open(file_name)?
    };

    let mut wtr = csv::Writer::from_writer(file);
    // Write headers
    if write_headers {
        wtr.write_record(&[
            "product_handle",
            "product_title",
            "vendor",
            "product_type",
            "variant_id",
            "variant_title",
            "variant_price",
            "variant_sku",
            "variant_barcode",
            "variant_grams",
            "image_position",
            "image_src",
        ])?;
    }

    for product in &products_response.products {
        let max_items = std::cmp::max(product.variants.len(), product.images.len());
        for i in 0..max_items {
            let variant = product.variants.get(i);
            let image = product.images.get(i);

            let image_pos_str = image
                .map(|img| img.position.to_string())
                .unwrap_or_default();
            let grams_str = variant
                .and_then(|v| v.grams.map(|g| g.to_string()))
                .unwrap_or_default();

            // Then use &grams_str and &image_pos_str in the array

            wtr.write_record(&[
                &product.handle,
                &product.title,
                &product.vendor,
                &product.product_type,
                variant.map(|v| v.id.to_string()).as_deref().unwrap_or(""),
                variant.map(|v| v.title.as_str()).unwrap_or(""),
                variant.map(|v| v.price.as_str()).unwrap_or(""),
                variant.map(|v| v.sku.as_str()).unwrap_or(""),
                variant.and_then(|v| v.barcode.as_deref()).unwrap_or(""),
                &grams_str,
                &image_pos_str,
                image.map(|img| img.src.as_str()).unwrap_or(""),
            ])?;
        }
    }
    wtr.flush()?;

    // Only print "Data written" message if write_headers is true (last page)
    if write_headers {
        println!("Data written to {}", file_name);
    }

    Ok(())
}
