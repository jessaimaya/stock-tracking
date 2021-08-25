use chrono::{DateTime, FixedOffset, NaiveDate, NaiveDateTime, NaiveTime, TimeZone, Utc};
use clap::{App, Arg};
use yahoo_finance_api as yahoo;
use yahoo_finance_api::Quote;

fn main() {
    let arguments = App::new("Simple Stock Tracking")
        .version("1.0")
        .author("Jessai M. <maya.jessai@gmail.com>")
        .about("Command line application to retrieve the prices for open, high, low and close of yahoo finance api.")
        .arg(
            Arg::with_name("symbols")
                .short("s")
                .long("symbols")
                .required(true)
                .value_name("SYMBOLS")
                .help("Symbols separated by commas to retrieve info like: MSFT, GOOG, AAPL, UBER, IBM")
        )
        .arg(
            Arg::with_name("from")
                .short("f")
                .long("from")
                .required(true)
                .value_name("FROM")
                .allow_hyphen_values(true)
                .help("Datetime [yyyy-mm-dd] start range.")
        )
        .get_matches();

    if let (Some(f), Some(s)) = (arguments.value_of("from"), arguments.value_of("symbols")) {
        let syms: Vec<&str> = s.split(',').collect();

        let to = Utc::now();
        let date =
            NaiveDate::parse_from_str(f, "%Y-%m-%d").expect("Error trying to get today's date.");
        let tz = FixedOffset::west(3600);
        let time = NaiveTime::from_hms(0, 0, 0);
        let date_time = NaiveDateTime::new(date, time);
        let tz_from = tz.from_local_datetime(&date_time).unwrap();
        let tz_from_utc = Utc.from_local_datetime(&tz_from.naive_utc()).unwrap();

        let quote_list: Vec<(Option<Vec<_>>, String)> = syms
            .iter()
            .map(|&sym| fetch_data(&sym, tz_from_utc, to))
            .collect();
        let resume: Vec<String> = quote_list
            .into_iter()
            .filter(|quote| quote.0.is_some())
            .map(|quote| calc_data(quote.0.unwrap(), quote.1))
            .collect();

        print_data(resume);
    }
}

fn fetch_data(
    symbol: &str,
    from: DateTime<Utc>,
    to: DateTime<Utc>,
) -> (Option<Vec<Quote>>, String) {
    let provider = yahoo::YahooConnector::new();
    let response = provider.get_quote_history(symbol, from, to);
    if let Ok(q) = response.expect("We couldn't fetch data").quotes() {
        (Some(q), String::from(symbol))
    } else {
        (None, String::from(symbol))
    }
}

fn calc_data(quotes: Vec<Quote>, sym: String) -> String {
    let last_quote = quotes
        .last()
        .expect("Failed trying to get last quote's element.");
    let last_date = NaiveDateTime::from_timestamp(last_quote.timestamp as i64, 0);
    let last_close = &last_quote.close;

    let adj_closes: Vec<f64> = quotes.iter().map(|q| q.adjclose).collect();

    let max = &adj_closes.iter().fold(0.0 / 0.0, |a, b| b.max(a));
    let min = &adj_closes.iter().fold(0.0 / 0.0, |a, b| b.min(a));

    let average = n_window_sma(30, &adj_closes);

    let diff = price_diff(&adj_closes).expect("Failed processing price difference.");
    let av_res = average.as_ref().unwrap().last().unwrap();

    let res = format!(
        "{},{},${:.2},{:.2}%,${:.2},${:.2},${:.2}",
        last_date, sym, last_close, diff.0, min, max, av_res
    );
    res
}

fn n_window_sma(n: usize, series: &[f64]) -> Option<Vec<f64>> {
    Some(
        series
            .chunks(n)
            .map(|chunk| chunk.iter().sum::<f64>() / n as f64)
            .collect(),
    )
}

fn price_diff(series: &[f64]) -> Option<(f64, f64)> {
    let first = series[0];
    let last = series[series.len() - 1];
    Some(((last / first) * 100.0, (first - last).abs()))
}

fn print_data(data: Vec<String>) {
    let headers = String::from("period start,symbol,price,change %,min,max,30d avg");
    println!("{}", format!("{}\n{}", headers, data.join("\n")));
}
