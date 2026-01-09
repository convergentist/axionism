
This policy seeks to reach a **Nash Equilibrium Strategy (NES)** that provides a known lower bound payoff when dealing with other rational agents.

## 1. Payoff Definition (Prospect Theory Utility)

The payoff $\pi_{i}$​ for the vault is not just the linear $APY$, but the subjective value $v$ relative to a reference point $r_{ref}$​ (typically the current portfolio yield):

$$ \pi_i(w_i, w_{-i}) = \sum_{j=1}^M w_{ij} \cdot \Delta r(w_i, w_{-i}) $$

Where the value function $v(x)$ is concave for gains (Risk Aversion) and convex for losses (Risk Seeking).

## 2. Competitive Prediction (QRE)

We use the **Logit Equilibrium** to predict the probability $P$ that a competitor $i$ will choose a specific allocation $w_i$​. This models rivals as "noisy" rational players who choose *better* responses with higher probability:

$$ P(w_i | w_{-i}) = \frac{\exp(\lambda_{r} \cdot \bar{u}_i(w_i, w_{-i}))}{\sum_{w'_i} \exp(\lambda_{r} \cdot \bar{u}_i(w'_i, w_{-i}))} $$
Where: 

- $\lambda_{r}$ ​: The rationality parameter. 
- $\bar{u}_{i}$ : Expected utility for our strategy.
- $w_{i}$ : Our Specific Liquidity Allocation 

	Note: 
	High $\lambda$ predicts a standard Nash best response; low $\lambda$ predicts random behavior.

## 3. The WoLF-PPO Learning Rate Adjustment

The "gap" is defined as the difference between your vault's current payoff $\pi_{current​}$ and the expected payoff at the predicted $QRE$ equilibrium $\pi_{QRE}$​. You adjust your allocation wi​ using a variable learning rate $\delta(t)$ : 

$$ w_i^{(t+1)} = w_i^{(t)} + \delta(t) \cdot [BR_i(w_{-i}^{(t)}) - w_i^{(t)}] $$

The Adjustment Logic:

• Winning ($\pi_{current} \geq \pi_{QRE}$)

If the vault is outperforming the predicted equilibrium, it is "winning." It uses a **smaller learning rate** $\alpha_{win}$​ to be cautious and maintain stability.

• Learning ($\pi_{current} \leq \pi_{QRE}$​)

If the vault detects a gap where it is underperforming (e.g., due to a competitor's non-rational move like a **Type 1 Yield Hunter** pushing utilization to 95%), it must "learn fast." It uses a **larger learning rate** $\alpha_{lose}$​ to rapidly reallocate capital.

## Algorithmic Specification
---

| Step  | Operation         | Variables Used                      | Logic                                                                                                 |
| ----- | ----------------- | ----------------------------------- | ----------------------------------------------------------------------------------------------------- |
| **1** | **Scan On-Chain** | $U_{j}​, L_{j}, w_{−i}^{observed​}$ | Fetch current market state and actual competitor weights.                                             |
| **2** | **Predict QRE**   | $\lambda_{r}​, P(w_i​∣w_{−i}​)$     | Compute the predicted distribution of rival moves.                                                    |
| **3** | **Detect Gap**    | $\pi_{current}​,\pi_{QRE​}$         | Calculate if the vault is winning or losing against the predicted NES.                                |
| **4** | **Apply WoLF**    | $\alpha_{win}​,\alpha_{lose​}$      | Set $\delta(t) = \alpha_{lose}$​ if $\pi_{current} \leq \pi_{QRE}$​ to capture First-Mover Advantage. |
| **5** | **Rebalance**     | $\Delta w_{i​}$                     | Execute if yield gain > gas costs and $                                                               |

**Variable Definitions:**

• $\alpha_{lose}$ ​: Learning rate when underperforming. Recommended ratio is $4 \times \alpha_{win}$​.
• $\lambda_{r}$ ​: Inferred from historical competitor noise. A high-variance competitor receives a lower $\lambda{r}$
• $BR_i$​: The Best-Response function, maximizing utility against the predicted $w_{−i}$​.


