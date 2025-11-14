#!/usr/bin/env perl
use v5.38;
use Test2::V0;

use FindBin qw($Bin);
use lib "$Bin/../lib";
use Day10;

my $solution = Solution->new( fh => *DATA );
is $solution->part1(), 36, "Part1";
is $solution->part2(), 81, "Part2";

done_testing();

__DATA__
89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732
