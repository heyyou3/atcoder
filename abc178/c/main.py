#!/usr/bin/env python
# -*- coding: utf-8 -*-

def resolve():
  n = int(input())
  ans = (10**n-2*9**n+8**n)%(10**9+7)
  return ans

def main():
  print(resolve())

main()
