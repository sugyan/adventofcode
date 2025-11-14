#!/usr/bin/env perl
use v5.38;
use Test2::V0;

use FindBin qw($Bin);
use lib "$Bin/../lib";
use Day06;

my $solution = Solution->new( fh => *DATA );
is $solution->part1(), 41, "Part1";
is $solution->part2(), 6,  "Part2";

done_testing();

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
