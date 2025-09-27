use 5.38.0;
use feature 'class';
no warnings 'experimental::class';

use Base;

class Coord {
    field $x : param : reader;
    field $y : param : reader;

    method add($other) {
        return Coord->new(
            x => $self->x + $other->x,
            y => $self->y + $other->y,
        );
    }

    method sub($other) {
        return Coord->new(
            x => $self->x - $other->x,
            y => $self->y - $other->y,
        );
    }

    method in_range( $xmin, $xmax, $ymin, $ymax ) {
        return
             $self->x >= $xmin
          && $self->x <= $xmax
          && $self->y >= $ymin
          && $self->y <= $ymax;
    }

    method to_string() {
        return join( ',', $self->x, $self->y );
    }
}

class Solution : isa(Base) {

    method parse($lines) {
        my $grid     = [ map { [ split //, $_ ] } $lines->@* ];
        my $antennas = {};
        for my $i ( 0 .. $grid->$#* ) {
            for my $j ( 0 .. $grid->[$i]->$#* ) {
                my $c = $grid->[$i][$j];
                if ( $c ne '.' ) {
                    push(
                        ( $antennas->{$c} //= [] )->@*,
                        Coord->new( x => $i, y => $j )
                    );
                }
            }
        }
        return {
            antennas => $antennas,
            map_size => [ $grid->$#*, $grid->[0]->$#* ],
        };
    }

    method part1() {
        my $input = $self->input();
        return count_antinodes( $input->{antennas}, $input->{map_size}, 0 );
    }

    method part2() {
        my $input = $self->input();
        return count_antinodes( $input->{antennas}, $input->{map_size}, 1 );
    }

    sub count_antinodes( $antennas, $map_size, $all_harmonics ) {
        my %seen = ();
        for my $coords ( values $antennas->%* ) {
            for my $pair ( combinations2( scalar $coords->@* ) ) {
                my $p0 = $coords->[ $pair->[0] ];
                my $p1 = $coords->[ $pair->[1] ];
                for my $candidates (
                    calculate_coords( $p0, $p0->sub($p1), $map_size ),
                    calculate_coords( $p1, $p1->sub($p0), $map_size ),
                  )
                {
                    if ( !$all_harmonics ) {
                        $candidates = [ $candidates->[1] // () ];
                    }
                    $seen{ $_->to_string() } = 1 for $candidates->@*;
                }
            }
        }
        return scalar keys %seen;
    }

    sub calculate_coords( $start, $d, $map_size ) {
        my @results = ();
        my $p       = $start;
        while ( $p->in_range( 0, $map_size->[0], 0, $map_size->[1] ) ) {
            push @results, $p;
            $p = $p->add($d);
        }
        return \@results;
    }

    sub combinations2($n) {
        my @result;
        for my $i ( 0 .. $n - 2 ) {
            for my $j ( $i + 1 .. $n - 1 ) {
                push @result, [ $i, $j ];
            }
        }
        return @result;
    }
}
