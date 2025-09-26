#!/usr/bin/env perl
use strict;
use warnings;
use Test::More tests => 2;

use FindBin qw($Bin);
use lib "$Bin/../lib";
use Day07;

my $solution = Solution->new( fh => *DATA );
is $solution->part1(), 3749,  "Part1";
is $solution->part2(), 11387, "Part2";

__DATA__
190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20
