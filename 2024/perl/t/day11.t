#!/usr/bin/env perl
use strict;
use warnings;
use Test::More tests => 1;

use FindBin qw($Bin);
use lib "$Bin/../lib";
use Day11;

my $solution = Solution->new( fh => *DATA );
is $solution->part1(), 55312, "Part1";

__DATA__
125 17
