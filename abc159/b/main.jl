parse_input(type=String, x=readline()) = begin
  type == String && return x
  parse(type, x)
end
parse_inputs(type=String) = parse_input.(type, split(readline()))

function main()
  s = parse_input()
  isequal(s, reverse(s)) && isequal(s[1:div(end-1, 2)], reverse(s[1:div(end-1, 2)])) && return "Yes"
  "No"
end

println(main())
