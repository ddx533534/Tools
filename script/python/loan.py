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
loan_amounts = [1000000, 1500000, 2000000,2500000, 3000000,3500000, 4000000,4500000, 5000000]
years = 30
interest_rate = 0.0315

# Collect data for plotting
principal_profits = []
profit_profits = []

for amount in loan_amounts:
    _, profit_principal, _, _ = equ_principal_month(amount, years, interest_rate)
    _, profit_profit, _, _ = equ_profit_month(amount, years, interest_rate)
    principal_profits.append(profit_principal)
    profit_profits.append(profit_profit)

# Function to format the ticks
def format_ticks(value, tick_number):
    return f'{int(value / 10000)}万'

# Plotting
plt.rcParams['font.sans-serif'] = ['PingFang HK']    # 用于简体中文
plt.rcParams['axes.unicode_minus'] = False  # 解决负号显示问题
plt.figure(figsize=(10, 6))
plt.plot(loan_amounts, principal_profits, label='等额本金利息', marker='o')
plt.plot(loan_amounts, profit_profits, label='等额本息利息', marker='s')
plt.title('不同贷款金额下,30年,3.15%利率,还款利息对比')
plt.xlabel('贷款金额（万元）')
plt.ylabel('总利息（万元）')

# Set the formatter for x and y axis
plt.gca().xaxis.set_major_formatter(FuncFormatter(format_ticks))
plt.gca().yaxis.set_major_formatter(FuncFormatter(format_ticks))

# Annotate data points
for i, amount in enumerate(loan_amounts):
    plt.text(amount, principal_profits[i], f'{int(principal_profits[i] / 10000)}万', fontsize=9, ha='right')
    plt.text(amount, profit_profits[i], f'{int(profit_profits[i] / 10000)}万', fontsize=9, ha='right')

plt.legend()
plt.grid(True)
plt.xticks(loan_amounts)
plt.show()
