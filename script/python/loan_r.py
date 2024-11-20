import numpy as np
import matplotlib.pyplot as plt
from matplotlib.ticker import FuncFormatter

def equ_principal_month(n, y, p):
    r = p / 12.0
    principal_per_month = n / (y * 12)
    profit_count = 0.0
    for index in range(y * 12):
        profit_count += (n - principal_per_month * index) * r
    count = profit_count + n
    count_average_per_month = profit_count / (y * 12) + principal_per_month
    profit_average_per_month = profit_count / (y * 12)
    return count, profit_count, count_average_per_month, profit_average_per_month

def equ_profit_month(n, y, p):
    r = p / 12.0
    count_month = y * 12
    base = r + 1.0
    count_average_per_month = n * r * base**count_month / (base**count_month - 1)
    count = count_average_per_month * count_month
    profit_count = count - n
    profit_average_per_month = profit_count / count_month
    return count, profit_count, count_average_per_month, profit_average_per_month

# Parameters
loan_amount = 1000000  # 贷款金额 100 万
years = 30
interest_rates = [0.0315, 0.0305, 0.0295, 0.0285, 0.0275]  # 不同的利率

# Collect data for plotting
principal_profits = []
profit_profits = []

for rate in interest_rates:
    _, profit_principal, _, _ = equ_principal_month(loan_amount, years, rate)
    _, profit_profit, _, _ = equ_profit_month(loan_amount, years, rate)
    principal_profits.append(profit_principal)
    profit_profits.append(profit_profit)

# Function to format the ticks
def format_ticks(value, tick_number):
    return f'{int(value / 10000)}万'

# Plotting
plt.rcParams['font.sans-serif'] = ['PingFang HK']    # 用于简体中文
plt.rcParams['axes.unicode_minus'] = False  # 解决负号显示问题
plt.figure(figsize=(10, 6))
plt.plot(interest_rates, principal_profits, label='等额本金利息', marker='o')
plt.plot(interest_rates, profit_profits, label='等额本息利息', marker='s')
plt.title('不同利率下,30年,贷款100万,还款利息对比')
plt.xlabel('利率')
plt.ylabel('总利息（万元）')

# Set the formatter for y axis
plt.gca().yaxis.set_major_formatter(FuncFormatter(format_ticks))

# Annotate data points
for i, rate in enumerate(interest_rates):
    plt.text(rate, principal_profits[i], f'{int(principal_profits[i] / 10000)}万', fontsize=9, ha='right')
    plt.text(rate, profit_profits[i], f'{int(profit_profits[i] / 10000)}万', fontsize=9, ha='right')

plt.xticks(interest_rates, [f'{r*100:.2f}%' for r in interest_rates])  # 格式化利率为百分比
plt.legend()
plt.grid(True)
plt.show()