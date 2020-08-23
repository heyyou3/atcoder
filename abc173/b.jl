function solve()
  n = parse(Int, readline())
  ac, wa, tle, re = 0, 0, 0, 0
  for i = 1:n
    s = readline()
    if s == "AC"
      ac += 1
    elseif s == "WA"
      wa += 1
    elseif s == "TLE"
      tle += 1
    elseif s == "RE"
      re += 1
    end
  end
  Dict("AC" => ac, "WA" => wa, "TLE" => tle, "RE" => re)
end

ans = solve()
println("AC x $(ans["AC"])")
println("WA x $(ans["WA"])")
println("TLE x $(ans["TLE"])")
println("RE x $(ans["RE"])")
