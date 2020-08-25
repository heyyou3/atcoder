function main()
  n, x, t = parse.(Int, split(readline()))
  cld(n, x) * t
end
println(main())
