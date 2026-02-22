import numpy as np
import scipy.stats as stats
import scipy.optimize as opt


def rosen(x):
    """The Rosenbrock function"""
    return sum(100.0*(x[1:]-x[:-1]**2.0)**2.0 + (1-x[:-1])**2.0)


def black_scholes(S, K, T, r, sigma, option_type='call'):
    d1 = (np.log(S / K) + (r + 0.5 * sigma ** 2) * T) / (sigma * np.sqrt(T))
    d2 = d1 - sigma * np.sqrt(T)
    if option_type == 'call':
        return S * stats.norm.cdf(d1) - K * np.exp(-r * T) * stats.norm.cdf(d2)
    elif option_type == 'put':
        return K * np.exp(-r * T) * stats.norm.cdf(-d2) - S * stats.norm.cdf(-d1)


def main():
    print("Hello from quantify!")
    """np.random.seed(seed=2020)
    beta = stats.beta(a=4, b=2)
    print(beta.rvs(size=10))
    x_0 = np.array([0.5, 1.6, 1.1, 0.8, 1.2])
    res = opt.minimize(rosen, x_0, method='nelder-mead',
                       options={'xtol': 1e-8, 'disp': True})
    print(res)"""
    op = black_scholes(S=100, K=100, T=1, r=0.05,
                       sigma=0.2, option_type='call')
    print(f"Black-Scholes Call Option Price: {op}")


if __name__ == "__main__":
    main()
