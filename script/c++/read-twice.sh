#!/bin/bash
# 结合 sar -B 1 命令可以采集文件内容读入页面缓存时，总共发生多少次页面调入？
rm -f testfile
echo "$(date):start file creation!"
dd if=/dev/zero of=testfile oflag=direct bs=1M count=1K
echo "$(date):end file creation!"
echo "$(date):start 3s sleep"
sleep 3
echo "$(date):end 3s sleep"

echo "$(date):start 1st read"
cat testfile >/dev/null 
echo "$(date):end 1st read"

echo "$(date):start 3s sleep"
sleep 3
echo "$(date):end 3s sleep"

echo "$(date):start 2nd read"
cat testfile >/dev/null 
echo "$(date):end 2nd read"
rm -f testfile
