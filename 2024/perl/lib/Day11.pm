use 5.38.0;
use feature 'class';
no warnings 'experimental::class';

use Base;

class Solution : isa(Base) {
    use List::Util qw(pairs pairmap sum);

    method parse($lines) {
        return [ split /\s+/, $lines->[0] ];
    }

    method part1() {
        my $input = $self->input();
        return count_stones( $input, 25 );
    }

    method part2() {
        my $input = $self->input();
        return count_stones( $input, 75 );
    }

    sub count_stones( $input, $n ) {
        my $counts = merge_counts( map { $_ => 1 } $input->@* );
        for ( 1 .. $n ) {
            $counts = merge_counts( pairmap { blink( $a, $b ) } $counts->%* );
        }
        return sum values $counts->%*;
    }

    sub merge_counts(@kvlist) {
        my %merged;
        for my $pair ( pairs @kvlist ) {
            $merged{ $pair->key + 0 } += $pair->value;
        }
        return \%merged;
    }

    sub blink( $n, $count ) {
        my $l = length($n);
        return
          map { $_ => $count }
          $l % 2 ? ( $n * 2024 || 1 ) : $n =~ /(.{@{[$l>>1]}})(.+)/;
    }
}
