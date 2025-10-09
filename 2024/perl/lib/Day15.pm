use 5.38.0;
use feature 'class';
no warnings 'experimental::class';

use Base;

class Solution : isa(Base) {

    method parse($lines) {
        my ( $map, $moves ) = split /\n\n/, join "\n", $lines->@*;
        return {
            warehouse => [ split /\n/, $map ],
            moves     => $moves =~ s/\s+//gr
        };
    }

    method part1() {
        my $input     = $self->input();
        my $warehouse = $input->{warehouse};
        return sum_of_coordinates( $warehouse, $input->{moves} );
    }

    method part2() {
        my $input     = $self->input();
        my $warehouse = [
            map {
                $_ =~ s/([#.])/$1$1/g;
                $_ =~ s/O/[]/g;
                $_ =~ s/@/@./;
                $_;
            } $input->{warehouse}->@*
        ];
        return sum_of_coordinates( $warehouse, $input->{moves} );
    }

    sub sum_of_coordinates( $warehouse, $moves ) {
        my $grid  = [ map { [ split //, $_ ] } $warehouse->@* ];
        my $robot = [ 0, 0 ];
      ROBOT:
        for my $i ( 0 .. $grid->$#* ) {
            for my $j ( 0 .. $grid->[$i]->$#* ) {
                if ( $grid->[$i][$j] eq '@' ) {
                    $robot = [ $i, $j ];
                    last ROBOT;
                }
            }
        }
        for my $move ( split //, $moves ) {
            if ( try_move( $grid, $robot->@*, $move ) ) {
                $robot = next_pos( $robot->@*, $move );
            }
        }
        my $sum = 0;
        for my $i ( 0 .. $grid->$#* ) {
            for my $j ( 0 .. $grid->[$i]->$#* ) {
                if ( $grid->[$i][$j] eq 'O' || $grid->[$i][$j] eq '[' ) {
                    $sum += $i * 100 + $j;
                }
            }
        }
        return $sum;
    }

    sub try_move( $grid, $i, $j, $move ) {
        my @stack = ( [ $i, $j ] );
        my %seen  = ();
        while (@stack) {
            my ( $ci, $cj ) = ( pop @stack )->@*;
            if ( $seen{"$ci,$cj"}++ ) {
                next;
            }
            my ( $ni, $nj ) = next_pos( $ci, $cj, $move )->@*;
            if ( $grid->[$ni][$nj] eq '#' ) {
                return 0;
            }
            elsif ( $grid->[$ni][$nj] eq 'O' ) {
                push @stack, [ $ni, $nj ];
            }
            elsif ( $grid->[$ni][$nj] eq '[' ) {
                push @stack, [ $ni, $nj ], [ $ni, $nj + 1 ];
            }
            elsif ( $grid->[$ni][$nj] eq ']' ) {
                push @stack, [ $ni, $nj ], [ $ni, $nj - 1 ];
            }
        }
        my @moves = map { [ split /,/, $_ ] } keys %seen;
        for my $p ( sort { sort_by( $a, $b, $move ) } @moves ) {
            my ( $ci, $cj ) = $p->@*;
            my ( $ni, $nj ) = next_pos( $ci, $cj, $move )->@*;
            $grid->[$ni][$nj] = $grid->[$ci][$cj];
            $grid->[$ci][$cj] = '.';
        }
        return 1;
    }

    sub next_pos( $i, $j, $move ) {
        if ( $move eq '^' ) { return [ $i - 1, $j + 0 ]; }
        if ( $move eq 'v' ) { return [ $i + 1, $j + 0 ]; }
        if ( $move eq '<' ) { return [ $i + 0, $j - 1 ]; }
        if ( $move eq '>' ) { return [ $i + 0, $j + 1 ]; }
        die "Invalid move: $move";
    }

    sub sort_by( $a, $b, $move ) {
        if ( $move eq '^' ) { return $a->[0] <=> $b->[0]; }
        if ( $move eq 'v' ) { return $b->[0] <=> $a->[0]; }
        if ( $move eq '<' ) { return $a->[1] <=> $b->[1]; }
        if ( $move eq '>' ) { return $b->[1] <=> $a->[1]; }
        die "Invalid move: $move";
    }
}
