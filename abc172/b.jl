s = readline()
t = readline()

ans = count(a -> a[1] != a[2], zip(s, t))
println(ans)
