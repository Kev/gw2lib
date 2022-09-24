use crate::{items::ItemId, Endpoint, FixedEndpoint, TimeStamp};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Buy {
    pub id: u64,
    pub item_id: ItemId,
    pub price: u64,
    pub quantity: u64,
    pub created: TimeStamp,
    pub purchased: TimeStamp,
}

pub type Buys = Vec<Buy>;

impl Endpoint for Buys {
    const AUTHENTICATED: bool = true;
    const LOCALE: bool = false;
    const URL: &'static str = "v2/commerce/transactions/history/buys";
    const VERSION: &'static str = "2022-09-22T00:00:00.000Z";
}

impl FixedEndpoint for Buys {}
