# Rust Shopify Scraper

A fast and efficient web scraper built in Rust for extracting product data from Shopify stores. This tool bypasses common anti-bot protections and exports data to CSV format with proper handling of nested product variants and images.

## Features

- 🚀 **Fast & Efficient**: Built with Rust and async/await for high performance
- 🛡️ **Anti-Bot Protection**: Bypasses Cloudflare and other common protections
- 📊 **CSV Export**: Exports data in a flattened CSV format for easy analysis
- 🔄 **Automatic Pagination**: Automatically fetches all pages until completion
- 🎯 **Smart Data Handling**: Properly handles nested variants and images
- 📅 **Timestamped Files**: Outputs files with current date in filename

## Installation

### Prerequisites

- Rust (latest stable version)
- Cargo (comes with Rust)

### Setup

1. Clone the repository:

```bash
git clone https://github.com/pranavp1507/rust-shopify-product-scraper
cd rust-shopify-scraper
```

2. Build the project:

```bash
cargo build --release
```

## Usage

### Basic Usage - Example

```bash
cargo run -- https://example-store.com
```

This will:

1. Scrape all products from the specified Shopify store
2. Handle pagination automatically
3. Export data to `products_YYYY-MM-DD.csv`

### Output Format

The CSV file contains the following columns:

| Column          | Description                      |
| --------------- | -------------------------------- |
| product_handle  | Unique product identifier        |
| product_title   | Product name                     |
| vendor          | Product vendor/brand             |
| product_type    | Category of the product          |
| variant_id      | Unique variant identifier        |
| variant_title   | Variant name (size, color, etc.) |
| variant_price   | Price of the variant             |
| variant_sku     | SKU code                         |
| variant_barcode | Barcode (if available)           |
| variant_grams   | Weight in grams                  |
| image_position  | Image order position             |
| image_src       | Image URL                        |

### Data Structure

The scraper handles the relationship between products, variants, and images intelligently:

- Each product can have multiple variants (different sizes, colors, etc.)
- Each product can have multiple images
- The CSV uses a flattened structure where each row represents the maximum of variants or images per product
- Empty cells are used when a product has fewer variants than images or vice versa

## Technical Details

### Dependencies

- `tokio` - Async runtime
- `reqwest` - HTTP client with anti-bot headers
- `serde` - JSON serialization/deserialization
- `csv` - CSV file writing
- `chrono` - Date/time handling

### Anti-Bot Features

The scraper includes several techniques to avoid detection:

- Browser-like User-Agent strings
- Comprehensive HTTP headers mimicking real browsers
- Proper handling of compressed responses
- Respectful request timing

### Error Handling

- Graceful handling of network errors
- Automatic retry logic for failed requests
- Proper error logging and reporting
- Safe handling of malformed JSON responses

## Examples

### Successful Run Output

```bash
Page 1: Found 250 products
Page 2: Found 250 products
Page 3: Found 250 products
...
Page 23: Found 250 products
No more products found. Stopping at page 24
Extraction completed! Total products: 5750
Final data written to products_2025-07-27.csv
```

### File Output

The generated CSV file will contain flattened product data:

```csv
product_handle,product_title,vendor,product_type,variant_id,variant_title,variant_price,variant_sku,variant_barcode,variant_grams,image_position,image_src
led-bulb-a19,LED Bulb A19,Acme Lighting,Lighting,12345,10W Warm White,15.99,LED-A19-10W,,100,1,https://example.com/image1.jpg
led-bulb-a19,LED Bulb A19,Acme Lighting,Lighting,12346,10W Cool White,15.99,LED-A19-10C,,100,2,https://example.com/image2.jpg
```

## Contributing

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add some amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## Legal Notice

⚠️ **Important**: This tool is for educational and research purposes only. Always ensure you have permission to scrape websites and comply with:

- The website's `robots.txt` file
- Terms of Service of the target website
- Local and international laws regarding web scraping
- Rate limiting to avoid overloading servers

The authors are not responsible for any misuse of this tool.

## Troubleshooting

### Common Issues

### 1. "Access Denied" or Cloudflare errors

- The target site may have updated their anti-bot protection
- Try updating the User-Agent string or other headers in the code
- Some sites may require additional headers

### 2. "No products found" on page 1\*\*

- Verify the store URL is correct
- Check if the store uses a different API endpoint
- Some stores may have additional authentication

### 3. Compilation errors\*\*

- Ensure you have the latest stable Rust version
- Run `cargo clean` and `cargo build` again

### Performance Tips

- For large stores (10,000+ products), consider adding delays between requests
- Monitor your system resources during large scraping operations
- Use SSD storage for better CSV writing performance

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Acknowledgments

- Built with ❤️ using Rust
- Inspired by the need for efficient e-commerce data analysis
- Thanks to the Rust community for excellent crates and documentation
