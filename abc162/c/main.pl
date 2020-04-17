use strict;
use warnings;
use v5.18.0;
use List::Util qw(sum);

chomp(my $arg = <>);

my @res = ();
my $sum;

sub gcd {
  my ($a, $b) = @_;
  return ($b == 0) ? $a : gcd($b, $a % $b);
}

for my $a (1..$arg) {
  for my $b (1..$arg) {
    my $tmp = ($res[$a][$b]) ? $res[$a][$b] : gcd($a, $b);
    $res[$a][$b] = $tmp;
    $res[$b][$a] = $tmp;

    for my $c (1..$arg) {
      my $tmp2 = ($res[$tmp][$c]) ? $res[$tmp][$c] : gcd($tmp, $c);
      $res[$tmp][$c] = $tmp2;
      $res[$c][$tmp] = $tmp2;
      $sum += $tmp2;
    }
  }
}

print $sum."\n";

