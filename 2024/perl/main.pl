#!/usr/bin/env perl
use 5.38.0;

use FindBin qw($Bin);
use lib "$Bin/lib";

my $day = shift // die "Usage: $0 dayXX\n";
$day =~ /\Aday(\d{2})\z/ or die "Invalid day format. Use dayXX.\n";

require "Day$1.pm";
my $solution = Solution->new( fh => *STDIN );
say "Part 1: ", $solution->part1();
say "Part 2: ", $solution->part2();
