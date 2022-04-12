use csv::Error;
use std::io;

struct Transaction {
    date: String,
    memo: String,
    amount: f32,
}

fn main() -> Result<(), Error> {

    let mut reader = csv::Reader::from_path("transactions.csv").expect("Can't read file");
    let mut wtr = csv::Writer::from_writer(io::stdout());
    wtr.write_record(&["Date", "Memo", "Amount"])?;

    for record in reader.records() {
        let record = record?;

        let date_col: Vec<&_> = record[0].split(' ').clone().collect();

        let amount: String = record[3].trim().to_string();    
        let mut amount: f32 = amount.parse().unwrap();

        match amount {
            i if i < 0.0 => amount = -i,
            i if i > 0.0 => continue,
            _ => println!("No zero txns")
        }
         
        let txn = Transaction { date: date_col[0].to_string(), memo: record[1].to_string(), amount };
        wtr.serialize((txn.date, txn.memo, txn.amount))?;
    }
    
    wtr.flush()?;


    Ok(())
}
