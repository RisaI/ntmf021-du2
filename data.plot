reset

f(x) = A * x**B
g(x) = C * x**D

fit f(x) "data.dat" using 1:2 via A, B
fit g(x) "data.dat" using 1:3 via C, D

print sprintf("Walk : A = %.3f, B = %.5f", A, B)
print sprintf("Noret: A = %.3f, B = %.5f", C, D)

plot "data.dat" using 1:2 with points title "Walk",\
     f(x) title "Walk Fit",\
     "data.dat" using 1:3 with points title "No return",\
     g(x) title "No return Fit"
