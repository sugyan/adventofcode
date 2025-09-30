use 5.38.0;
use feature 'class';
no warnings 'experimental::class';

use Base;

class Solution : isa(Base) {
    use List::Util qw(sum);

    my @DIR = ( [ -1, 0 ], [ 0, 1 ], [ 1, 0 ], [ 0, -1 ] );

    method parse($lines) {
        my %topographic_map = ();
        for my $i ( 0 .. $lines->$#* ) {
            my @s = split //, $lines->[$i];
            for my $j ( 0 .. $#s ) {
                $topographic_map{"$i,$j"} = $s[$j];
            }
        }
        return \%topographic_map;
    }

    method part1() {
        my $input = $self->input();
        return sum map { dfs( $input, $_, {}, 1, 0 ) } keys $input->%*;
    }

    method part2() {
        my $input = $self->input();
        return sum map { dfs( $input, $_, {}, 0, 0 ) } keys $input->%*;
    }

    sub dfs( $map, $pos, $visited, $mark_visited, $h ) {
        return 0 if ( $map->{$pos} // -1 ) != $h || $visited->{$pos};
        $visited->{$pos} = 1 if $mark_visited;
        my ( $i, $j ) = split /,/, $pos;
        return ( $map->{$pos} == 9 ) + sum map {
            my ( $ni, $nj ) = ( $i + $_->[0], $j + $_->[1] );
            dfs( $map, "$ni,$nj", $visited, $mark_visited, $h + 1 )
        } @DIR;
    }
}
