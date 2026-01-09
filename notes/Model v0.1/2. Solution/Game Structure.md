
## 1. Pre-Allocation Market State
---

### Game

The game consists of $N$ vault managers (players) competing to allocate capital across $M$ Morpho markets (pools).

$$\Gamma = (N, \{W_i\}_{i=1}^N, \{\pi_i\}_{i=1}^N)$$
### Players

$N$ vault managers, indexed by $i \in \{1, \ldots, N\}$

### Markets

$M$ Morpho markets, indexed by $j \in \{1, \ldots, M\}$ (in this case, $M = 3$)

### Action Space (Strategy Set) for Player $i$

$$W_i = \left\{ w_i \in \mathbb{R}_+^M : \sum_{j=1}^M w_{ij} = 1, \; w_{ij} \geq 0 \; \forall j \right\}$$

Where:
- $w_{ij}$ represents the proportion of vault $i$'s capital allocated to market $j$.

### Joint Strategy Profile

$$w = (w_1, w_2, \ldots, w_N) \in W_1 \times W_2 \times \cdots \times W_N$$
### Strategies of All Players Except $i$

$$w_{-i} = (w_1, \ldots, w_{i-1}, w_{i+1}, \ldots, w_N)$$

### Total Value Locked (TVL) for Vault $i$

$$\text{TVL}_i > 0$$

## 2. Post-Allocation Market State
---

### Capital Allocation from Vault $i$ to Market $j$

$$\Delta S_{ij} = w_{ij} \cdot \text{TVL}_i$$

### Total Supply in Market $j$ After All Allocations

$$S_j'(w) = S_j^{(0)} + \sum_{i=1}^N \Delta S_{ij} = S_j^{(0)} + \sum_{i=1}^N w_{ij} \cdot \text{TVL}_i$$

Where:
- $S_j^{(0)}$ is the current total supply in market $j$ before new allocations.

### Post-Allocation Utilization Rate in Market $j$

$$U_j'(w) = \frac{B_j^{(0)}}{S_j'(w)} = \frac{B_j^{(0)}}{S_j^{(0)} + \sum_{i=1}^N w_{ij} \cdot \text{TVL}_i}$$

Where : 

- $B_j^{(0)}$ is the current total borrows in market $j$ (assumed constant in short term).

### Supply APY for Market $j$ Given Strategy Profile $w$

The Interest Rate Model (IRM) for market $j$ determines the borrow rate as a function of utilization:

$$r_j^{\text{borrow}}(U) = \text{IRM}_j(U)$$

For the case of *AdaptiveCurveIRM*:

$$\text{IRM}_j(U) = r_{\text{target},j} \cdot g(U)$$

Where:
$$g(U) = \begin{cases}
\left(1 - \frac{1}{4}\right) \cdot \frac{U - 0.9}{1 - 0.9} + 1 & \text{if } U > 0.9 \\
\left(1 - \frac{1}{4}\right) \cdot \frac{U - 0.9}{0.9} + 1 & \text{if } U \leq 0.9
\end{cases}$$

**Supply APY in Market $j$:**

$$r_j(w) = r_j^{\text{borrow}}(U_j'(w)) \cdot U_j'(w) \cdot (1 - \phi_j)$$

Where:
- $\phi_j$ is the protocol fee for market $j$ (considered 0 for now).

**Simplified Notation:**

For clarity, we denote:

$$r_j(w) = \text{SupplyAPY}_j(w_1, w_2, \ldots, w_N)$$

