use 5.38.0;
use feature 'class';
no warnings 'experimental::class';

use Base;

class Solution : isa(Base) {
    use List::Util qw(sum);

    my @DIR = ( [ -1, 0 ], [ 0, 1 ], [ 1, 0 ], [ 0, -1 ] );

    method parse($lines) {
        my %garden_plots = ();
        for my $i ( 0 .. $lines->$#* ) {
            my @s = split //, $lines->[$i];
            for my $j ( 0 .. $#s ) {
                $garden_plots{ key( $i, $j ) } = $s[$j];
            }
        }
        return \%garden_plots;
    }

    method part1() {
        my $input = $self->input();
        return sum map { $_->@* * perimeter( $input, $_ ) } areas($input)->@*;
    }

    method part2() {
        my $input = $self->input();
        return sum map { $_->@* * sides( $input, $_ ) } areas($input)->@*;
    }

    sub key( $i, $j ) {
        return "$i,$j";
    }

    sub areas($map) {
        my @areas = ();
        my $seen  = {};
        for my $pos ( keys $map->%* ) {
            next if $seen->{$pos};
            my ( $i, $j ) = split /,/, $pos;
            push @areas, dfs( $map, $i, $j, $seen );
        }
        return \@areas;
    }

    sub dfs( $map, $i, $j, $seen ) {
        my $c     = $map->{ key( $i, $j ) };
        my @stack = ( [ $i, $j ] );
        my @area  = ();
        while (@stack) {
            my ( $i, $j ) = ( pop @stack )->@*;
            my $key = key( $i, $j );
            next if $seen->{$key};
            $seen->{$key} = 1;
            push @area, [ $i, $j ];
            for my $d (@DIR) {
                my ( $ni, $nj ) = ( $i + $d->[0], $j + $d->[1] );
                next if ( $map->{ key( $ni, $nj ) } // "" ) ne $c;
                push @stack, [ $ni, $nj ];
            }
        }
        return \@area;
    }

    sub perimeter( $map, $area ) {
        my $sum = 0;
        for my $p ( $area->@* ) {
            my ( $i, $j ) = $p->@*;
            my $c = $map->{ key( $i, $j ) };
            for my $d (@DIR) {
                my ( $ni, $nj ) = ( $i + $d->[0], $j + $d->[1] );
                $sum++ if ( $map->{ key( $ni, $nj ) } // "" ) ne $c;
            }
        }
        return $sum;
    }

    sub sides( $map, $area ) {
        my %corners = ();
        my %set     = map { key( $_->@* ) => 1 } $area->@*;
        for my $p ( $area->@* ) {
            my ( $i, $j ) = $p->@*;
            for my $q (
                [ $i - 1, $j - 1 ],
                [ $i - 1, $j - 0 ],
                [ $i - 0, $j - 1 ],
                [ $i - 0, $j - 0 ]
              )
            {
                my ( $ni, $nj ) = $q->@*;
                my $key = key( $ni, $nj );
                next if exists $corners{$key};
                my @mask = map { exists $set{ key( $_->@* ) } } (
                    [ $ni + 0, $nj + 0 ],
                    [ $ni + 0, $nj + 1 ],
                    [ $ni + 1, $nj + 0 ],
                    [ $ni + 1, $nj + 1 ],
                );
                my $count = grep { $_ } @mask;
                if ( $count == 1 || $count == 3 ) {
                    $corners{$key} = 1;
                }
                elsif ( $count == 2 && $mask[0] == $mask[3] ) {
                    $corners{$key} = 2;
                }
                else {
                    $corners{$key} = 0;
                }

            }
        }
        return sum values %corners;
    }
}
