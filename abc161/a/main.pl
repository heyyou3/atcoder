use strict;
use warnings;

chomp(my $xyz = <>);
my ($x, $y, $z) = split(/ /, $xyz);
print "$z $x $y \n";

