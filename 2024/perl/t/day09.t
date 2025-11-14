#!/usr/bin/env perl
use v5.38;
use Test2::V0;

use FindBin qw($Bin);
use lib "$Bin/../lib";
use Day09;

my $solution = Solution->new( fh => *DATA );
is $solution->part1(), 1928, "Part1";
is $solution->part2(), 2858, "Part2";

done_testing();

__DATA__
2333133121414131402
