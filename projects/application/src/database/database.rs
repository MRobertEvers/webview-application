use rusqlite::*;

#[derive(Debug, Clone)]
pub struct Card {
    pub name: String,
    pub uuid: String,
    pub scryfall_id: String,
}

pub fn read_card_by_uuid(card_uuid: &str) -> Result<Card> {
    let conn = rusqlite::Connection::open(
        "C:\\Users\\Matthew\\Documents\\GitHub\\webview_application\\deps\\AllPrintings.sqlite",
    )?;

    let q = std::fmt::format(format_args!(
        "SELECT name, uuid, scryfallId  FROM cards WHERE uuid='{}'",
        card_uuid
    ));
    let mut result = conn.prepare(&q)?;

    let mut iter = result.query_map(params![], |row| {
        Ok(Card {
            name: row.get(0)?,
            uuid: row.get(1)?,
            scryfall_id: row.get(2)?,
        })
    })?;

    let first_result = iter.nth(0);
    match first_result {
        Some(card) => Ok(card.unwrap().clone()),
        None => Err(rusqlite::Error::QueryReturnedNoRows),
    }
}
