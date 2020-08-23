function main()
  k = parse(Int, readline())
  if k % 2 == 0 return -1 end
  count = 1
  now = mod(7, k)
  while now != 0
    count += 1
    now = mod(7 + now*10, k)
    if count > 10^6; count = -1; break; end
  end
  count
end

println(main())
