use rusqlite::{Connection, Result,params};
use csv::Reader;

pub fn convert_csv_to_sql(dataset: &str) -> Result<String> {
    let conn = Connection::open("lucy.db")?;

    conn.execute("DROP TABLE IF EXISTS grades", [])?;
    conn.execute(
        "CREATE TABLE grades (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            grade REAL
        )",
        [],
    )?;

    let mut rdr = Reader::from_path(dataset).expect("Failed to read dataset");
    let mut st = conn.prepare(
        "INSERT INTO grades (
            grade
        ) 
        VALUES (?)",
    )?;

    for result in rdr.records() {
        match result {
            Ok(record) => {
                st.execute(&[&record[0]])?;
            }
            Err(err) => {
                eprintln!("Error reading CSV: {:?}", err);
            }
        }
    }

    Ok("lucy".to_string())
}

pub fn query_crud(query: &str) -> Result<()> {
    let conn = Connection::open("lucy.db")?;
    // Read operation
    if query.trim().to_lowercase().starts_with("select") {
        let mut stmt = conn.prepare(query)?;

        let results = stmt.query_map(params![], |row| {
            Ok((
                row.get::<usize, i64>(0)?, 
                row.get::<usize, f64>(1)?, 
            ))
        })?;

        for result in results {
            match result {
                Ok((
                    id,
                    grade,
                )) => {
                    println!(
                        "Result: id={}, grade={}",
                        id,
                        grade,
                    );
                }
                Err(e) => eprintln!("Error in row: {:?}", e),
            }
        }
    } else {
        // other CUD operations
        let _num_affected_rows = conn.execute_batch(query)?;
    }
    Ok(())
}