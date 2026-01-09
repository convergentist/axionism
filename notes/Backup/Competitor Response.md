
Players do not choose the single best response; they choose "better" responses with higher probability.

## Logit Equilibrium

The probability that competitor $i$ chooses allocation $w_i$​ is:

$$ P(w^{i} | w^{-i}) = \frac{\exp(\lambda^{i}_{r} \cdot \bar{u}^i(w^{i}, w^{-i}))}{\sum_{w^{j}} \exp(\lambda^{j}_{r} \cdot \bar{u}^{j}(w^{j}, w^{-j}))} $$

Where : 

- $\lambda^{i}_{r}$ : Rationality on competitor $i$
- $w^i$ : allocation action from competitor $i$
- $w^{-i}$ : allocation action from any other competitor, except $i$.
- $\bar{u}^{i}$ : utility function for $i$.

• As $\lambda_{r} \to \infty$ the player becomes perfectly rational (Nash).
• As $\lambda_{r} \to 0$ the player becomes an **Improviser** (Type 3), acting randomly

# Logit Equilibrium (Corrected)

The probability $\pi_{i,j}$ that competitor $i$ chooses a specific allocation strategy $j$ (where $j$ is one possible weight vector $w_{i,j}$​ from their strategy set $A_{i}$​) is defined as:

$$ \pi_{ij} = P(w_{ij} | \pi_{-i}) = \frac{\exp(\lambda_i \cdot \bar{u}_{ij}(\pi))}{\sum_{k=1}^{J_i} \exp(\lambda_i \cdot \bar{u}_{ik}(\pi))} $$

**Where:**

• $\pi_{ij}$​ : The probability (or frequency) with which player $i$ chooses action $j$.
• $\lambda_{i}$ ​: The **rationality (precision) parameter** for competitor $i$. It dictates the "noise" in their decision-making: as $\lambda \to 0$, actions become purely random; as $\lambda \to \infty$, the player converges to a perfectly rational Nash best-response.
• $w_{i,j}$​ : A specific allocation action (weight vector) for competitor $i$ from their finite set of possible strategies $A_{i} ​= \{ w_{i1}​,...,w_{i}J_{i}​​ \}$.
• $\pi_{-i}$ : The probability distribution of strategies chosen by all other competitors except $i$.
• $\bar{u}_{i,j}(\pi)$ : The **expected utility** for player $i$ when choosing strategy $j$, calculated as $u_{i}(w_{ij}, \pi_{-i}​)$. In your model, this incorporates the S-shaped value function from Prospect Theory.
• $J_{i}$​ : The total number of discrete allocation strategies available to player $i$ in the strategy set $A_{i​}$.



