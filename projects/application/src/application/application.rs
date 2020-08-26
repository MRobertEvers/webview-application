use rusqlite::*;

#[derive(Debug)]
struct Person {
    name: String,
}

pub fn read_card(card_name: &str) -> Result<()> {
    let conn = rusqlite::Connection::open(
        "C:\\Users\\Matthew\\Documents\\GitHub\\webview_application\\deps\\AllPrintings.sqlite",
    )?;

    let mut stmt =
        conn.prepare("SELECT name FROM cards WHERE uuid='b8a68840-4044-52c0-a14e-0a1c630ba42c'")?;

    let person_iter = stmt.query_map(params![], |row| Ok(Person { name: row.get(0)? }))?;

    for person in person_iter {
        println!("Found person {:?}", person.unwrap());
    }
    Ok(())
}
