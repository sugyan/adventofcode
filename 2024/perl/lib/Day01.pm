use 5.38.0;
use feature 'class';
no warnings 'experimental::class';

class Solution {
    use List::Util qw(sum zip);

    field $fh : param;
    field $input;

    ADJUST {
        $input = {
            l => [],
            r => [],
        };
        while ( defined( my $line = <$fh> ) ) {
            chomp $line;
            my ( $l, $r ) = split /\s+/, $line, 2;
            push $input->{l}->@*, $l;
            push $input->{r}->@*, $r;
        }
    }

    method part1() {
        my $l = [ sort { $a <=> $b } $input->{l}->@* ];
        my $r = [ sort { $a <=> $b } $input->{r}->@* ];
        return sum( map { abs( $_->[0] - $_->[1] ) } zip( $l, $r ) );
    }

    method part2() {
        my %counts;
        $counts{$_}++ for $input->{r}->@*;
        return sum( map { $_ * ( $counts{$_} // 0 ) } $input->{l}->@* );
    }
}
