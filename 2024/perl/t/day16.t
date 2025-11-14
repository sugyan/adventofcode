#!/usr/bin/env perl
use v5.38;
use Test2::V0;

use FindBin qw($Bin);
use lib "$Bin/../lib";
use Day16;

my $example1 = <<'END';
###############
#.......#....E#
#.#.###.#.###.#
#.....#.#...#.#
#.###.#####.#.#
#.#.#.......#.#
#.#.#####.###.#
#...........#.#
###.#.#####.#.#
#...#.....#.#.#
#.#.#.###.#.#.#
#.....#...#.#.#
#.###.#.#.#.#.#
#S..#.....#...#
###############
END

my $example2 = <<'END';
#################
#...#...#...#..E#
#.#.#.#.#.#.#.#.#
#.#.#.#...#...#.#
#.#.#.#.###.#.#.#
#...#.#.#.....#.#
#.#.#.#.#.#####.#
#.#...#.#.#.....#
#.#.#####.#.###.#
#.#.#.......#...#
#.#.###.#####.###
#.#.#...#.....#.#
#.#.#.#####.###.#
#.#.#.........#.#
#.#.#.#########.#
#S#.............#
#################
END

{
    open my $fh, '<', \$example1 or die $!;
    my $solution = Solution->new( fh => $fh );
    is $solution->part1(), 7036, "Part1 example1";
}
{
    open my $fh, '<', \$example2 or die $!;
    my $solution = Solution->new( fh => $fh );
    is $solution->part1(), 11048, "Part1 example2";
}

{
    open my $fh, '<', \$example1 or die $!;
    my $solution = Solution->new( fh => $fh );
    is $solution->part2(), 45, "Part2 example1";
}
{
    open my $fh, '<', \$example2 or die $!;
    my $solution = Solution->new( fh => $fh );
    is $solution->part2(), 64, "Part2 example2";
}

done_testing();
