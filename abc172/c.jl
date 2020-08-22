function main()
  n, m, k = parse.(Int, split(readline()))
  a = parse.(Int, split(readline()))
  b = parse.(Int, split(readline()))

  t = sum(b)
  j = m
  ans = 0
  for i in 0:n
    while j > 0 && t > k
      j -= 1
      t -= b[j+1]
    end
    t > k && break
    ans = max(ans, i+j)
    i == n && break
    t += a[i+1]
  end
  ans
end

println(main())
