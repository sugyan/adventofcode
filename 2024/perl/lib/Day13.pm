use 5.38.0;
use feature 'class';
no warnings 'experimental::class';

use Base;

class Machine {
    field $a_x     : param;
    field $a_y     : param;
    field $b_x     : param;
    field $b_y     : param;
    field $prize_x : param;
    field $prize_y : param;

    method tokens($offset) {
        my $d = det( $a_x, $b_x, $a_y, $b_y );
        return 0 if $d == 0;

        my $d_a = det( $prize_x + $offset, $b_x, $prize_y + $offset, $b_y );
        my $d_b = det( $a_x, $prize_x + $offset, $a_y, $prize_y + $offset );
        return 0 if $d_a % $d != 0 || $d_b % $d != 0;

        my ( $a, $b ) = ( $d_a / $d, $d_b / $d );
        return $a * 3 + $b;
    }

    sub det( $a, $b, $c, $d ) {
        $a * $d - $b * $c;
    }
}

class Solution : isa(Base) {
    use List::Util qw(sum);

    method parse($lines) {
        return [
            map {
                my ( $a_x, $a_y ) = /Button A: X\+(\d+), Y\+(\d+)/;
                my ( $b_x, $b_y ) = /Button B: X\+(\d+), Y\+(\d+)/;
                my ( $p_x, $p_y ) = /Prize: X=(\d+), Y=(\d+)/;
                Machine->new(
                    a_x     => $a_x,
                    a_y     => $a_y,
                    b_x     => $b_x,
                    b_y     => $b_y,
                    prize_x => $p_x,
                    prize_y => $p_y,
                );
            } split /\n\n/,
            join( "\n", $lines->@* )
        ];
    }

    method part1() {
        my $input = $self->input();
        return sum map { $_->tokens(0) } $input->@*;
    }

    method part2() {
        my $input = $self->input();
        return sum map { $_->tokens(10_000_000_000_000) } $input->@*;
    }
}
