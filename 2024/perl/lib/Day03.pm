use 5.38.0;
use feature 'class';
no warnings 'experimental::class';

use Base;

class Solution : isa(Base) {
    use List::Util qw(pairmap sum);

    method parse($lines) {
        return join "", $lines->@*;
    }

    method part1() {
        my $input = $self->input();
        return sum pairmap { $a * $b } $input =~ /mul\((\d+),(\d+)\)/g;
    }

    method part2() {
        my $input = $self->input();
        $input =~ s/don't\(\).*?(do\(\)|\z)//g;
        return sum pairmap { $a * $b } $input =~ /mul\((\d+),(\d+)\)/g;
    }
}
