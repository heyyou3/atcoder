use strict;
use warnings;
use List::Util qw(min);

chomp(my ($n, $k) = split(/ /, <>));

print min($n % $k, $k - ($n % $k))."\n";

