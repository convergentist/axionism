
The solution is comprehended of a hybrid approach that uses an updated economic theory to replace the utility theory, which is the Prospect Theory. Instead of a single expected response a probabilistic approach is provided by the Quantal Response Equilibrium, and, for the iterative process of execution actions, the Win or Learn Fast learning algorithm is used. All of the mentioned elements form the Behavioral QRE-WoLF Allocation Policy.


# Environment definition
---

| Variable       | Definition                                  | Source                  | Computation                                                          | Cardinality                 |
| -------------- | ------------------------------------------- | ----------------------- | -------------------------------------------------------------------- | --------------------------- |
| $S_t$          | System state at time $t$ : $(Uj​,rj​,Lj​)$  | Morpho Subgraph / Nodes |                                                                      | $j \times 3$                |
| $w_{ij}$       | Allocation weight of player $i$ in pool $j$ | Decision Variable       | $w^{avg}_{i,j} = \frac{1}{T} \sum^{T}_{t=1} w_{i,j} (t)$             | $i$-players​ $×$ $j$-pools​ |
| $U_j$          | Utilization Rate of pool $j$                | Morpho Protocol         | $U_{j} = \frac{\text{Total Borrows}}{\text{Total Supply}}$           | $j$-pools​                  |
| $L_{j}$        | Total Liquidity                             | Blockchain Node         |                                                                      | Real-time                   |
| $\text{APY}_j$ | Supply APY of pool $j$                      | Morpho Protocol         | $\text{borrow}APY \times \text{utilization} \times (1 - \text{fee})$ | $i$                         |
| APY_           | Borrow APY of $j$ pool                      | Morpho Protocol         |                                                                      |                             |




# Behavioral (PT)
---

**Equation: Logit Response Function** :

The probability $P$ that a vault $i$ chooses a specific allocation $w_{i}$​ is determined by:

$$ P(w_i | w_{-i}) = \frac{\exp(\lambda_{r} \cdot \bar{u}_i(w_i, w_{-i}))}{\sum_{w'} \exp(\lambda_{r} \cdot \bar{u}_i(w', w_{-i}))} $$

Where: 

- $w_{i}, w_{-i}$ : Strategies for the player and the competitors.
- $\bar{u}_i$ : Expected utility calculated using *Prospect Theory* value function.
- $\lambda_{r}$ : 
	- $\lambda_{r} \rightarrow \infty$ : Converges to a Nash equilibrium.
	- $\lambda_{r} \rightarrow 0$ : Actions become purely random.

The payoff πi​ for player i is the expected subjective value of the portfolio minus a penalty for utilization risk:

$$ \pi_i(w_i, w_{-i}) = \sum_{j} w_{ij} \cdot v(\text{APY}_j(U_j) - r_{ref}) - \text{Penalty}(U_j > 0.9) $$

## Quantal Response Equilibrium (QRE)

## Win or Learn Fast (WOLF)

To handle the non-stationary nature of competing vaults, we incorporate **WoLF-PPO** logic to vary the learning rate.

**Equation: Variable Learning Rate** :

$$ \delta(t) = \begin{cases} \delta_{win} & \text{if } \pi_{current} > \pi_{QRE} \\ \delta_{learn} & \text{if } \pi_{current} \leq \pi_{QRE} \end{cases} $$
Where: 

• $\pi_{QRE}$​: The payoff expected at the estimated *Quantal Response Equilibrium*.
• $\delta_{learn} ​\geq \delta_{win}$​ : We learn faster when underperforming to capture the **First-Mover Advantage**.



