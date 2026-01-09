
To find an optimal policy for USDc liquidity allocation/extraction across three Morpho Blue lending pools:

- USDC/cbBTC : 
- USDC/wstETH : 
- USDC/PT-sUSDE : 

This by only conducting lending actions without borrowing actions. Also, incorporating the fact that the challenge poses a zero sum game, that we will be competing against other vault managers for the same liquidity, and when multiple vaults pile into a pool, utilization spikes and rate volatility increases, etc. 

## Scope
---

### Considered aspects

- Interest Rate Model : Morpho's AdaptiveCurveIRM is the only considered.
- Competitors modeling : Given the game theoretic nature of the protocol.
- Risks modeling : Collateral, and Liquidity related risks.
- Behavioral framework: Using the Prospect Theory (Modern approach) almost as a replacement for Expected Utility Theory (Classical approach).
- Policy approach : Instead of static actions, an iterative algorithm for decision making is proposed.
- Collateral modeling (Pending)

### Aspects out of this scope

- Deployment specifics.
- Monitor and rebalancing after deployment.
- Smart contract aspects (deployment, oracle risks)
- Regulatory aspects (accounts, users) 
- Financial transactions (deposits / withdrawals)
- Interaction with other products/strategies in other protocols
- Any other not explicitly included in the Modeling considerations list. 

# Competitors Modeling
---

Given the presence of other agents whose actions directly impact our potential outcomes, a game theoretic approach makes sense.

### Taxonomy

- A non-cooperative dynamic game with perfect information, where multiple players can implement mixed strategies, with the potential convergence to an evolutionary equilibrium.

# Enhancements
---

### Behavioral Modeling

Incorporation of biases even in automated responses

### Probabilistic Responses

Quantal Response  responsiveness converges probabilistically

### Iterative Decision Making

WOLF for Iterative decision making

# Collateral Risk Modeling (Pending)
---

- Collateral Risk: 
	- The collateral asset posses inherent risk sources, it determines the baseline risk profile. 

