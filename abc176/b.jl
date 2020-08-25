function main()
  N = readline()
  total = 0
  for n in N
    total += parse(Int, n)
  end
  if total % 9 == 0 "Yes" else "No" end
end
println(main())
