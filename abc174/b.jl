function main()
  n, d = parse.(Int, split(readline()))
  count = 0
  for _ = 1:n
    x, y = parse.(Int, split(readline()))
    if sqrt((x^2) + (y^2)) <= d
      count += 1
    end
  end
  count
end
println(main())
