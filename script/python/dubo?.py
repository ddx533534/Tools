# -*- coding: utf-8 -*-
import matplotlib.pyplot as plt
# 薪资参考数据来源:http://www.199it.com/archives/1266088.html
benke_success_graduation_rate = 0.9
benke_salary_month = 0.6
benke_start = 0

shuo_success_graduation_rate = 0.8
shuoshi_salary_month = 0.9
shuoshi_start = 2

boshi_success_graduation_rate = 0.4
boshi_salary_month = 1.4
boshi_start = 4

# 中智咨询调研结果显示
# 43% 的企业表示每年都上调应届生起薪
# 48% 的企业会隔年或每隔几年上调起薪
# 仅 9% 的企业表示几乎不作调整
# 博士生最高（14.0%）
# 依次为硕士生（12.5%）
# 本科生（9.7%）
# 博士每年涨薪幅度数学期望 = 0.43 * 0.14 + 0.48 * (0.14/2) + 0.09 * 0 = 0.09
# 硕士每年涨薪幅度数学期望 = 0.43 * 0.13 + 0.48 * (0.13/2) + 0.09 * 0 = 0.08
# 本科每年涨薪幅度数学期望 = 0.43 * 0.1 + 0.48 * (0.1/2) + 0.09 * 0 = 0.07
benke_raise_rate = 0.07
shuoshi_raise_rate = 0.08
boshi_raise_rate = 0.09



def show_plot():
    x_data = [i for i in range(0,21)]

    benke_data = [calculate_salary(i,benke_start,benke_success_graduation_rate,benke_salary_month,0,benke_raise_rate) for i in x_data]
    shuoshi_data = [calculate_salary(i,shuoshi_start,shuo_success_graduation_rate,shuoshi_salary_month,1,shuoshi_raise_rate) for i in x_data]
    boshi_data = [calculate_salary(i,boshi_start,boshi_success_graduation_rate,boshi_salary_month,2,boshi_raise_rate) for i in x_data]


    # 创建主图形和主轴
    fig, ax1 = plt.subplots(sharex=True, sharey=True)
    annotation = ax1.annotate('', xy=(0, 0), xytext=(20, 20),
                          textcoords='offset points',
                          arrowprops=dict(arrowstyle='->'))
    # 绘制折线图
    plt.scatter(x_data,benke_data,color='blue')
    plt.plot(x_data, benke_data, color='blue', label='benke')
    plt.scatter(x_data,shuoshi_data,color='red')
    plt.plot(x_data, shuoshi_data, color='red', label='shuoshi')
    plt.scatter(x_data,boshi_data,color='green')
    plt.plot(x_data, boshi_data, color='green', label='boshi')

    # 设置坐标轴标签
    plt.xlabel('year')
    plt.ylabel('accumulation_salary(w)')

    # 显示图例
    plt.legend()

    # 显示刻度
    plt.xticks(range(0, int(max(x_data)) + 3, 1))
    plt.yticks(range(0, int(max(boshi_data)) + 1, 25))
    # 设置x轴和y轴的范围
    plt.xlim(0, max(x_data))
    plt.ylim(0, max(boshi_data)+80)

    plt.grid(color='grey', linestyle='--', linewidth=0.5)
    # 显示图形
    plt.show()

# 收益的数学期望  
def calculate_salary(year,start,graduation_rate,salary_month,delayed_year,raise_rate):
    return success_graduation_salary(year,start,salary_month,raise_rate) * graduation_rate + failure_graduation_salary(year,start,salary_month,delayed_year,raise_rate) * (1-graduation_rate)

## 正常毕业情况下，到达指定年数，所获收益
def success_graduation_salary(year,start,salary_month,raise_rate):
    auctual_year = year - start
    if(auctual_year <= 0):
        return 0
    _salary_month = salary_month
    _result = round( salary_month * 12 ,1)
    if(auctual_year == 1):
        return _result
    for i in range(2,auctual_year + 1):
        _salary_month = (1 + raise_rate) * _salary_month
        _result += round(salary_month * 12,1)
    return _result

## 延毕情况下，到达指定年数，所获收益
def failure_graduation_salary(year,start,salary_month,delayed_year,raise_rate):
    auctual_year = year - start- delayed_year
    if(auctual_year <= 0):
        return 0
    _salary_month = salary_month
    _result = round( salary_month * 12 ,1)
    if(auctual_year == 1):
        return _result
    for i in range(2,auctual_year + 1):
        _salary_month = (1 + raise_rate) * _salary_month
        _result += round( salary_month * 12,1)
    return _result


if __name__ == "__main__":
    show_plot()