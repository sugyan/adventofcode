#!/usr/bin/env perl
use strict;
use warnings;
use Test::More tests => 13;

use FindBin qw($Bin);
use lib "$Bin/../lib";
use Day20;

my $solution = Solution->new( fh => *DATA );
is $solution->part1(), 0, "Part1";
is $solution->part2(), 0, "Part2";

# tests for 2 seconds cheat (x11)
is $solution->cheat_counts( 2, 64 ), 1;
is $solution->cheat_counts( 2, 40 ), 2;
is $solution->cheat_counts( 2, 38 ), 3;
is $solution->cheat_counts( 2, 36 ), 4;
is $solution->cheat_counts( 2, 20 ), 5;
is $solution->cheat_counts( 2, 12 ), 8;
is $solution->cheat_counts( 2, 10 ), 10;
is $solution->cheat_counts( 2, 8 ),  14;
is $solution->cheat_counts( 2, 6 ),  16;
is $solution->cheat_counts( 2, 4 ),  30;
is $solution->cheat_counts( 2, 2 ),  44;

__DATA__
###############
#...#...#.....#
#.#.#.#.#.###.#
#S#...#.#.#...#
#######.#.#.###
#######.#.#...#
#######.#.###.#
###..E#...#...#
###.#######.###
#...###...#...#
#.#####.#.###.#
#.#...#.#.#...#
#.#.#.#.#.#.###
#...#...#...###
###############
