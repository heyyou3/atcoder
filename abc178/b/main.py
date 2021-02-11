#!/usr/bin/env python
# -*- coding: utf-8 -*-

def resolve():
  a, b, c, d = [int(x) for x in input().split(' ')]
  return max(a*c, a*d, b*c, b*d)

def main():
  print(resolve())

main()
