#!/usr/bin/env python
# -*- coding: utf-8 -*-

def resolve():
  s = int(input())
  a = [0]*2010
  a[0], a[1], a[2] = 1, 0, 0
  i = 3
  while i <= s:
    a[i] = (a[i-1] + a[i-3])%(10**9+7)
    i += 1
  return a[s]

def main():
  print(resolve())

main()
