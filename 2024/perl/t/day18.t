#!/usr/bin/env perl
use v5.38;
use Test2::V0;

BEGIN { $ENV{EXAMPLE_MODE} = 1; }
use FindBin qw($Bin);
use lib "$Bin/../lib";
use Day18;

my $solution = Solution->new( fh => *DATA );
is $solution->part1(), 22,    "Part1";
is $solution->part2(), "6,1", "Part2";

done_testing();

__DATA__
5,4
4,2
4,5
3,0
2,1
6,3
2,4
1,5
0,6
3,3
2,6
5,1
1,2
5,5
2,5
6,5
1,4
0,4
6,4
1,1
6,1
1,0
0,5
1,6
2,0
