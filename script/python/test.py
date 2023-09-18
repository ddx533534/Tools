# 示例数据
x = [20, 60, 100, 180]
y = [30, 50, 70, 90]

# 创建一个字典将 x 和 y 值关联起来
data_dict = dict(zip(x, y))

# 创建 z 范围从 0 到 240
z_range = range(0, 241)

# 生成对应的 y 值列表
y_values = [data_dict[x] if x in data_dict else "null" for x in z_range]

print(z_range)
print(y_values)
