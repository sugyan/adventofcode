use 5.38.0;
use feature 'class';
no warnings 'experimental::class';

use Base;

class Solution : isa(Base) {
    my @DIR = ( [ -1, 0 ], [ 0, 1 ], [ 1, 0 ], [ 0, -1 ] );

    method parse($lines) {
        my $area = [ map { [ split // ] } $lines->@* ];
        my ( $map, $guard ) = ( {}, [] );
        for my $i ( 0 .. $#$area ) {
            for my $j ( 0 .. $#{ $area->[$i] } ) {
                $map->{"$i,$j"} = $area->[$i][$j] eq '#';
                if ( $area->[$i][$j] eq '^' ) {
                    $guard = [ $i, $j, 0 ];
                }
            }
        }
        return { map => $map, guard => $guard };
    }

    method part1() {
        my $input = $self->input();
        return scalar distinct_positions( $input->{map}, $input->{guard}->@* );
    }

    method part2() {
        my $input = $self->input();
        my %map   = $input->{map}->%*;
        return scalar grep {
            $map{$_} = 1;
            my $result = will_stuck_in_loop( \%map, $input->{guard}->@* );
            $map{$_} = 0;
            $result;
        } distinct_positions( $input->{map}, $input->{guard}->@* );
    }

    sub distinct_positions( $map, $i, $j, $d ) {
        my %path = ( "$i,$j" => 1 );
        while ( my @next = next_position( $map, $i, $j, $d ) ) {
            ( $i, $j, $d ) = @next;
            $path{"$i,$j"} = 1;
        }
        return keys %path;
    }

    sub will_stuck_in_loop( $map, $i, $j, $d ) {
        my %seen = ( "$i,$j,$d" => 1 );
        while ( my @next = next_position( $map, $i, $j, $d ) ) {
            ( $i, $j, $d ) = @next;
            return 1 if exists $seen{"$i,$j,$d"};
            $seen{"$i,$j,$d"} = 1;
        }
        return 0;
    }

    sub next_position( $map, $i, $j, $d ) {
        my ( $di, $dj ) = $DIR[$d]->@*;
        my ( $ni, $nj ) = ( $i + $di, $j + $dj );
        if ( !exists $map->{"$ni,$nj"} ) {
            return;
        }
        if ( $map->{"$ni,$nj"} ) {
            return $i, $j, ( $d + 1 ) % 4;
        }
        else {
            return $ni, $nj, $d;
        }
    }
}
