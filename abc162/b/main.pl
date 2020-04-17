use strict;
use warnings;
use List::Util qw(sum);

chomp(my $n = <>);

my @nums;

for my $i (1..$n) {
  if ($i%15 == 0) {
    next;
  }

  if ($i%5 == 0) {
    next;
  }

  if ($i%3 == 0) {
    next;
  }

  push(@nums, $i);
}

print sum(@nums)."\n";

