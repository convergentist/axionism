pub mod queries {
    // ------------------------------------------------------------------------
    // TABLE 2: Market State (Current)
    // ------------------------------------------------------------------------
    pub const MARKET_STATE: &str = r#"
query MarketState($chainIds: [Int!], $first: Int!, $skip: Int!) {
  markets(
    first: $first
    skip: $skip
    orderBy: SupplyAssetsUsd
    orderDirection: Desc
    where: { chainId_in: $chainIds }
  ) {
    items {
      uniqueKey
      lltv
      oracleAddress
      irmAddress
      loanAsset { address symbol decimals }
      collateralAsset { address symbol decimals }
      state {
        borrowAssets
        borrowAssetsUsd
        supplyAssets
        supplyAssetsUsd
        collateralAssets
        collateralAssetsUsd
        liquidityAssets
        liquidityAssetsUsd
        utilization
        fee
        timestamp
      }
    }
    pageInfo { countTotal }
  }
}
"#;

    // ------------------------------------------------------------------------
    // TABLE 3: IRM State + Current Curve
    // ------------------------------------------------------------------------
    pub const IRM_STATE: &str = r#"
query IrmState($uniqueKey: String!, $chainId: Int!, $curvePoints: Int!) {
  marketByUniqueKey(uniqueKey: $uniqueKey, chainId: $chainId) {
    uniqueKey
    irmAddress
    state {
      borrowApy
      supplyApy
      avgBorrowApy
      avgSupplyApy
      rateAtUTarget
    }
    currentIrmCurve(numberOfPoints: $curvePoints) {
      utilization
      borrowApy
      supplyApy
    }
  }
}
"#;

    // ------------------------------------------------------------------------
    // TABLE 4: Derived Metrics (APYs, Rewards)
    // ------------------------------------------------------------------------
    pub const DERIVED_METRICS: &str = r#"
query DerivedMetrics($chainIds: [Int!], $first: Int!, $skip: Int!) {
  markets(
    first: $first
    skip: $skip
    orderBy: SupplyAssetsUsd
    orderDirection: Desc
    where: { chainId_in: $chainIds }
  ) {
    items {
      uniqueKey
      state {
        utilization
        supplyApy
        borrowApy
        avgSupplyApy
        avgBorrowApy
        avgNetSupplyApy
        avgNetBorrowApy
        rewards {
          asset { address symbol }
          supplyApr
          borrowApr
        }
      }
    }
    pageInfo { countTotal }
  }
}
"#;

    // ------------------------------------------------------------------------
    // TABLE 6: Oracle Data
    // ------------------------------------------------------------------------
    pub const ORACLE_DATA: &str = r#"
query OracleData($chainIds: [Int!], $first: Int!, $skip: Int!) {
  markets(
    first: $first
    skip: $skip
    where: { chainId_in: $chainIds }
  ) {
    items {
      uniqueKey
      oracleAddress
      oracle {
        address
        type
        baseFeedOne { address chain { id } }
        baseFeedTwo { address }
        quoteFeedOne { address }
        quoteFeedTwo { address }
      }
      oracleInfo { type }
    }
    pageInfo { countTotal }
  }
}
"#;

    // ------------------------------------------------------------------------
    // TABLE 7: Collateral-Specific Protocol Data
    // ------------------------------------------------------------------------
    pub const COLLATERAL_DATA: &str = r#"
query CollateralData($chainIds: [Int!], $first: Int!, $skip: Int!) {
  markets(
    first: $first
    skip: $skip
    where: { chainId_in: $chainIds }
  ) {
    items {
      uniqueKey
      lltv
      collateralAsset {
        address
        symbol
        decimals
        priceUsd
      }
      state {
        collateralAssets
        collateralAssetsUsd
      }
    }
    pageInfo { countTotal }
  }
}
"#;

    // ------------------------------------------------------------------------
    // TABLE 10: Vault Data (MetaMorpho)
    // ------------------------------------------------------------------------
    pub const VAULT_DATA: &str = r#"
query VaultData($chainIds: [Int!], $first: Int!, $skip: Int!) {
  vaults(
    first: $first
    skip: $skip
    orderBy: TotalAssetsUsd
    orderDirection: Desc
    where: { chainId_in: $chainIds }
  ) {
    items {
      address
      name
      symbol
      chainId
      asset { address symbol decimals }
      curator { address }
      state {
        totalAssets
        totalAssetsUsd
        totalSupply
        fee
        apy
        netApy
        allocation {
          market { uniqueKey }
          supplyAssets
          supplyAssetsUsd
          supplyCap
        }
      }
    }
    pageInfo { countTotal }
  }
}
"#;

    // ------------------------------------------------------------------------
    // TABLE 11: Vault Strategy Data
    // ------------------------------------------------------------------------
    pub const VAULT_STRATEGY: &str = r#"
query VaultStrategy($address: String!, $chainId: Int!) {
  vaultByAddress(address: $address, chainId: $chainId) {
    address
    name
    timelock
    supplyQueue { uniqueKey lltv loanAsset { symbol } collateralAsset { symbol } }
    withdrawQueue { uniqueKey lltv }
    state {
      allocation {
        market { uniqueKey lltv }
        supplyCap
        supplyAssets
        supplyAssetsUsd
        pendingSupplyAssets
        enabled
      }
    }
  }
}
"#;

    // ------------------------------------------------------------------------
    // TABLE 15 & 16: Historical Time Series (DYNAMIC - use with pagination)
    // ------------------------------------------------------------------------
    pub const HISTORICAL_MARKET: &str = r#"
query HistoricalMarket($uniqueKey: String!, $chainId: Int!, $options: TimeseriesOptions) {
  marketByUniqueKey(uniqueKey: $uniqueKey, chainId: $chainId) {
    uniqueKey
    historicalState {
      supplyAssets(options: $options) { x y }
      borrowAssets(options: $options) { x y }
      supplyAssetsUsd(options: $options) { x y }
      borrowAssetsUsd(options: $options) { x y }
      supplyApy(options: $options) { x y }
      borrowApy(options: $options) { x y }
      utilization(options: $options) { x y }
      fee(options: $options) { x y }
    }
  }
}
"#;

    pub const HISTORICAL_VAULT: &str = r#"
query HistoricalVault($address: String!, $chainId: Int!, $options: TimeseriesOptions) {
  vaultByAddress(address: $address, chainId: $chainId) {
    address
    historicalState {
      totalAssets(options: $options) { x y }
      totalAssetsUsd(options: $options) { x y }
      apy(options: $options) { x y }
      netApy(options: $options) { x y }
      fee(options: $options) { x y }
    }
  }
}
"#;

    pub const HISTORICAL_ASSET_PRICE: &str = r#"
query HistoricalAssetPrice($address: String!, $chainId: Int!, $options: TimeseriesOptions) {
  assetByAddress(address: $address, chainId: $chainId) {
    address
    symbol
    historicalPriceUsd(options: $options) { x y }
  }
}
"#;
}
