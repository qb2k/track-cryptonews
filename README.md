# Track Crypto News — Rust Project

![Rust](https://img.shields.io/badge/Rust-000?logo=rust&logoColor=white)
![Actix-web](https://img.shields.io/badge/Actix--web-25A162?logo=rust&logoColor=white)
![MIT License](https://img.shields.io/badge/License-MIT-blue)
![CoinGecko API](https://img.shields.io/badge/API-CoinGecko-orange)

A lightweight Rust web application that retrieves and displays cryptocurrency information. Enter a coin’s name or ticker symbol to get its details via the CoinGecko API — no API key required.

---

## Technologies Used

- **Language:** Rust  
- **Frameworks & Libraries:**  
  - Actix-web  
  - Reqwest  
  - Serde  
- **API:** [CoinGecko](https://www.coingecko.com/en/api)  
- **Frontend:** Basic HTML search form  
- **Project Structure:**
```bash
track-cryptonews/  
│── src/  
│   │── main.rs          
│   │── api/
│   │   │── coingecko.rs
│   │   │── cryptonews.rs
│   │   │── mod.rs
│   │── handlers/
│   │   │── mod.rs
│   │   │── news_handler.rs
│   │── models/
│   │   │── mod.rs
│   │   │── news.rs
│   │── templates/
│   │   │── index.html
│   │   │── result.html
│   │── utils
│   │   │── cache.rs
│   │   │── mod.rs
│── static/  
│   │── styles.css
│             
│── .env
│── Cargo.toml
│── README.md                
│── LICENSE 
│── .gitignore  
```
---

## Getting Started

1. Install [Rust](https://rustup.rs)  
2. Clone the repository:
   ```bash
   git clone https://github.com/qb2k/track-cryptonews.git
   cd track-cryptonews
   ```
3. Run the application:
   ```bash
   cargo run
   ```
4. Open your browser and go to:
   - Home Page: [http://127.0.0.1:8080/](http://127.0.0.1:8080/)
   - Example Search: [http://127.0.0.1:8080/news/bitcoin](http://127.0.0.1:8080/news/bitcoin)

---

## Example API Response

**GET** `/news/bitcoin`  

**Response:**
```json
[
  {
    "title": "About BITCOIN",
    "source": "CoinGecko",
    "date": "N/A",
    "summary": "...",
    "url": "https://bitcoin.org"
  }
]
```

---

## Screenshots

**Home Page**  
![Home](photo_1_2025-08-06_20-16-34.png)

**Bitcoin Search Result**  
![Bitcoin Search](photo_2_2025-08-06_20-16-34.png)

---

## 📄 License

This project is licensed under the MIT License. See the [LICENSE](./LICENSE) file for details.

---

## 👥 Authors

**Daniil** & **Bauyrzhan**  
Blockchain Tech — April 2025  
