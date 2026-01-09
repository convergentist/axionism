
The following is the **Prospect Theory Value Function** to model how vault managers perceive gains and losses relative to their reference yield.

For a given pool $j$, the change in yield $\Delta_{j}r = \text{supplyAPY}_{j} - r_{ref}$ is the subjective value determined by:

$$ \Delta_{j}r = \begin{cases}
	(\Delta_{j}r)^\alpha & \text{if} \quad \Delta_{j}r \geq 0 \\ 
	-\eta (-\Delta_{j}r)^\beta & \text{if} \quad \Delta_{j}r < 0 
	\end{cases} $$

- $r_{ref}$ : Reference point in the form of a value benchmark, opportunity cost, or, previous state.
- $\Delta_{j}r$ :  The change in yield relative to the reference point.
- $\eta$ : Loss aversion coefficient (typically $\approx 2$).
- $\alpha, \beta$ : Sensitivity parameters, representing the decreasing sensitivity to large changes.

And thus the above model can express the following : 

- **Yield Hunter (Type 1)** 
	- Low $\eta$ exhibits **Propensity for Risk in Losses** (convex utility) to "break even".
- **Safety Player (Type 2)** 
	- High $\eta$ exhibits **Aversion to Risk in Gains** (concave utility) to "lock-in" gains.
- **Improviser (Type 3)**
	- Modeled via $\alpha, \beta \leq 1$ **Decreasing Sensitivity** makes large losses/gains feel marginal compared to changes near $r_{ref}$​.
