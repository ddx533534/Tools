# -*- coding: utf-8 -*-
import matplotlib.pyplot as plt

# 定义横坐标数据
x = [1, 2, 3, 4, 5]

# 定义纵坐标数据
y1 = [2, 4, 6, 8, 10]
y2 = [1, 3, 5, 7, 9]
y3 = [3, 6, 9, 12, 15]

# 绘制折线图
plt.plot(x, y1, color='blue', label='Line 1')
plt.plot(x, y2, color='red', label='Line 2')
plt.plot(x, y3, color='green', label='Line 3')

# 设置坐标轴标签
plt.xlabel('x')
plt.ylabel('y')

# 显示图例
plt.legend()

# 显示图形
plt.show()
