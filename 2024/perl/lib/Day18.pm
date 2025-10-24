use 5.38.0;
use feature 'class';
no warnings 'experimental::class';

use Base;

class Solution : isa(Base) {
    use constant {
        SIZE             => ( $ENV{EXAMPLE_MODE} ? 7  : 71 ),
        FIRST_SOME_BYTES => ( $ENV{EXAMPLE_MODE} ? 12 : 1024 ),
    };

    my @DIR = ( [ -1, 0 ], [ 0, 1 ], [ 1, 0 ], [ 0, -1 ] );

    method parse($lines) {
        my %space = ();
        my @bytes = ();
        for my $i ( 0 .. SIZE - 1 ) {
            for my $j ( 0 .. SIZE - 1 ) {
                $space{"$i,$j"} = scalar $lines->@*;
            }
        }
        for my $t ( 0 .. $lines->$#* ) {
            my ( $i, $j ) = split /,/, $lines->[$t];
            $space{"$i,$j"} = $t;
            push @bytes, "$i,$j";
        }
        return {
            space => \%space,
            bytes => \@bytes,
        };
    }

    method part1() {
        my $input = $self->input();
        return bfs( $input->{space}, FIRST_SOME_BYTES );
    }

    method part2() {
        my $input = $self->input();
        my ( $lo, $hi ) = ( 0, scalar $input->{bytes}->@* );
        while ( $lo + 1 < $hi ) {
            my $mid = int( ( $lo + $hi ) / 2 );
            if ( defined bfs( $input->{space}, $mid ) ) {
                $lo = $mid;
            }
            else {
                $hi = $mid;
            }
        }
        return $input->{bytes}[$lo];
    }

    sub bfs( $space, $threshold ) {
        my %dist = ();
        my @q    = ( [ 0, 0 ] );
        $dist{"0,0"} = 0;
        while (@q) {
            my ( $i, $j ) = @{ shift @q };
            for my $d (@DIR) {
                my ( $ni, $nj ) = ( $i + $d->[0], $j + $d->[1] );
                if (  !exists $dist{"$ni,$nj"}
                    && exists $space->{"$ni,$nj"}
                    && $space->{"$ni,$nj"} >= $threshold )
                {
                    $dist{"$ni,$nj"} = $dist{"$i,$j"} + 1;
                    push @q, [ $ni, $nj ];
                }
            }
        }
        my $exit = join ',', ( SIZE - 1, SIZE - 1 );
        return $dist{$exit};
    }
}
