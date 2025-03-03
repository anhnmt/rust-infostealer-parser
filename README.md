# rust-infostealer-parser

Detects and parses infostealer.

## Installation

```toml
[dependencies]
rust-infostealer-parser = { git = "https://github.com/anhnmt/rust-infostealer-parser", tag = "0.1.0" }
```

## Supports

- Group Meta
  - META

    ![META.png](docs/images/META.png)
  
  - REDLINE
  
    ![REDLINE.png](docs/images/REDLINE.png)
  
  - BRADMAX
  
    ![BRADMAX.png](docs/images/BRADMAX.png)
  
  - MANTICORE
  
    ![MANTICORE.png](docs/images/MANTICORE.png)

- Group Unknown
  - Stealers that we cannot identify.

## References

- https://github.com/anhnmt/go-infostealer-parser
- https://github.com/milxss/universal_stealer_log_parser
- https://github.com/lexfo/stealer-parser