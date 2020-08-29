using Printf
parse_input(type=String, x=readline()) = begin
  type == String && return x
  parse(type, x)
end
parse_inputs(type=String) = parse_input.(type, split(readline()))

function main()
  l = parse_input(Int)
  @sprintf("%6.12f", (l^3)/27)
end

println(main())
