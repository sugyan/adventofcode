#!/usr/bin/env perl
use strict;
use warnings;
use Test::More tests => 2;

use FindBin qw($Bin);
use lib "$Bin/../lib";
use Day09;

my $solution = Solution->new( fh => *DATA );
is $solution->part1(), 1928, "Part1";
is $solution->part2(), 2858, "Part2";

__DATA__
2333133121414131402
