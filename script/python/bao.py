import pandas as pd
import matplotlib.pyplot as plt

# 读取数据
df = pd.read_csv('data.csv', sep='\t', header=None)

# 选择第一列和第四列的数据，并按照第四列的数据从大到小排序
df = df[[0, 3]].sort_values(by=3, ascending=False)

# 画条形图
plt.barh(df[0], df[3])
plt.xlabel('Value')
plt.ylabel('Library')
plt.title('Bar Chart')
plt.gca().invert_yaxis()  # 反转y轴，使得值最大的条形位于最上方
plt.show()