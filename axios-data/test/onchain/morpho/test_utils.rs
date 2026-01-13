// Known good test data
const TEST_MARKET_KEY: &str =
    "0xb323495f7e4148be5643a4ea4a8221eef163e4bccfdedc2a6f4696baacbc86cc";
const TEST_VAULT_ADDRESS: &str = "0xBEEF01735c132Ada46AA9aA4c54623cAA92A64CB";
const TEST_WSTETH: &str = "0x7f39C581F595B53c5cb19bD0b3f8dA6c935E2Ca0";
const CHAIN_ETHEREUM: i32 = 1;

pub fn get_week_ago_timestamps() -> (i64, i64) {
    let now = chrono::Utc::now().timestamp();
    let week_ago = now - 7 * 24 * 60 * 60;
    (week_ago, now)
}
