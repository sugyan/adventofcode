#!/usr/bin/env perl
use strict;
use warnings;
use Test::More tests => 2;

use FindBin qw($Bin);
use lib "$Bin/../lib";
use Day01;

my $solution = Solution->new( fh => *DATA );
is $solution->part1(), 11, "Part1";
is $solution->part2(), 31, "Part2";

__DATA__
3   4
4   3
2   5
1   3
3   9
3   3
