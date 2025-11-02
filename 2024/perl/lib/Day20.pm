use 5.38.0;
use feature 'class';
no warnings 'experimental::class';

use Base;

class Solution : isa(Base) {

    method parse($lines) {
        my $start     = [ 0, 0 ];
        my %racetrack = ();
        for my $i ( 0 .. $lines->$#* ) {
            my @s = split //, $lines->[$i];
            for my $j ( 0 .. $#s ) {
                if ( $s[$j] eq 'S' ) {
                    $start = [ $i, $j ];
                }
                elsif ( $s[$j] eq '#' ) {
                    next;
                }
                $racetrack{"$i,$j"} = $s[$j];
            }
        }
        return {
            racetrack => \%racetrack,
            start     => $start,
        };
    }

    method part1() {
        return $self->cheat_counts( 2, 100 );
    }

    method part2() {
        return $self->cheat_counts( 20, 100 );
    }

    method cheat_counts( $seconds, $threshold ) {
        my $input = $self->input();
        my $dist  = bfs( $input->{racetrack}, $input->{start} );
        my $count = 0;
        while ( my ( $key, $d ) = each $dist->%* ) {
            my ( $i, $j ) = split /,/, $key, 2;
            for my $di ( -$seconds .. $seconds ) {
                my $r = $seconds - abs($di);
                for my $dj ( -$r .. $r ) {
                    next if $di == 0 && $dj == 0;
                    my ( $ni, $nj ) = ( $i + $di, $j + $dj );
                    my $nd   = abs($di) + abs($dj);
                    my $nkey = "$ni,$nj";
                    next unless exists $dist->{$nkey};
                    if ( $dist->{$nkey} - $nd - $d >= $threshold ) {
                        $count++;
                    }
                }
            }
        }
        return $count;
    }

    sub bfs( $racetrack, $start ) {
        my %dist = ();
        my @q    = ( [ $start, 0 ] );
        while (@q) {
            my ( $p, $v ) = @{ shift @q };
            my ( $i, $j ) = $p->@*;
            $dist{"$i,$j"} = $v;
            for my $d ( [ -1, 0 ], [ 0, 1 ], [ 1, 0 ], [ 0, -1 ] ) {
                my ( $ni, $nj ) = ( $i + $d->[0], $j + $d->[1] );
                my $key = "$ni,$nj";
                if ( exists $racetrack->{$key} && !exists $dist{$key} ) {
                    push @q, [ [ $ni, $nj ], $v + 1 ];
                }
            }
        }
        return \%dist;
    }
}
