parse_int(x) = parse(Int, x)
function main()
  n = parse_int(readline())
  A = parse_int.(split(readline()))
  res = map(enumerate(A)) do (i, a)
    if i == 1 return 0 end
    if a >= A[i-1]
      return 0
    else
      ans = A[i-1] - a
      A[i] = A[i] + ans
      return ans
    end
  end
  sum(res)
end
println(main())
