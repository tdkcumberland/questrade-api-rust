use polars_core::prelude::*;
use polars_io::prelude::*;
use std::io::Cursor;
use std::fs::File;
use serde_json::{Result, Value};

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let basic_json = r#"{"symbol": "THI.TO","symbolId": 38738,"openQuantity": 100,"currentMarketValue": 6017,"currentPrice": 60.17,"averageEntryPrice": 60.23,"closedPnl": 0,"openPnl": -6,"totalCost": false,"isRealTime": "Individual","isUnderReorg": false}"#;
                let file = Cursor::new(basic_json);
                let df = JsonReader::new(file)
                    .infer_schema_len(Some(3))
                    .with_json_format(JsonFormat::JsonLines)
                    .with_batch_size(3)
                    .finish()
                    .unwrap();

        println!("{:?}", df);
    }

    #[test]
    fn json_deserialize() {
        let basic_json = r#"{
            "positions": [
                  {
                       "symbol": "THI.TO",
                       "symbolId": 38738,
                       "openQuantity": 100,
                       "currentMarketValue": 6017,
                       "currentPrice": 60.17,
                       "averageEntryPrice": 60.23,
                       "closedPnl": 0,
                       "openPnl": -6,
                       "totalCost": false,
                       "isRealTime": "Individual",
                       "isUnderReorg": false
                  }
             ]
        }"#;
        let v: Value = serde_json::from_str(basic_json).unwrap();
        let v_array = v["positions"].as_array().unwrap();
        for ele in v_array {
            print!("{}",ele.to_string())
        }
    }

    #[test]
    fn parquet() {
        let r = File::open("../../libs/polars/src/sample.parquet").unwrap();
        let reader = ParquetReader::new(r);
        let df = reader.finish().unwrap();
        println!("{:?}", df);

    }
}
