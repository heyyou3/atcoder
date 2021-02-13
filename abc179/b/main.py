#!/usr/bin/env python
# -*- coding: utf-8 -*-

def resolve():
  n = int(input())
  cnt = 0
  for d in range(n):
    a, b = input().split(' ')
    if a == b:
      cnt += 1
      if cnt >= 3:
        break
    else:
      cnt = 0
  return "Yes" if cnt >= 3 else "No"

def main():
  print(resolve())

main()
