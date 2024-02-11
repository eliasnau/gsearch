# GSEARCH

GSearch is a Rust crate for interacting with the Google Custom Search API. It allows you to perform searches and retrieve search results in a convenient way.

## Getting Started

### Prerequisites

- Before using GSearch, you'll need the following:
  - Rust installed on your machine. Visit Rust's official website for installation instructions.
    Installation
  - Add GSearch as a dependency in your Cargo.toml file:

### Installation

Add GSearch as a dependency in your Cargo.toml file:

```toml
[dependencies]
gsearch = "0.1.0"
```

## Ussage

```rust
use gsearch::{GSearch, GSearchError};

fn main() {
    let api_key = "YOUR_API_KEY";
    let cx = "YOUR_CX";
    let gsearch = GSearch::new(api_key, cx);

    let query = "lectures";
    match gsearch.search(query) {
        Ok(result) => {
            // Handle successful search result
            println!("{:?}", result);
        }
        Err(error) => {
            // Handle errors
            match error {
                GSearchError::RequestError(req_err) => {
                    eprintln!("Request error: {:?}", req_err);
                }
                GSearchError::ApiResponseError(status_code) => {
                    eprintln!("API response error: {:?}", status_code);
                }
                GSearchError::JsonError(json_err) => {
                    eprintln!("JSON parsing error: {:?}", json_err);
                }
            }
        }
    }
}

```

Replace "YOUR_API_KEY" and "YOUR_CX" with your actual API key and CX values.

## Get Credentials

1. Google Custom Search Engine ID (CX):

   - Go to https://cse.google.com/all.
   - Select your search engine or create a new one.
   - Find the CX ID titled as "Search engine ID."
   - The public URL also has the CX ID in the query parameter as ?cx=\*\*

2. ## Google API Key:
   - Go to the Google Cloud Console.
   - Create a new project or select an existing one.
   - Enable the "Custom Search API" for your project.
   - Create API credentials, and obtain the API key.

#### Remember to keep your API key and CX secure, and avoid hardcoding them directly in your code.

## License

This project is licensed under the MIT License - see the LICENSE.md file for details.
