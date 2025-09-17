use 5.38.0;
use feature 'class';
no warnings 'experimental::class';

use Base;

class Solution : isa(Base) {
    use List::Util qw(sum zip);

    method parse($lines) {
        my ( @l, @r );
        for my $line ( $lines->@* ) {
            my ( $l, $r ) = split /\s+/, $line, 2;
            push @l, $l;
            push @r, $r;
        }
        return { l => \@l, r => \@r };
    }

    method part1() {
        my $input = $self->input();
        my $l     = [ sort { $a <=> $b } $input->{l}->@* ];
        my $r     = [ sort { $a <=> $b } $input->{r}->@* ];
        return sum( map { abs( $_->[0] - $_->[1] ) } zip( $l, $r ) );
    }

    method part2() {
        my $input = $self->input();
        my %counts;
        $counts{$_}++ for $input->{r}->@*;
        return sum( map { $_ * ( $counts{$_} // 0 ) } $input->{l}->@* );
    }
}
