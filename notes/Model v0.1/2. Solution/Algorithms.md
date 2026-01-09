
# Best response for Vault $i$
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
