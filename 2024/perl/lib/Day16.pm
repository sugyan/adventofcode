use 5.38.0;
use feature 'class';
no warnings 'experimental::class';

use Base;

class Solution : isa(Base) {
    use List::Util qw(min pairs);

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
        my $mins  = dijkstra_with_paths($input);
        return find_min_score( $mins, $input->{end}->@* );
    }

    method part2() {
        my $input = $self->input();
        my $mins  = dijkstra_with_paths($input);
        my $best  = find_min_score( $mins, $input->{end}->@* );
        my %seen  = ();
        my @q     = map { $_->key } grep {
                 $_->key =~ /^$input->{end}[0],$input->{end}[1],/
              && $_->value->[0] == $best
        } pairs $mins->%*;
        while ( my $item = shift @q ) {
            $seen{ $item =~ s/,(\d+)$//r } = 1;
            if ( exists $mins->{$item} ) {
                push @q, $mins->{$item}[1]->@*;
            }
        }
        return scalar keys %seen;
    }

    sub dijkstra_with_paths($input) {
        my @q    = ( [ 0, $input->{start}, 1 ] );
        my %mins = ();
        while ( my $item = pop @q ) {
            my ( $cost, $pos, $dir ) = $item->@*;
            my ( $i, $j ) = $pos->@*;
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
                if ( exists $mins{$nkey} ) {
                    my $min = $mins{$nkey};
                    if ( $min->[0] < $ncost ) {
                        next;
                    }
                    elsif ( $min->[0] == $ncost ) {
                        push $min->[1]->@*, "$i,$j,$dir";
                        next;
                    }
                }
                $mins{$nkey} = [ $ncost, ["$i,$j,$dir"] ];

                push @q, [ $ncost, [ $ni, $nj ], $ndir ];
                @q = sort { $b->[0] <=> $a->[0] } @q;
            }
        }
        return \%mins;
    }

    sub find_min_score( $mins, $i, $j ) {
        return min map { $_->value->[0] }
          grep { $_->key =~ /^$i,$j,/ } pairs $mins->%*;
    }
}
