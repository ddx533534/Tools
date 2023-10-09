import matplotlib.pyplot as plt
import math

x = [i for i in range(2,16)]
y = [0.321452,
0.364733,
0.411471,
0.350221,
1.164852,
1.013115,
1.611350,
1.662467,
2.068249,
2.188604,
2.129478,
4.885184,
4.988005,
5.641264]
y = [round(math.log10(i),4) for i in y]

plt.scatter(x, y)
plt.title('Cache Plot')
plt.xlabel('2^(i)(KB)')
plt.ylabel('lg(seconds)')
plt.grid(True)  # 显示网格

plt.show() 