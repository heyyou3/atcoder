parse_input(type=String, x=readline()) = begin
  type == String && return x
  parse(type, x)
end
parse_inputs(type=String) = parse_input.(type, split(readline()))

function solve()
  d, t, s = parse_inputs(Int)
  d/s <= t && return "Yes"
  "No"
end

println(solve())
