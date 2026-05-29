#[derive(Clone)]
#[contracttype]
pub enum DataKey {
    Budget(BytesN<32>),
}

let budget = Budget {
    owner,
    limit,
    spent: 0,
    category,
};