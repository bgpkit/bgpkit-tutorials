use bgpkit_broker::{BgpkitBroker, BrokerItem, QueryParams};
use bgpkit_parser::BgpkitParser;
use rayon::prelude::*;

fn main() {
    let broker = BgpkitBroker::new_with_params(
        "https://api.broker.bgpkit.com/v1",
        QueryParams {
            start_ts: Some(1640995200),
            end_ts: Some(1640998799),
            project: Some("route-views".to_string()),
            data_type: Some("update".to_string()),
            ..Default::default()
        });

    let items = broker.into_iter().collect::<Vec<BrokerItem>>();

    let sum: usize = items.par_iter().map(|item| {
        println!("processing {}...", &item.url);
        let parser = BgpkitParser::new(&item.url).unwrap();
        let count = parser.into_record_iter().count();
        count
    }).sum();

    println!("total number of records is {}", sum);
}

