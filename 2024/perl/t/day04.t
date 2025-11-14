#!/usr/bin/env perl
use v5.38;
use Test2::V0;

use FindBin qw($Bin);
use lib "$Bin/../lib";
use Day04;

my $solution = Solution->new( fh => *DATA );
is $solution->part1(), 18, "Part1";
is $solution->part2(), 9,  "Part2";

done_testing();

__DATA__
MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX
