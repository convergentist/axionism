
By analyzing historical rebalancing events for every competitor $i$, the anchor event is the act of conducting a capital allocation action (adding/extracting) into and from a pool.

# Behavior Types

Type 1 : Risk Seeking with Loss Aversion (Yield hunter)

- Risk Seeking $\rightarrow$ Fast reactor.
- Loss aversion : Low $\to \lambda \approx 1.5$
- Sensitivity : Convex in losses $\to \beta > \alpha$
- Risk Aversion : Low $\to \gamma < 1$
- Behavior : Chases high yields, risk seeking in losses.
- Would reallocate to a pool approaching 90% trying to reach break even.

Type 2 : Risk Aversion with Safety Seeking (Safety player)

- Risk Averse $\rightarrow$ Slow reactor.
- Loss aversion : High $\to \lambda \approx 3.0$
- Sensitivity : Concave in gains $\to \alpha < \beta$
- Risk Aversion : High $\to \gamma > 5$
- Behavior : Locks-in gains, avoids high utilization markets.
- Would cut profitable pool allocations too early to "lock-in" modest gains.

Type 3 : Sensitivity Decrease (Improviser mode)

- Decreasing Sensitivity $\rightarrow$ Numbed reactor.
- Loss aversion : Medium $\to \lambda \approx 2.0$
- Sensitivity : Diminishing $\to \alpha , \beta \ll 1$
- Risk Aversion : Medium $\to \gamma \approx 2$
- Behavior : Adaptive (Large changes feel marginal).
- Would exhibit a marginal subjective evaluation of the value of a change, in a decreasing manner, as the magnitude of that change increases.

Type 4 : Rational Optimizer (Classical and almost theoretic case)

- Loss aversion : None $\to \lambda = 1$
- Sensitivity : Linear $\to \alpha = \beta = 1$
- Risk Aversion : Low $\to \gamma = 0.5$
- Behavior : Classic Expected Utility Maximizer.
- Would exhibit perfect rationality without any effect of the previously cited biases.


## Quantifying biases

Using historical rebalance events to identify the patterns :

### Loss Aversion

The monetary proportion of realized losses are equal or greater than two times the realized gains.

```math
  LA = \frac{|\text{Floating Loss}_{\text{max}}|}{|\text{Realized Gain}|} > 2
```

### Disposition Effect

The presence of "cutting fast gains", and, "holding losses too long" patterns.

1. **Identify Anchors:** For a given competitor, extract all rebalance events where they moved capital out of a pool that was yielding above their reference APY (the "winner").
2. **Scan for Floating Losses:** For every anchor event, check if the vault simultaneously maintained exposure to an underperforming pool (the "loser") that had a lower APY or was yielding a relative loss.
3. **Apply the Ratio Test:** Measure the disproportion between holding time between the realized gain and the retained loss.

```math
  DE = \frac{1}{N} \sum_{i=1}^{N} \Delta\tau^{i}_{loss} - \frac{1}{M} \sum_{j=1}^{M} \Delta\tau^{j}_{gain}
```

Where : 

- $\Delta\tau^{i}_{loss}, \ \Delta\tau^{j}_{gain}$ : Difference between open and close time for a given $i$, or, $j$ allocation.
- that resulted in an either $loss$ or a $gain$ outcome, respectively.

### Sensitivity Decrease

The tendency to exhibit **diminishing sensitivity** to large yield changes relative to small ones, manifesting as:

1. An **inaction zone** where moderate yield changes fail to trigger rebalancing
2. A **concave response function** where doubling the yield signal does not double the allocation response

#### Step 1: Extract Yield Change Events

For each vault, construct pairs of observations. $( |\Delta r_j|, | \Delta w_{j} | )$

Where: 

- $|\Delta r_j| = | r^{post}_{j} − r^{pre}_{j} |$ : Is the absolute yield change in pool $j$.
- $|\Delta w_j| = |w^{post}_{j} - w^{pre}_{j}|$ : Is the absolute allocation change in response.

#### Step 2: Fit Power Function

Estimate the sensitivity parameter $\alpha$ from the response function.

```math
  |\Delta w_{j}| = k \cdot |\Delta r_{j} |^{\alpha} + \epsilon
```

Where: 

- $k > 0$ : is a scaling constant.
- $\alpha \in (0, 1]$ : The sensitivity exponent.
- $\epsilon$ : Noise or residual. 

#### Step 3: Build interpretation

Response classification based on empirical values crossed with evidence from Prospect Theory.

- $\alpha = 1$ : Linear response (no diminishing sensitivity)
- $\alpha \leq 1$ : Concave response (diminishing sensitivity is present)

