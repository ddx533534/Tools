import matplotlib.pyplot as plt


# 打开文件（如果文件不存在，将会创建一个空文件）
file_path = '2core/2core-2process.txt'  # 替换为你的文件路径

# 初始化两个空数组来保存以0和1开头的数据
data_starting_with_0 = []
data_starting_with_1 = []

# 逐行读取文件并根据以0或1开头的数据将其添加到相应的数组中
with open(file_path, 'r') as file:
    lines = file.readlines()
    for line in lines:
        parts = line.strip().split('\t')
        if len(parts) >= 3:
            first_char = parts[0]
            if first_char == '0':
                data_starting_with_0.append(parts)
            elif first_char == '1':
                data_starting_with_1.append(parts)

# 预处理数据
x1 = [int(row[1]) for row in data_starting_with_0]
y1 = [int(row[2]) for row in data_starting_with_0]

x2 = [int(row[1]) for row in data_starting_with_1]
y2 = [int(row[2]) for row in data_starting_with_1]

data_dict1 = dict(zip(x1, y1))
data_dict2 = dict(zip(x2, y2))

# 创建 z 范围从 0 到 250
x_values = range(0, 121)

# 生成对应的 y 值列表
y_values1 = [data_dict1[x] if x in data_dict1 else -1 for x in x_values]
y_values2 = [data_dict2[x] if x in data_dict2 else -1 for x in x_values]

# 创建主图形和主轴
fig, ax1 = plt.subplots()

# 绘制第一组数据在主轴上
ax1.scatter(x_values, y_values1,s=1, label='group1', color='blue', marker='o')

# 设置主轴标签
ax1.set_xlabel('time(ms)')
ax1.set_ylabel('progress(%)', color='blue')
ax1.tick_params(axis='y', labelcolor='blue')

# 创建次坐标轴（共享x轴）
ax2 = ax1.twinx()

# 绘制第二组数据在次坐标轴上
ax2.scatter(x_values, y_values2,s=1, label='group2', color='red', marker='o')

# 设置刻度
plt.xticks(range(0, int(max(x_values)) + 1, 10))
plt.yticks(range(0, int(max(max(y_values1, y_values2))) + 1, 25))

# 显示图形
plt.show()





