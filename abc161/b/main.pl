use strict;
use warnings;
use List::Util qw(sum);

chomp(my $nm = <>);
my ($n, $m) = split(/ /, $nm);

chomp(my $input_a = <>);
my @a = split(/ /, $input_a);
@a = sort {$b <=> $a} @a;

# 総投票数
my $sum = sum(@a);

if ($a[$m-1] >= $sum/(4*$m)) {
  print "Yes\n";
} else {
  print "No\n";
}

