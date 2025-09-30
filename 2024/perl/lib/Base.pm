use 5.38.0;
use feature 'class';
no warnings 'experimental::class';

class Base {
    field $fh : param;
    field $lines = [];
    field $input;

    ADJUST {
        while ( defined( my $line = <$fh> ) ) {
            chomp $line;
            push $lines->@*, $line;
        }
    }

    method input() {
        return $input //= $self->parse($lines);
    }

    method parse($lines) {
        return $lines;
    }

    method part1() { die "Not implemented" }
    method part2() { die "Not implemented" }
}
