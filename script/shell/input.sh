#!/bin/bash

# 无限循环执行
while true
do
    # 记录日志 - 模拟点击坐标 (400, 1800)
    echo "[$(date)] Clicking at (400, 1800)"
    adb shell input tap 400 1800
    sleep 1
    echo "[$(date)] Clicking at (400, 1800)"    
    adb shell input tap 400 1800
    # 等待3秒
    sleep 3

    # 记录日志 - 模拟点击坐标 (100, 180)
    echo "[$(date)] Clicking at (100, 180)"
    adb shell input tap 100 180
    # 等待3秒
    sleep 3

    # 记录日志 - 模拟点击坐标 (300, 1920)
    echo "[$(date)] Clicking at (300, 1920)"
    adb shell input tap 300 2000
    # 等待3秒
    sleep 3

    # 记录日志 - 模拟点击坐标 (600, 2000)
    echo "[$(date)] Clicking at (600, 2000)"
    adb shell input tap 600 2000
    # 等待3秒
    sleep 5
done

