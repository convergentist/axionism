
For this project, there were developed 4 algorithms:

1. Calibration of $\theta_{i}$ Behavioral params for every Vault $i$.
2. Compute the $\omega_{i}$ Best Response for a single vault $i$ perspective.
3. Compute the allocati0on policy using WOLF

# Algorithm 0: Compute Vault Ranking.
---

Using both the historical allocations and the current (or last knonw) set of values for each vault $i$, the creation of a ranking system for the observable vaults, is defined through the following algorithm:

**Input:**
- Vault's $i$ Historical Allocation Data.
- Vault's $t$ (Last) Total Value Locked (TVL).

**Output:**
- Ranking $\rho^{t}$:

# Algorithm 1: Calibrate Prospect Theory Params.
---

For each Vault *i*, the following algorithm is used to compute the $\theta$ params formulated with prospect theory: $\theta_{i} = \{ \alpha_{i}, \beta_{i}, \lambda_{i}, \gamma_{i}, \kappa_{i} \}$. 

**Input:**
- Historical allocation data {(*wᵢ*⁽ᵗ⁾, *w*₋ᵢ⁽ᵗ⁾, *r*⁽ᵗ⁾)} for *t* = 1, ..., *T*

**Output:**
- Estimated parameters $\hat{\theta_{i}} =  \( \hat{\alpha_{i}}, \hat{\beta_{i}}, \hat{\lambda_{i}}, \hat{\gamma_{i}}, \hat{\kappa{i}} \)$
<!-- $θ̂ᵢ = (α̂ᵢ, β̂ᵢ, λ̂ᵢ, γ̂ᵢ, κ̂ᵢ) -->

---

1. **Define likelihood function**
   
   For each observed allocation *wᵢ*⁽ᵗ⁾, assume it's the best response:
   
   ℒ(θᵢ | data) = ∏ₜ *p*(*wᵢ*⁽ᵗ⁾ | *w*₋ᵢ⁽ᵗ⁾, *r*⁽ᵗ⁾, θᵢ)

2. **Model choice probability**
   
   *p*(*wᵢ* | *w*₋ᵢ, *r*, θᵢ) = QRE

3. **Optimize log-likelihood**
   
   θ̂ᵢ ← arg maxₜₕₑₜₐ log ℒ(θᵢ | data)

4. **Validate fit**
   
   Compute out-of-sample prediction accuracy on held-out data
   
   **return** θ̂ᵢ


## Algorithm 1: Compute Best Response for Vault *i*

**Input:**
- Current allocations: $w^0 = (w_{1}^{0}, ..., w_{n}^{0})$
- Market state: $(S_{j}^{0}, B_{h}^{0})$ for $j = 1, ..., M$
- Vault *i* parameters: (αᵢ, βᵢ, λᵢ, γᵢ, κᵢ, TVLᵢ)
- Opponent strategies space: *w*₋ᵢ

**Output:** Decision *wᵢ*

**Require:** SLSQP (Sequential Least Squares Programming)

---

1. **Compute reference point**
   
   *r*ᵣₑ𝒻,ᵢ ← Σⱼ *wᵢⱼ*⁽⁰⁾ · *rⱼ*(*w*⁽⁰⁾)

2. **Define objective function**
   
   πᵢ(*wᵢ*) = Σⱼ *wᵢⱼ* · *vᵢ*(*rⱼ*(*wᵢ*, *w*₋ᵢ) − *r*ᵣₑ𝒻,ᵢ) − γᵢ · Riskᵢ(*wᵢ*) − κᵢ · Costᵢ(*wᵢ*)

3. **Set up optimization problem**
   
   **maximize** πᵢ(*wᵢ*) over *wᵢ* ∈ *Wᵢ*
   
   subject to: Σⱼ *wᵢⱼ* = 1, *wᵢⱼ* ≥ 0, ∀*j* ∈ {1, ..., *M*}

4. **Solve using nonlinear optimization**
   
   *wᵢ*ᴮᴿ ← SLSQP(πᵢ, *wᵢ*⁽⁰⁾, constraints)

5. **Validate solution**
   
   **if** πᵢ(*wᵢ*ᴮᴿ) < πᵢ(*wᵢ*⁽⁰⁾) − ε **then**
   > **return** *wᵢ*⁽⁰⁾
   
   **else**
   > **return** *wᵢ*ᴮᴿ

---

	


## Compute Best Response for Vault $i$

```pseudo
\begin{algorithm}
\caption{Compute Best Response for Vault $i$}
\begin{algorithmic}

\input \\
	Current allocations: $w^{(0)} = (w_1^{(0)} \ldots, w_N^{(0)})$ \\
	Market state: $(S_j^{(0)}, B_j^{(0)})_{j=1}^M$ \\
	Vault $i$ parameters: $(\alpha_i, \beta_i, \lambda_i, \gamma_i, \kappa_i, \text{TVL}_i)$ \\
	Opponent strategies space: $w_{-i}$ \\

\Output \\ 
	decision: $w_{i}$
	
\Require \\
	$\text{SLSP: Sequential Least Squares Programming}$

\Procedure{Decision}{}
	\State \textbf{Step 1:} Compute reference point
	\State $\quad$ $r_{\text{ref},i} \leftarrow \sum_{j=1}^M w_{ij}^{(0)} \cdot r_j(w^{(0)})$
	\State \textbf{Step 2:} Define objective function \\
	\State $\quad$ $\pi_i(w_i) = \sum_{j=1}^M w_{ij} \cdot v_i(r_j(w_i, w_{-i}) - r_{\text{ref},i}) - \gamma_i \cdot \text{Risk}_i(w_i) - \kappa_i \cdot \text{Cost}_i(w_i)$
	\State \textbf{Step 3:} Set up optimization problem \\
	\State $\quad$ \textbf{maximize} $\pi_i(w_i)$ over $w_i \in W_i$
	$\ s.t. \ \sum_{j=1}^M w_{ij} = 1, \ w_{ij} \geq 0, \ \forall j \in \{1, \ldots, M\}$
	\State \textbf{Step 4:} Solve using nonlinear optimization
	\State $\quad$ $w_i^{\text{BR}} \leftarrow \text{SLSQP}(\pi_i, w_i^{(0)}, \text{constraints})$
	\State \textbf{Step 5: Validate solution} 
		\If{$\pi_i(w_i^{\text{BR}}) < \pi_i(w_i^{(0)}) - \epsilon$}
			\Return$w_i^{(0)}$
		\Else
			\Return $w_i^{\text{BR}}$ \EndIf	
\EndProcedure
\end{algorithmic}
\end{algorithm}

```

## Calibrate $PT$ parameters

```pseudo
\begin{algorithm}
\caption{calibrate prospect theory parameters for vault $i$}
\begin{algorithmic}

\Input \\

\State Historical allocation data $\{(w_i^{(t)}, w_{-i}^{(t)}, r^{(t)})\}_{t=1}^t$ for vault $i$

\Output \\

\State Estimated params $\hat{\theta}_i = (\hat{\alpha}_i, \hat{\beta}_i, \hat{\lambda}_i, \hat{\gamma}_i, \hat{\kappa}_i)$

\Procedure{Compute}{}

	\State \textbf{Step 1:} Define likelihood function \\
	$ \ \ $for each observed allocation $w_i^{(t)}$, assume it's the best response: \\
	
	\State $\quad \mathcal{l}(\theta_i \mid \text{data}) = \prod_{t=1}^t p(w_i^{(t)} \mid w_{-i}^{(t)}, r^{(t)}, \theta_i)$
	
	\State \textbf{Step 2:} Model choice probability \\
	
	\State $\quad p(w_i \mid w_{-i}, r, \theta_i) = QRE$
	
	\State \textbf{Step 3:} Optimize log-likelihood \\ 
	
	\State$\quad\hat{\theta}_i \leftarrow \arg\max_{\theta_i} \log \mathcal{l}(\theta_i \mid \text{data})$
	
	\State \textbf{Step 4:} Validate fit \\
	$ \ \ $compute out-of-sample prediction accuracy on held-out data
	
	\State $\quad$\textbf{return} $\hat{\theta}_i$

\EndProcedure
\end{algorithmic}
\end{algorithm}
```

test-driven development, freq of deploys slowed down. longer term larger releases. 

