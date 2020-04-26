$n = <>;
chomp($n);

my @input;

for (1..$n) {
  chomp(my $word = <>);
  push(@input, $word);
}
print(@input);
