#!/usr/bin/env perl
use strict;
use warnings;
use Test::More tests => 8;

use FindBin qw($Bin);
use lib "$Bin/../lib";
use Day12;

my $example1 = <<'END';
AAAA
BBCD
BBCC
EEEC
END

my $example2 = <<'END';
OOOOO
OXOXO
OOOOO
OXOXO
OOOOO
END

my $example3 = <<'END';
RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE
END

{
    open my $fh, '<', \$example1 or die $!;
    my $solution = Solution->new( fh => $fh );
    is $solution->part1(), 140, "Part1 example1";
}
{
    open my $fh, '<', \$example2 or die $!;
    my $solution = Solution->new( fh => $fh );
    is $solution->part1(), 772, "Part1 example2";
}
{
    open my $fh, '<', \$example3 or die $!;
    my $solution = Solution->new( fh => $fh );
    is $solution->part1(), 1930, "Part1 example3";
}

{
    open my $fh, '<', \$example1 or die $!;
    my $solution = Solution->new( fh => $fh );
    is $solution->part2(), 80, "Part2 example1";
}
{
    open my $fh, '<', \$example2 or die $!;
    my $solution = Solution->new( fh => $fh );
    is $solution->part2(), 436, "Part2 example2";
}
{
    open my $fh, '<', \$example3 or die $!;
    my $solution = Solution->new( fh => $fh );
    is $solution->part2(), 1206, "Part2 example3";
}
{
    open my $fh, '<', \<<'END' or die $!;
EEEEE
EXXXX
EEEEE
EXXXX
EEEEE
END
    my $solution = Solution->new( fh => $fh );
    is $solution->part2(), 236, "Part2 extra example 1";
}
{
    open my $fh, '<', \<<'END' or die $!;
AAAAAA
AAABBA
AAABBA
ABBAAA
ABBAAA
AAAAAA
END
    my $solution = Solution->new( fh => $fh );
    is $solution->part2(), 368, "Part2 extra example 2";
}
