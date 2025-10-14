use 5.38.0;
use feature 'class';
no warnings 'experimental::class';

use Base;

class Solution : isa(Base) {
    use List::Util qw(pairs);

    my $STEP = 1;
    my $TURN = 1000;
    my @DIR  = ( [ -1, 0 ], [ 0, 1 ], [ 1, 0 ], [ 0, -1 ] );

    method parse($lines) {
        my ( $start, $end ) = ( [ 0, 0 ], [ 0, 0 ] );
        my %maze = ();
        for my $i ( 0 .. $lines->$#* ) {
            my @s = split //, $lines->[$i];
            for my $j ( 0 .. $#s ) {
                if ( $s[$j] eq 'S' ) {
                    $start = [ $i, $j ];
                }
                if ( $s[$j] eq 'E' ) {
                    $end = [ $i, $j ];
                }
                if ( $s[$j] ne '#' ) {
                    $maze{"$i,$j"} = 1;
                }
            }
        }
        return {
            maze  => \%maze,
            start => $start,
            end   => $end,
        };
    }

    method part1() {
        my $input = $self->input();
        return solve($input)->{part1};
    }

    method part2() {
        my $input = $self->input();
        return solve($input)->{part2};
    }

    sub solve($input) {
        my $start = $input->{start};
        my $end   = $input->{end};
        my @q     = ( [ 0, $start, 1, [ join ",", $start->@* ] ] );
        my $best  = 'Inf';
        my %mins  = ();
        my %path  = ();
        while ( my $item = pop @q ) {
            my ( $cost, $pos, $dir, $path ) = $item->@*;
            last if $cost > $best;
            my ( $i, $j ) = $pos->@*;
            if ( $i == $end->[0] && $j == $end->[1] ) {
                $best = $cost;
                %path = ( %path, map { $_ => 1 } $path->@* );
                next;
            }

            my @next_dirs = (
                ( $dir, $cost + $STEP ),
                ( ( $dir + 1 ) % 4, $cost + $TURN + $STEP ),
                ( ( $dir + 3 ) % 4, $cost + $TURN + $STEP ),
            );
            for my $pair ( pairs @next_dirs ) {
                my ( $ndir, $ncost ) = $pair->@*;
                my ( $ni, $nj ) = ( $i + $DIR[$ndir][0], $j + $DIR[$ndir][1] );
                next unless exists $input->{maze}{"$ni,$nj"};

                my $nkey = "$ni,$nj,$ndir";
                next if exists $mins{$nkey} && $mins{$nkey} < $ncost;
                $mins{$nkey} = $ncost;

                push_sorted( \@q,
                    [ $ncost, [ $ni, $nj ], $ndir, [ $path->@*, "$ni,$nj" ] ] );
            }
        }
        return {
            part1 => $best,
            part2 => scalar keys %path,
        };
    }

    sub push_sorted( $q, $item ) {
        my ( $lo, $hi ) = ( 0, $q->$#* );
        while ( $lo <= $hi ) {
            my $mid = ( $lo + $hi ) >> 1;
            if ( $q->[$mid][0] > $item->[0] ) {
                $lo = $mid + 1;
            }
            else {
                $hi = $mid - 1;
            }
        }
        splice $q->@*, $lo, 0, $item;
    }
}
