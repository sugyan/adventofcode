#!/usr/bin/env perl
use strict;
use warnings;
use Test::More tests => 2;

use FindBin qw($Bin);
use lib "$Bin/../lib";
use Day19;

my $solution = Solution->new( fh => *DATA );
is $solution->part1(), 6,  "Part1";
is $solution->part2(), 16, "Part2";

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
