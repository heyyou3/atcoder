parse_input(type=String, x=readline()) = begin
  type == String && return x
  parse(type, x)
end
parse_inputs(type=String) = parse_input.(type, split(readline()))

function solve()
  _ = parse_input()
  A = parse_inputs(BigInt)
  end_a = A[end]
  ans = 0
  for a in A[end-1:-1:1]
    ans += a*end_a
    end_a += a
  end
  mod(ans, (10^9)+7)
end

println(solve())
