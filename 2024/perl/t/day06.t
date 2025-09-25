#!/usr/bin/env perl
use strict;
use warnings;
use Test::More tests => 2;

use FindBin qw($Bin);
use lib "$Bin/../lib";
use Day06;

my $solution = Solution->new( fh => *DATA );
is $solution->part1(), 41, "Part1";
is $solution->part2(), 6,  "Part2";

__DATA__
....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...
