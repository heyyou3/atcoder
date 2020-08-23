function main()
  n = parse(Int, readline())
  res = n % 1000
  if res != 0 return 1000 - res end
  res
end

println(main())
