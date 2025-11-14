#!/usr/bin/env perl
use v5.38;
use Test2::V0;

use FindBin qw($Bin);
use lib "$Bin/../lib";
use Day19;

my $solution = Solution->new( fh => *DATA );
is $solution->part1(), 6,  "Part1";
is $solution->part2(), 16, "Part2";

done_testing();

__DATA__
r, wr, b, g, bwu, rb, gb, br

brwrr
bggr
gbbr
rrbgbr
ubwu
bwurrg
brgr
bbrgwb
