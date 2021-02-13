#!/usr/bin/env python
# -*- coding: utf-8 -*-

def resolve():
  s = input()
  if s[-1] == "s":
    return s+"es"
  else:
    return s+"s"

def main():
  print(resolve())

main()
