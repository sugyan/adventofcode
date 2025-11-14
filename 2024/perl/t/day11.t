#!/usr/bin/env perl
use v5.38;
use Test::More tests => 1;

use FindBin qw($Bin);
use lib "$Bin/../lib";
use Day11;

my $solution = Solution->new( fh => *DATA );
is $solution->part1(), 55312, "Part1";

done_testing();

__DATA__
125 17
