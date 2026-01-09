
Morpho blue's Adaptive Curve IRM

Supply APY

$$
supply_{APY} = borrow_{APY} \cdot R_{u} \cdot (1 - fee)
$$
Where: 

- $borrow_{APY} = e^{(borrow_{Rate} \ \times \ secondsPerYear)} - 1$
- $supply_{apy} = borrow_{APY} \times utilization \times (1 - fee)$
- $utilization = \frac{borrowed}{supplied}$
