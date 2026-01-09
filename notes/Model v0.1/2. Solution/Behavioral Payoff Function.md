
Prospect Theory for Value function considering the reference point (for loss aversion, disposition effect, and, sensitivity decrease), as well with transaction and rebalancing costs.

- Base Payoff
- Value Function
- Risk Penalty
- Transaction Cost

**Risk-Adjusted, Behavioral Payoff Function:**

```math
  \boxed{\pi_i(w_i, w_{-i}) = \sum_{j=1}^M w_{ij} \cdot v_i(\Delta r_{ij}) - \gamma_i \cdot \text{Risk}_i(w_i, w) - \kappa_i \cdot \text{Cost}_i(w_i, w_i^{(0)})}
```

Where:

- $\pi_i(w_i, w_{-i})$ : Payoff function, for the vault $i$, given the allocation strategies of vault $i$ and all the rest of participating vaults $-i$.
- $w_{ij}$ : Allocation strategy of vault $i$ in pool $j$.
- $v_{i}(\Delta r_{ij})$ :  The change in yield relative to the reference point. 
- $\gamma_i \geq 0$: **Risk aversion** for vault $i$.
- $\text{Risk}_i(w_i, w)$: **Risk penalty function** (specified below).
- $\kappa_i \geq 0$: **Transaction cost sensitivity** for vault $i$.
- $\text{Cost}_i(w_i, w_i^{(0)})$: **Rebalancing cost function** (specified below).

## Value Function

**Yield Change for Vault $i$ in Market $j$:**

$$\Delta r_{ij}(w) = r_j(w) - r_{\text{ref},i}$$
Where: 
$$v_i(\Delta r_{ij}) = \begin{cases}
(\Delta r_{ij})^{\alpha_i} & \text{if } \Delta r_{ij} \geq 0 \quad \text{(gains)} \\
-\eta_i \cdot (-\Delta r_{ij})^{\beta_i} & \text{if } \Delta r_{ij} < 0 \quad \text{(losses)}
\end{cases}$$

**Parameters:**

- $\alpha_i \in (0, 1]$: **Sensitivity to gains** for vault $i$. When $\alpha_i < 1$, exhibits diminishing sensitivity (concave utility over gains, implying risk aversion in gains).
- $\beta_i \in (0, 1]$: **Sensitivity to losses** for vault $i$. When $\beta_i < 1$, exhibits diminishing sensitivity (convex utility over losses, implying risk-seeking in losses).
- $\eta_i > 1$: **Loss aversion coefficient** for vault $i$. Measures how much more losses hurt compared to equivalent gains.

$$\alpha_i = \beta_i = 0.88, \quad \lambda_i = 2.25$$

- Tversky, A., & Kahneman, D. (1992). Advances in prospect theory: Cumulative representation of uncertainty. *Journal of Risk and Uncertainty*, 5(4), 297-323.

### Reference Determination

The reference point $r_{\text{ref},i}$ captures vault $i$'s baseline expectation, the following are 3 different ideas for this: Status quo (and the one used further in the project), Market Average Yield, Best Available Yield.

**Reference:** This follows the status quo reference point formulation in Kahneman & Tversky (1979) and its application to portfolio theory in De Giorgi & Hens (2006).

#### A) Status Quo

$$r_{\text{ref},i} = \sum_{j=1}^M w_{ij}^{(0)} \cdot r_j(w^{(0)})$$
Where:
- $w^{(0)} = (w_1^{(0)}, \ldots, w_N^{(0)})$ is the current allocation profile (before rebalancing)
- $w_i^{(0)}$ is vault $i$'s current allocation
- $r_j(w^{(0)})$ is the current yield in market $j$


#### B) Market Average Yield

$$r_{\text{ref},i} = \frac{1}{M} \sum_{j=1}^M r_j(w^{(0)})$$

#### C) Best Available Market

$$r_{\text{ref},i} = \max_{j \in \{1,\ldots,M\}} r_j(w^{(0)})$$

## Risk Penalty

**Utilization Risk Penalty:**

Penalizes allocation to high-utilization markets (liquidity withdrawal risk):

```math
  \text{Risk}_i^{\text{util}}(w_i, w) = \sum_{j=1}^M w_{ij} \cdot \max\left(0, U_j'(w) - U_{\text{safe}}\right)^2
```

Where : 

- $U_{\text{safe}} = 0.85$ is the safety threshold (adjustable parameter).

**Concentration Risk Penalty ():**

Using the concept of the Herfindahl Index, which penalizes under-diversification :

```math
  \text{Risk}_i^{\text{conc}}(w_i) = \sum_{j=1}^M w_{ij}^2
```

This is minimized at $1/M$ (equal allocation) and maximized at $1$ (single-market concentration).

**Combined Risk Function:**

```math
  \text{Risk}_i(w_i, w) = \rho_1 \cdot \text{Risk}_i^{\text{util}}(w_i, w) + \rho_2 \cdot \text{Risk}_i^{\text{conc}}(w_i)
```

Where:

- $\rho_1, \rho_2 > 0$ are weighting parameters (typically $\rho_1, \rho_2 \in [0.5, 2]$).

## Transaction Cost


```math
  \text{Cost}_i(w_i, w_i^{(0)}) = c_{\text{gas}} \cdot \text{TVL}_i \cdot \sum_{j=1}^M \left|w_{ij} - w_{ij}^{(0)}\right|
```

Where :

- $c_{\text{gas}} \in [0.001, 0.005]$ represents the per-dollar cost of rebalancing (gas fees + slippage), typically 0.1-0.5%.
- $TVL_i$ : current Total Value Locked for the $i$ Vault.
- $w_{ij}$ :  The Vault $i$'s intended allocation on pool $j$.
- $w_{ij}^{(0)}$ : The vault $i$'s current allocation on pool $j$.

