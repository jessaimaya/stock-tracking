# How to use

Here is an example of how to run the program:
```bash
cargo run -- --symbols AAPL,IBM,UBER -f 2021-01-01
```
After some time, you should get something like this:
``` 
period start,symbol,price,change %,min,max,30d avg
2021-08-24 13:30:00,AAPL,$149.62,116.16%,$115.99,$151.12,$59.25
2021-08-24 13:30:00,IBM,$139.84,116.95%,$114.41,$149.56,$56.44
2021-08-24 13:30:00,UBER,$40.17,78.55%,$39.86,$63.18,$16.64
```