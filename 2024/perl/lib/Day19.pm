use 5.38.0;
use feature 'class';
no warnings 'experimental::class';

use Base;

class Solution : isa(Base) {
    use List::Util qw(sum);

    method parse($lines) {
        my @patterns = split /,\s*/, $lines->[0];
        my @designs  = $lines->@[ 2 .. $lines->$#* ];
        return {
            patterns => \@patterns,
            designs  => \@designs,
        };
    }

    method part1() {
        my $input = $self->input();
        return scalar grep { $_ > 0 }
          map { count_paths( $_, $input->{patterns} ) }
          ( $input->{designs}->@* );
    }

    method part2() {
        my $input = $self->input();
        return sum map { count_paths( $_, $input->{patterns} ) }
          ( $input->{designs}->@* );
    }

    sub count_paths( $design, $patterns ) {
        my @dp = (0) x ( length($design) + 1 );
        $dp[0] = 1;
        for my $i ( 0 .. length $design ) {
            for my $p ( $patterns->@* ) {
                my $l = length $p;
                if ( substr( $design, $i, $l ) eq $p ) {
                    $dp[ $i + $l ] += $dp[$i];
                }
            }
        }
        return $dp[ length $design ];
    }
}
