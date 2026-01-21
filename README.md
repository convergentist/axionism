# Behavioral Payoff Function

A game-theoretic framework for modeling strategic interactions among vault managers competing to allocate capital across Morpho Blue lending markets. This framework incorporates Prospect Theory to capture behavioral biases in decision-making, including reference-dependent evaluation, risk/loss propension/aversion, and decreasing sensitivity to gain/losses.

## Table of Contents

- [Overview](#overview)
- [Game Structure](#1-game-structure)
- [Market Dynamics](#2-market-dynamics)
- [Two-Stage Payoff Function](#3-two-stage-payoff-function)
- [Parameter Specification](#4-parameter-specification)
- [Gradient Computation](#5-gradient-computation)
- [Algorithms](#6-algorithms)
- [Hyperparameter Guidelines](#7-hyperparameter-guidelines)
- [References](#8-references)

---

## Overview

This framework models a repeated simultaneous-move game where $N$ vault managers allocate capital across $M$ Morpho lending markets. The strategic interactions among players arise from two channels:

1. **Utilization Effects (Primary):** When vaults allocate to the same market, increased supply reduces utilization and thus lending yields for all participants.

2. **Liquidation Externalities (Secondary):** Concentrated positions in a market increase systemic liquidation risk for all vaults exposed to that market.

The key innovation is applying Prospect Theory at the aggregate payoff level with adaptive reference points, creating a non-stationary game that requires specialized equilibrium computation methods rather than classical approaches.

---

## 1. Game Structure

### 1.1 Formal Definition

The game is defined as:

$$\Gamma = \left(N, \{W_i\}_{i=1}^N, \{\tilde{\pi}_i\}_{i=1}^N, \{r_i^t\}_{i=1}^N\right)$$

*Where*:

- $N$ is the set of vault managers (players), indexed by $i \in \{1, \ldots, N\}$
- $W_i$ is the action space for player $i$
- $\tilde{\pi}_i$ is the PT-transformed payoff function for player $i$
- $r_i^t$ is the reference point for player $i$ at time $t$

### 1.2 Action Space

Each vault manager $i$ chooses an allocation over $M$ markets:

$$W_i = \left\{ w_i \in \mathbb{R}_+^M : \sum_{j=1}^M w_{ij} = 1, \; w_{ij} \geq 0 \; \forall j \right\}$$

The action space is the $(M-1)$-simplex $\Delta^{M-1}$.

### 1.3 Strategy Profiles

The joint strategy profile and notation for opponent strategies:

$$w = (w_1, w_2, \ldots, w_N) \in W_1 \times W_2 \times \cdots \times W_N$$

$$w_{-i} = (w_1, \ldots, w_{i-1}, w_{i+1}, \ldots, w_N)$$

### 1.4 Capital

Each vault $i$ controls capital $K_i = \text{TVL}_i > 0$, which determines their market impact.

---

## 2. Market Dynamics

### 2.1 Supply Aggregation

$$S_j(w)$$ as the total supply in market $j$ after all $w$ allocations:

$$S_j(w) = S_j^{\text{ext}} + \sum_{i=1}^N w_{ij} \cdot K_i$$

*Where* :

- $S_j^{\text{ext}}$ is supply from non-strategic agents (passive depositors, retail LPs).

### 2.2 Utilization Rate

Post-allocation utilization in market $j$:

$$U_j(w) = \frac{D_j}{S_j(w)} = \frac{D_j}{S_j^{\text{ext}} + \sum_{i=1}^N w_{ij} \cdot K_i}$$

*Where* :

- $D_j$ is total borrow demand in market $j$.

> **Strategic Implication:** $\frac{\partial U_j}{\partial w_{ij}} < 0$ — increasing allocation to market $j$ reduces utilization (more supply chasing same demand), creating the congestion-game structure.

### 2.3 Interest Rate Model

The borrow rate follows the \textbf{AdaptiveCurveIRM}:

$$R_j^{\text{borrow}}(U) = r_{\text{target},j} \cdot g(U)$$

*Where*:

$$g(U) = \begin{cases}
\left(1 - \frac{1}{4}\right) \cdot \frac{U - U^*}{1 - U^*} + 1 & \text{if } U > U^* \\[6pt]
\left(1 - \frac{1}{4}\right) \cdot \frac{U - U^*}{U^*} + 1 & \text{if } U \leq U^*
\end{cases}$$

with target utilization $U^* = 0.9$.

### 2.4 Supply APY

$$R_j(w) = R_j^{\text{borrow}}(U_j(w)) \cdot U_j(w) \cdot (1 - \phi_j)$$

*Where*:

- $\phi_j$ is the protocol fee (set to 0 for this analysis).

---

## 3. Two-Stage Payoff Function

The payoff is constructed in two stages: raw economic payoff followed by Prospect Theory transformation.

### 3.1 Stage 1: Raw Payoff

$$\boxed{\Pi_i^t(w_i, w_{-i}) = \underbrace{\sum_{j=1}^M w_{ij} \cdot K_i \cdot R_j(w)}_{\text{Gross Yield}} - \underbrace{\sum_{j=1}^M w_{ij} \cdot K_i \cdot \ell_j^t(w)}_{\text{Liquidation Losses}} - \underbrace{\kappa_i \cdot \mathcal{C}(w_i, w_i^{t-1})}_{\text{Rebalancing Cost}}}$$

Compact form:

$$\Pi_i^t(w_i, w_{-i}) = \sum_{j=1}^M w_{ij} \cdot K_i \cdot R_j(w) \cdot \bigl(1 - \ell_j^t(w)\bigr) - \kappa_i \cdot \mathcal{C}(w_i, w_i^{t-1})$$

#### 3.1.1 Liquidation Loss Rate

$$\ell_j^t(w) = \underbrace{\ell_j^{\text{base}}}_{\text{Baseline Risk}} + \underbrace{\psi_j \cdot \left(\sum_{k=1}^N K_k \cdot w_{kj}\right)^{\theta}}_{\text{Concentration Externality}}$$

*Where*:

- $\ell_j^{\text{base}} \geq 0$: Baseline liquidation probability (collateral quality, historical defaults)

- $\psi_j \geq 0$: Concentration risk sensitivity

- $\theta > 1$: Concentration risk convexity (superlinear: doubling exposure more than doubles risk)

> **Strategic Implication:** $\frac{\partial \ell_j^t}{\partial w_{kj}} > 0$ for all $k$ creates strategic substitutes — vaults prefer to avoid markets where others concentrate.

#### 3.1.2 Rebalancing Cost

$$\mathcal{C}(w_i, w_i^{t-1}) = \sum_{j=1}^M c_j \cdot \left| w_{ij} - w_{ij}^{t-1} \right|$$

*Where* :

- $c_j \in [0.001, 0.005]$ to capture gas fees + slippage (0.1–0.5%).

### 3.2 Stage 2: Prospect Theory Transformation

#### 3.2.1 Reference Point Dynamics

The reference point is the previous period's raw payoff:

$$r_i^t = \Pi_i^{t-1}(w_i^{t-1}, w_{-i}^{t-1})$$

**Initialization:** $r_i^0 = \sum_{j=1}^M w_{ij}^{(0)} \cdot K_i \cdot R_j(w^{(0)})$

#### 3.2.2 Value Function

$$V_i(x) = \begin{cases}
x^{\alpha_i} & \text{if } x \geq 0 \quad \text{(gains)} \\[4pt]
-\lambda_i \cdot (-x)^{\beta_i} & \text{if } x < 0 \quad \text{(losses)}
\end{cases}$$

**Parameters:**

| Parameter | Range | Interpretation |
|-----------|-------|----------------|
| $\alpha_i$ | $(0, 1]$ | Diminishing sensitivity to gains |
| $\beta_i$ | $(0, 1]$ | Diminishing sensitivity to losses |
| $\lambda_i$ | $> 1$ | Loss aversion coefficient |

**Canonical Values (Tversky & Kahneman, 1992):** $\alpha_i = \beta_i = 0.88$, $\lambda_i = 2.25$

#### 3.2.3 PT-Transformed Payoff

$$\boxed{\tilde{\pi}_i^t(w_i, w_{-i}) = V_i\bigl(\Pi_i^t(w_i, w_{-i}) - r_i^t\bigr)}$$

---

## 4. Parameter Specification

### 4.1 Vault-Specific Behavioral Parameters

$$\theta_i = \{\alpha_i, \beta_i, \lambda_i, \kappa_i\}$$

| Parameter | Symbol | Constraints | Description |
|-----------|--------|-------------|-------------|
| Gain sensitivity | $\alpha_i$ | $(0, 1]$ | Concavity over gains |
| Loss sensitivity | $\beta_i$ | $(0, 1]$ | Convexity over losses |
| Loss aversion | $\lambda_i$ | $> 1$ | Losses hurt more than equivalent gains |
| Transaction cost sensitivity | $\kappa_i$ | $\geq 0$ | Penalty for rebalancing |

> **Note:** Loss aversion ($\lambda_i > 1$) captures asymmetric risk preferences through the PT value function.

### 4.2 Market-Specific Parameters

| Parameter | Description |
|-----------|-------------|
| $r_{\text{target},j}$ | Target interest rate in IRM |
| $\phi_j$ | Protocol fee (currently 0) |
| $\ell_j^{\text{base}}$ | Baseline liquidation risk |
| $\psi_j$ | Concentration risk sensitivity |
| $c_j$ | Rebalancing cost coefficient |

### 4.3 Global Parameters

| Parameter | Range | Description |
|-----------|-------|-------------|
| $\theta$ | $[1.5, 2.5]$ | Concentration risk convexity |

---

## 5. Gradient Computation

For WoLF-IGA, compute $\nabla_{w_i} \tilde{\pi}_i^t$ via chain rule:

$$\nabla_{w_i} \tilde{\pi}_i^t = V_i'(\Pi_i^t - r_i^t) \cdot \nabla_{w_i} \Pi_i^t$$

### 5.1 Value Function Derivative

$$V_i'(x) = \begin{cases}
\alpha_i \cdot x^{\alpha_i - 1} & \text{if } x > 0 \\[4pt]
\lambda_i \cdot \beta_i \cdot (-x)^{\beta_i - 1} & \text{if } x < 0
\end{cases}$$

> **Numerical Stability:** The kink at $x = 0$ requires smoothing. Use $\epsilon$-interpolation with $\epsilon \approx 10^{-6}$.

### 5.2 Raw Payoff Gradient

Let $Y_j(w) = R_j(w) \cdot (1 - \ell_j^t(w))$ denote risk-adjusted yield:

$$\frac{\partial \Pi_i^t}{\partial w_{im}} = K_i \cdot Y_m(w) + \sum_{j=1}^M w_{ij} \cdot K_i \cdot \frac{\partial Y_j}{\partial w_{im}}$$

---

## 6. Algorithms

### Algorithm 0: Vault Ranking

Construct a ranking system for observable vaults based on historical performance and strategic sophistication.

**Input:**

- Historical allocations $\{w_i^{(t)}\}_{t=1}^T$
- Current TVL: $K_i$
- Market state history $\{R_j^{(t)}, U_j^{(t)}\}_{t=1}^T$

**Output:**

- Ranking $\rho^t$

**Procedure:**

> 1.- Get $TVL+{i}$.

> 2.- Sort in descending order $TVL_{i}$.

---

### Algorithm 1: Calibrate Prospect Theory Parameters

Estimate behavioral parameters from observed decisions using maximum likelihood under *Quantal Response Estimation (QRE)*.

**Input:**

- Historical data $\{(w_i^{(t)}, w_{-i}^{(t)}, \Pi_i^{(t)})\}_{t=1}^T$

- Market states $\{(S_j^{(t)}, D_j^{(t)}, R_j^{(t)})\}_{t=1}^T$

**Output:**

- $\hat{\theta}_i = (\hat{\alpha}_i, \hat{\beta}_i, \hat{\lambda}_i, \hat{\kappa}_i)$

**Procedure:**

> 1. **Construct reference point sequence**
   
>>    For $t > 1$: $r_i^{(t)} = \Pi_i^{(t-1)}$

> 2. **Define QRE choice probability**
   
>>    $$p(w_i \mid w_{-i}, r_i, \theta_i) = \frac{\exp\bigl(\mu \cdot \tilde{\pi}_i(w_i, w_{-i}; \theta_i)\bigr)}{\int_{W_i} \exp\bigl(\mu \cdot \tilde{\pi}_i(w', w_{-i}; \theta_i)\bigr) dw'}$$

> 3. **Maximize log-likelihood**
   
>>   $$\hat{\theta}_i \leftarrow \arg\max_{\theta_i} \sum_{t=1}^T \log p(w_i^{(t)} \mid w_{-i}^{(t)}, r_i^{(t)}, \theta_i)$$
   
>>   Subject to: $\alpha_i, \beta_i \in (0, 1]$, $\lambda_i > 1$, $\kappa_i \geq 0$

> 4. **Validate**
   
>    Compute out-of-sample prediction accuracy; report bootstrap confidence intervals

---

### Algorithm 2: Compute Best Response

Given fixed opponent strategies, maximize PT-transformed payoff.

**Input:**

- Current allocations $w^{t-1}$
- Market state $(S_j^{\text{ext}}, D_j)$
- Parameters $\theta_i$, capital $K_i$, reference $r_i^t$
- Fixed opponent strategies $w_{-i}$

**Output:**

- Best response $w_i^{BR}$

**Procedure:**

1. **Define objective**
   
   $$\tilde{\pi}_i(w_i) = V_i\left(\sum_{j=1}^M w_{ij} \cdot K_i \cdot R_j(w_i, w_{-i}) \cdot (1 - \ell_j(w_i, w_{-i})) - \kappa_i \cdot \mathcal{C}(w_i, w_i^{t-1}) - r_i^t\right)$$

2. **Solve constrained optimization**
   
   $$w_i^{BR} \leftarrow \text{SLSQP}\left(\max_{w_i \in W_i} \tilde{\pi}_i(w_i)\right)$$

3. **Validate**
   
   If $\tilde{\pi}_i(w_i^{BR}) < \tilde{\pi}_i(w_i^{t-1}) - \epsilon$, return $w_i^{t-1}$; else return $w_i^{BR}$

---

### Algorithm 3: WoLF-IGA Policy Learning

Win-or-Learn-Fast Infinitesimal Gradient Ascent (WoLF-IGA) for non-stationary multi-agent learning.

**Input:**

- Initial allocations $w^0$, reference points $r_i^0$
- Parameters $\{\theta_i\}_{i=1}^N$
- Learning rates $\delta_l > \delta_w > 0$
- Smoothing $\eta \in (0, 1)$
- Max iterations $T_{\max}$, tolerance $\epsilon$

**Output:**

- Approximate Nash equilibrium $w^*$
- Policy history $\{w^t\}_{t=0}^T$


---

> **Procedure:**

---

> **Initialize:**

> $$\xi_i^0 \text{ such that } w_i^0 = \text{softmax}(\xi_i^0) \quad \forall i \in \{1, \ldots, N\}$$

> $$\bar{V}_i^0 = 0 \quad \forall i \in \{1, \ldots, N\}$$

> **For** $t = 0$ **to** $T_{\max} - 1$ **do:**

>> **Step 1: Compute allocations**
>> $$w_i^t = \text{softmax}(\xi_i^t) = \frac{\exp(\xi_{ij}^t)}{\sum_{k=1}^M \exp(\xi_{ik}^t)} \quad \forall i, j$$

>> **Step 2: Compute raw payoffs**
>> $$\Pi_i^t = \sum_{j=1}^M w_{ij}^t \cdot K_i \cdot R_j(w^t) \cdot \bigl(1 - \ell_j^t(w^t)\bigr) - \kappa_i \cdot \mathcal{C}(w_i^t, w_i^{t-1})$$

>> **Step 3: Compute PT-transformed payoffs**
>> $$\tilde{\pi}_i^t = V_i\bigl(\Pi_i^t - r_i^t\bigr)$$

>> **Step 4: Determine learning rate (WoLF condition)**
>> $$\delta_i^t = \begin{cases} \delta_l & \text{if } \tilde{\pi}_i^t < \bar{V}_i^t \quad \text{(losing: learn fast)} \\[4pt] \delta_w & \text{if } \tilde{\pi}_i^t \geq \bar{V}_i^t \quad \text{(winning: learn slow)} \end{cases}$$

>> **Step 5: Compute policy gradient**
>> $$g_i^t = V_i'(\Pi_i^t - r_i^t) \cdot \nabla_{\xi_i} \Pi_i^t$$

>> **Step 6: Update policy parameters**
>> $$\xi_i^{t+1} = \xi_i^t + \delta_i^t \cdot g_i^t$$

>> **Step 7: Update reference points**
>> $$r_i^{t+1} = \Pi_i^t$$

>> **Step 8: Update equilibrium value estimates**
>> $$\bar{V}_i^{t+1} = (1 - \eta) \cdot \bar{V}_i^t + \eta \cdot \tilde{\pi}_i^t$$

>> **Step 9: Check convergence** 
>> $$\text{if} \ \displaystyle\max_{i} \|w_i^{t+1} - w_i^t\|_2 < \epsilon \rightarrow break$$

> **end for**

> **Return:** $w_i^* = \text{softmax}(\xi_i^T) \quad \forall i \in \{1, \ldots, N\}$


---


#### Softmax Gradient

The gradient through softmax parameterization:

$$\frac{\partial \Pi_i^t}{\partial \xi_{im}} = \sum_{j=1}^M \frac{\partial \Pi_i^t}{\partial w_{ij}} \cdot w_{ij}^t \cdot (\mathbb{1}_{j=m} - w_{im}^t)$$

---


## 7. Hyperparameter Guidelines

### 7.1 WoLF Learning Rates

| Parameter | Recommended | Notes |
|-----------|-------------|-------|
| $\delta_w$ | 0.01 | Win (slow) learning rate |
| $\delta_l$ | 0.04 | Lose (fast) learning rate |
| Ratio $\delta_l / \delta_w$ | 2–4 | Higher = faster adaptation when losing |

### 7.2 Equilibrium Value Smoothing

| Parameter | Recommended | Notes |
|-----------|-------------|-------|
| $\eta$ | 0.01–0.1 | Smaller = more stable, slower adaptation |

### 7.3 Concentration Risk

| Parameter | Recommended | Notes |
|-----------|-------------|-------|
| $\theta$ | 1.5–2.5 | Higher = more punishing concentration |
| $\psi_j$ | Calibrate | From historical liquidation data |

---

## 8. References

1. Tversky, A., & Kahneman, D. (1992). Advances in prospect theory: Cumulative representation of uncertainty. *Journal of Risk and Uncertainty*, 5(4), 297–323.

2. Bowling, M., & Veloso, M. (2002). Multiagent learning using a variable learning rate. *Artificial Intelligence*, 136(2), 215–250.

3. De Giorgi, E., & Hens, T. (2006). Making prospect theory fit for finance. *Financial Markets and Portfolio Management*, 20(3), 339–360.

---

## License

[Specify your license here]

