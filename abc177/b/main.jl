parse_input(type=String, x=readline()) = begin
  type == String && return x
  parse(type, x)
end
parse_inputs(type=String) = parse_input.(type, split(readline()))

function solve()
  S = parse_input()
  T = parse_input()
  smax = 0
  for i in 1:length(S)-length(T)+1
    s = 0
    for j in 1:length(T)
      if S[i+j-1] == T[j]
        s += 1
      end
    end
    if smax < s
      smax = s
    end
  end
  length(T) - smax
end

println(solve())
