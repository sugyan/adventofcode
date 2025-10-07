use 5.38.0;
use feature 'class';
no warnings 'experimental::class';

use Base;

class XY {
    use constant SPACE => $ENV{EXAMPLE_MODE}
      ? { x => 11,  y => 7 }
      : { x => 101, y => 103 };

    field $x : param;
    field $y : param;

    ADJUST {
        $x %= SPACE->{x};
        $y %= SPACE->{y};
    }

    method quadrant() {
        return {
            x => int( SPACE->{x} / 2 ) <=> $x,
            y => int( SPACE->{y} / 2 ) <=> $y
        };
    }

    method key() {
        return "$x,$y";
    }
}

class Robot {
    field $px : param;
    field $py : param;
    field $vx : param;
    field $vy : param;

    method position($t) {
        return XY->new( x => $px + $vx * $t, y => $py + $vy * $t );
    }
}

class Solution : isa(Base) {
    use List::Util qw(reduce);

    method parse($lines) {
        return [
            map {
                my ( $px, $py, $vx, $vy ) =
                  /p=(-?\d+),(-?\d+) v=(-?\d+),(-?\d+)/;
                Robot->new( px => $px, py => $py, vx => $vx, vy => $vy )
            } $lines->@*
        ];
    }

    method part1() {
        my $input = $self->input();
        my %count = ();
        for my $r ( $input->@* ) {
            my $quad = $r->position(100)->quadrant();
            if ( $quad->{x} && $quad->{y} ) {
                $count{ join ",", $quad->@{qw(x y)} }++;
            }
        }
        return reduce { $a * $b } values %count;
    }

    method part2() {
        my $input = $self->input();
        for ( my $i = 1 ; ; $i++ ) {
            my %set = map { $_->position($i)->key() => 1 } $input->@*;
            return $i if scalar keys %set == scalar $input->@*;
        }
    }
}
