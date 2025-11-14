#!/usr/bin/env perl
use v5.38;
use Test2::V0;

use FindBin qw($Bin);
use lib "$Bin/../lib";
use Day02;

my $solution = Solution->new( fh => *DATA );
is $solution->part1(), 2, "Part1";
is $solution->part2(), 4, "Part2";

done_testing();

__DATA__
7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9
