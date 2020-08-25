function main()
  S = split(readline(), 'S')
  ans = 0

  for s = S
    ans = max(ans, length(s))
  end

  ans
end
println(main())
