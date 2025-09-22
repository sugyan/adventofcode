use 5.38.0;
use feature 'class';
no warnings 'experimental::class';

use Base;

class Solution : isa(Base) {
    use List::Util qw(all zip);

    method parse($lines) {
        my ( $first, $second ) = split /\n\n/, join( "\n", $lines->@* ), 2;
        return {
            rules   => { map { $_, 1 } split /\n/, $first },
            updates => [ map { [ split /,/ ] } split /\n/, $second ]
        };
    }

    method part1() {
        my $input = $self->input();
        return analyze( $input->{rules}, $input->{updates} )->{part1};
    }

    method part2() {
        my $input = $self->input();
        return analyze( $input->{rules}, $input->{updates} )->{part2};
    }

    sub analyze( $rules, $updates ) {
        my $sum = {
            part1 => 0,
            part2 => 0,
        };
        for my $update ( $updates->@* ) {
            my @sorted = sort { $rules->{"$a|$b"} ? -1 : 1 } $update->@*;
            my $middle = $sorted[ int( @sorted / 2 ) ];
            if ( all { $_->[0] == $_->[1] } zip $update, \@sorted ) {
                $sum->{part1} += $middle;
            }
            else {
                $sum->{part2} += $middle;

            }
        }
        return $sum;
    }
}
