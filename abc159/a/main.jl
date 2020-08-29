parse_input(type=String, x=readline()) = begin
  type == String && return x
  parse(type, x)
end
parse_inputs(type=String) = parse_input.(type, split(readline()))

function main()
  n, m = parse_inputs(Int)
  Int(n*(n-1)/2 + m*(m-1)/2)
end

println(main())
