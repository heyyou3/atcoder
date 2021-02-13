#!/usr/bin/env python
# -*- coding: utf-8 -*-

def resolve():
  n = int(input())
  ans = 0
  for a in range(1, n):
    ans += (n - 1) // a
  return ans

def main():
  print(resolve())

main()
