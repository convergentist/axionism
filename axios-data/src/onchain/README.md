# OnChain Data Sourcing

## CHAIN IDs REFERENCE

- Ethereum Mainnet: 1
- Base: 8453
- Arbitrum: 42161
- Optimism: 10
- Polygon: 137

## PAGINATION NOTES

- Use `first` (max 100) and `skip` for pagination
- `pageInfo.countTotal` gives total count
- historicalState ONLY works on single-entity queries (marketByUniqueKey, vaultByAddress)
- For 1 month of HOUR data: chunk into 7-day segments to avoid timeouts
- For 1 month of MINUTE data: chunk into 1-day segments

## EXAMPLE: Fetching 1 Month of Hourly Data (4 chunks)

- Chunk 1: startTimestamp = now - 30d, endTimestamp = now - 23d
- Chunk 2: startTimestamp = now - 23d, endTimestamp = now - 16d
- Chunk 3: startTimestamp = now - 16d, endTimestamp = now - 9d
- Chunk 4: startTimestamp = now - 9d,  endTimestamp = now
- Chunk 5: startTimestamp = now - 2d,  endTimestamp = now

