#!/usr/bin/env perl
use v5.38;
use Test2::V0;

use FindBin qw($Bin);
use lib "$Bin/../lib";
use Day17;

my $example1 = <<'END';
Register A: 729
Register B: 0
Register C: 0

Program: 0,1,5,4,3,0
END

my $example2 = <<'END';
Register A: 2024
Register B: 0
Register C: 0

Program: 0,3,5,4,3,0
END

{
    open my $fh, '<', \$example1 or die $!;
    my $solution = Solution->new( fh => $fh );
    is $solution->part1(), "4,6,3,5,6,3,5,2,1,0", "Part1";
}
{
    open my $fh, '<', \$example2 or die $!;
    my $solution = Solution->new( fh => $fh );
    is $solution->part2(), 117440, "Part2";
}

done_testing();
